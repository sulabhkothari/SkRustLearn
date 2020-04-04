use std::borrow::BorrowMut;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn concurrent_main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // We can use the move keyword before the parameter list of a closure to force the closure to take
    // ownership of the values it uses in the environment. This technique is especially useful when
    // creating new threads in order to transfer ownership of values from one thread to another.

    // Rust infers how to capture v, and because println! only needs a reference to v, the closure tries
    // to borrow v. However, there’s a problem: Rust can’t tell how long the spawned thread will run, so
    // it doesn’t know if the reference to v will always be valid.

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // If we were allowed to run this code, there’s a possibility the spawned thread would be
    // immediately put in the background without running at all. The spawned thread has a reference
    // to v inside, but the main thread immediately drops v, using the drop function
    //drop(v); // oh no!

    handle.join().unwrap();

    // The move keyword overrides Rust’s conservative default of borrowing; it doesn’t let us violate
    // the ownership rules.

    // mpsc stands for multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        println!("Sent: {}", val);
        tx.send(val).unwrap();
    });

    // The receiving end of a channel has two useful methods: recv and try_recv. We’re using recv,
    // short for receive, which will block the main thread’s execution and wait until a value is sent
    // down the channel. Once a value is sent, recv will return it in a Result<T, E>. When the sending
    // end of the channel closes, recv will return an error to signal that no more values will be coming.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // The try_recv method doesn’t block, but will instead return a Result<T, E> immediately: an Ok
    // value holding a message if one is available and an Err value if there aren’t any messages this
    // time. Using try_recv is useful if this thread has other work to do while waiting for messages:
    // we could write a loop that calls try_recv every so often, handles a message if one is available,
    // and otherwise does other work for a little while until checking again.
    //let mut flag = true;
    //while flag {
    //    match rx.try_recv() {
    //        Ok(v) => {
    //            println!("Got: {}", v);
    //            flag = false;
    //        },
    //        Err(_) => continue,
    //    }
    //}
    //send_multiple_msg_over_channel();

    shared_state_concurrency();
}

fn send_multiple_msg_over_channel() {
    println!("#############################################");
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // We’re not calling the recv function explicitly anymore: instead, we’re treating rx as an
    // iterator. For each value received, we’re printing it. When the channel is closed, iteration will end.
    // Because we don’t have any code that pauses or delays in the for loop in the main thread, we
    // can tell that the main thread is waiting to receive values from the spawned thread.
    for received in rx {
        println!("Got: {}", received);
    }
}

// Mutex is an abbreviation for mutual exclusion
fn shared_state_concurrency() {
    println!("#############################################");

    use std::sync::{Arc, Mutex};
    let m = Mutex::new(5);

    {
        // The type system ensures that we acquire a lock before using the value in m: Mutex<i32> is
        // not an i32, so we must acquire the lock to be able to use the i32 value. We can’t forget;
        // the type system won’t let us access the inner i32 otherwise.
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    // As you might suspect, Mutex<T> is a smart pointer. More accurately, the call to lock returns
    // a smart pointer called MutexGuard, wrapped in a LockResult that we handled with the call to
    // unwrap. The MutexGuard smart pointer implements Deref to point at our inner data; the smart
    // pointer also has a Drop implementation that releases the lock automatically when a MutexGuard
    // goes out of scope, which happens at the end of the inner scope. As a result, we don’t risk
    // forgetting to release the lock and blocking the mutex from being used by other threads because
    // the lock release happens automatically.

    println!("m = {:?}", m);

    // Compilation Error with Rc: `Rc<Mutex<i32>>` cannot be sent between threads safely. The compiler
    // is also telling us the reason why: the trait `Send` is not implemented for `Rc<Mutex<i32>>`.
    // Send trait ensures the types we use with threads are meant for use in concurrent situations.
    // This could lead to wrong counts—subtle bugs that could in turn lead to memory leaks or a value
    // being dropped before we’re done with it. What we need is a type exactly like Rc<T> but one that
    // makes changes to the reference count in a thread-safe way.
    // Fortunately, Arc<T> is a type like Rc<T> that is safe to use in concurrent situations. The a
    // stands for atomic, meaning it’s an atomically reference counted type.
    // Mutex<T> provides interior mutability, as the Cell family does.
    // Mutex<T> comes with the risk of creating deadlocks. These occur when an operation needs to
    // lock two resources and two threads have each acquired one of the locks, causing them to wait
    // for each other forever.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// The std::marker traits Sync and Send.

// Allowing Transference of Ownership Between Threads with Send:
// The Send marker trait indicates that ownership of the type implementing Send can be transferred
// between threads. Almost every Rust type is Send, but there are some exceptions, including Rc<T>
// so that you don’t have to pay the thread-safe performance penalty. Therefore, Rust’s type system
// and trait bounds ensure that you can never accidentally send an Rc<T> value across threads unsafely.
// Any type composed entirely of Send types is automatically marked as Send as well. Almost all
// primitive types are Send, aside from raw pointers.

// Allowing Access from Multiple Threads with Sync:
// The Sync marker trait indicates that it is safe for the type implementing Sync to be referenced
// from multiple threads. In other words, any type T is Sync if &T (a reference to T) is Send,
// meaning the reference can be sent safely to another thread. Similar to Send, primitive types are
// Sync, and types composed entirely of types that are Sync are also Sync.
// The smart pointer Rc<T> is also not Sync for the same reasons that it’s not Send. The RefCell<T>
// type (which we talked about in Chapter 15) and the family of related Cell<T> types are not Sync.
// The implementation of borrow checking that RefCell<T> does at runtime is not thread-safe.

// Implementing Send and Sync Manually Is Unsafe:
// Because types that are made up of Send and Sync traits are automatically also Send and Sync, we
// don’t have to implement those traits manually. As marker traits, they don’t even have any methods
// to implement. They’re just useful for enforcing invariants related to concurrency.

// Building new concurrent types not made up of Send and Sync parts requires careful thought to uphold
// the safety guarantees. The Rust standard library provides channels for message passing and smart
// pointer types, such as Mutex<T> and Arc<T>, that are safe to use in concurrent contexts.
