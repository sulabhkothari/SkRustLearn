use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub fn webserver_main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

// In the handle_connection function, we’ve made the stream parameter mutable. The reason is that
// the TcpStream instance keeps track of what data it returns to us internally. It might read more
// data than we asked for and save that data for the next time we ask for data. It therefore needs
// to be mut because its internal state might change; usually, we think of “reading” as not needing
// mutation, but in this case we need the mut keyword.
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    // Because we’re reading raw bytes into the buffer, we transform get into a byte string by adding
    // the b"" byte string syntax at the start of the content data.
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // The String::from_utf8_lossy function takes a &[u8] and produces a String from it. The “lossy”
    // part of the name indicates the behavior of this function when it sees an invalid UTF-8 sequence:
    // it will replace the invalid sequence with �, the U+FFFD REPLACEMENT CHARACTER. You might see
    // replacement characters for characters in the buffer that aren’t filled by request data.
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

// Uniform Resource Identifier (URI)
// TcpStream contains an internal buffer to minimize calls to the underlying operating system.

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        // We can't pass same receiver to multiple threads because the channel implementation that
        // Rust provides is multiple producer, single consumer. This means we can’t just clone the
        // consuming end of the channel. Additionally, taking a job off the channel queue involves
        // mutating the receiver, so the threads need a safe way to share and modify receiver;
        // otherwise, we might get race conditions.

        // Share ownership across multiple threads and allow the threads to mutate the value, we
        // need to use Arc<Mutex<T>>. The Arc type will let multiple workers own the receiver, and
        // Mutex will ensure that only one worker gets a job from the receiver at a time.
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        // In ThreadPool::new, we put the receiving end of the channel in an Arc and a Mutex. For
        // each new worker, we clone the Arc to bump the reference count so the workers can share
        // ownership of the receiving end.

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        // After creating a new Job instance using the closure we get in execute, we send that job
        // down the sending end of the channel. We’re calling unwrap on send for the case that sending
        // fails. This might happen if, for example, we stop all our threads from executing, meaning
        // the receiving end has stopped receiving new messages. At the moment, we can’t stop our
        // threads from executing: our threads continue executing as long as the pool exists. The
        // reason we use unwrap is that we know the failure case won’t happen, but the compiler doesn’t
        // know that.
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        // If we tried to send a message and join immediately in the same loop, we couldn’t guarantee
        // that the worker in the current iteration would be the one to get the message from the channel.
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // The error tells us we can’t call join because we only have a mutable borrow of each worker
        // and join takes ownership of its argument. To solve this issue, we need to move the thread
        // out of the Worker instance that owns thread so join can consume the thread.
        // If Worker holds an Option<thread::JoinHandle<()>> instead, we can call the take method on
        // the Option to move the value out of the Some variant and leave a None variant in its place.
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            // This code compiles and runs but doesn’t result in the desired threading behavior: a
            // slow request will still cause other requests to wait to be processed. The reason is
            // somewhat subtle: the Mutex struct has no public unlock method because the ownership
            // of the lock is based on the lifetime of the MutexGuard<T> within the LockResult<MutexGuard<T>>
            // that the lock method returns. At compile time, the borrow checker can then enforce the
            // rule that a resource guarded by a Mutex cannot be accessed unless we hold the lock.
            // But this implementation can also result in the lock being held longer than intended
            // if we don’t think carefully about the lifetime of the MutexGuard<T>. Because the values
            // in the while expression remain in scope for the duration of the block, the lock remains
            // held for the duration of the call to job(), meaning other workers cannot receive jobs.
            //      By using loop instead and acquiring the lock and a job within the block rather
            // than outside it, the MutexGuard returned from the lock method is dropped as soon as the
            // let job statement ends. This ensures that the lock is held during the call to recv,
            // but it is released before the call to job(), allowing multiple requests to be serviced
            // concurrently.
            // while let Ok(job) = receiver.lock().unwrap().recv()

            loop {
                // Here, we first call lock on the receiver to acquire the mutex, and then we call
                // unwrap to panic on any errors. Acquiring a lock might fail if the mutex is in a
                // poisoned state, which can happen if some other thread panicked while holding the
                // lock rather than releasing the lock. In this situation, calling unwrap to have
                // this thread panic is the correct action to take. Feel free to change this unwrap
                // to an expect with an error message that is meaningful to you.
                // If we get the lock on the mutex, we call recv to receive a Job from the channel.
                // A final unwrap moves past any errors here as well, which might occur if the thread
                // holding the sending side of the channel has shut down, similar to how the send
                // method returns Err if the receiving side shuts down.
                // The call to recv blocks, so if there is no job yet, the current thread will wait
                // until a job becomes available. The Mutex<T> ensures that only one Worker thread
                // at a time is trying to request a job.
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// pub fn spawn<F, T>(f: F) -> JoinHandle<T>
//     where
//         F: FnOnce() -> T + Send + 'static,
//         T: Send + 'static
// read like (FnOnce() -> T ) + Send + 'static
// The F type parameter is the one we’re concerned with here; the T type parameter is related to the
// return value, and we’re not concerned with that. We can see that spawn uses FnOnce as the trait
// bound on F. This is probably what we want as well, because we’ll eventually pass the argument we
// get in execute to spawn. We can be further confident that FnOnce is the trait we want to use
// because the thread for running a request will only execute that request’s closure one time, which
// matches the Once in FnOnce.
// The F type parameter also has the trait bound Send and the lifetime bound 'static, which are useful
// in our situation: we need Send to transfer the closure from one thread to another and 'static
// because we don’t know how long the thread will take to execute.
