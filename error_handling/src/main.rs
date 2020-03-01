// The main function is special, and there are restrictions on what its return type must be. One
// valid return type for main is (), and conveniently, another valid return type is Result<T, E>
// The Box<dyn Error> type is called a trait object, which we’ll talk about in the “Using Trait
// Objects that Allow for Values of Different Types” section in Chapter 17. For now, you can read
// Box<dyn Error> to mean “any kind of error.” Using ? in a main function with this return type is
// allowed.
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    read_username_from_file_with_error_propagation_operator2().expect("Failure from called method!!!");
    Ok(())
}

// RUST_BACKTRACE=1 cargo run
// In order to get backtraces with this information, debug symbols must be enabled.
// Debug symbols are enabled by default when using cargo build or cargo run without the --release
// flag, as we have here.
fn using_a_panic_backtrace() {
    // panicked at 'index out of bounds: the len is 3 but the index is 99',
    // /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806:10

    // This error points at a file we didn’t write, libcore/slice/mod.rs. That’s the implementation
    // of slice in the Rust source code. The code that gets run when we use [] on our vector v is in
    // libcore/slice/mod.rs, and that is where the panic! is actually happening.

    // The next note line tells us that we can set the RUST_BACKTRACE environment variable to get a
    // backtrace of exactly what happened to cause the error. A backtrace is a list of all the
    // functions that have been called to get to this point. Backtraces in Rust work as they do in
    // other languages: the key to reading the backtrace is to start from the top and read until you
    // see files you wrote. That’s the spot where the problem originated. The lines above the lines
    // mentioning your files are code that your code called; the lines below are code that called
    // your code. These lines might include core Rust code, standard library code, or crates that
    // you’re using.
    let v = vec![1, 2, 3];

    v[99];
}

fn imp_concepts() {
// When the panic! macro executes, your program will print a failure message, unwind and clean up
// the stack, and then quit.
// Unwinding the Stack or Aborting in Response to a Panic
//By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the
// stack and cleans up the data from each function it encounters. But this walking back and cleanup
// is a lot of work. The alternative is to immediately abort, which ends the program without
// cleaning up. Memory that the program was using will then need to be cleaned up by the operating
// system. If in your project you need to make the resulting binary as small as possible, you can
// switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate
// [profile] sections in your Cargo.toml file. For example, if you want to abort on panic in release
// mode, add this:
//[profile.release]
//panic = 'abort'

// Other languages, like C, will attempt to give you exactly what you asked for in this situation,
// even though it isn’t what you want: you’ll get whatever is at the location in memory that would
// correspond to that element in the vector, even though the memory doesn’t belong to the vector.
// This is called a buffer overread and can lead to security vulnerabilities if an attacker is able
// to manipulate the index in such a way as to read data they shouldn’t be allowed to that is stored
// after the array.
}

// enum Result<T, E> {
//    Ok(T),
//    Err(E),
//}

use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

fn recoverable_errors_with_result() {

    // Because this error message starts with the text we specified, Failed to open hello.txt, it
    // will be easier to find where in the code this error message is coming from. If we use unwrap
    // in multiple places, it can take more time to figure out exactly which unwrap is causing the
    // panic because all unwrap calls that panic print the same message.
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    let f = File::open("hello.txt");

    // The type of the value that File::open returns inside the Err variant is io::Error, which is a
    // struct provided by the standard library. This struct has a method kind that we can call to
    // get an io::ErrorKind value. The enum io::ErrorKind is provided by the standard library and
    // has variants representing the different kinds of errors that might result from an io operation.
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // With Closures
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let f = File::open("hello.txt").unwrap();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


fn read_username_from_file_with_error_propagation_operator() -> Result<String, io::Error> {
    // There is a difference between what the match expression from Listing 9-6 and the ? operator
    // do: error values that have the ? operator called on them go through the from function,
    // defined in the From trait in the standard library, which is used to convert errors from one
    // type into another. When the ? operator calls the from function, the error type received is
    // converted into the error type defined in the return type of the current function. This is
    // useful when a function returns one error type to represent all the ways a function might fail,
    // even if parts might fail for many different reasons. As long as each error type implements
    // the from function to define how to convert itself to the returned error type, the ? operator
    // takes care of the conversion automatically.
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

    // ? on Err will shortcircuit and return Err directly
}

//Even Shorter
fn read_username_from_file_with_error_propagation_operator2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

use std::fs;
// Reading a file into a string is a fairly common operation, so Rust provides the convenient
// fs::read_to_string function that opens the file, creates a new String, reads the contents of the
// file, puts the contents into that String, and returns it.
fn read_username_from_file_using_direct_read() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// We’re only allowed to use the ? operator in a function that returns Result or Option or another
// type that implements std::ops::Try.


// Guidelines for Error Handling
//It’s advisable to have your code panic when it’s possible that your code could end up in a bad
// state. In this context, a bad state is when some assumption, guarantee, contract, or invariant
// has been broken, such as when invalid values, contradictory values, or missing values are passed
// to your code—plus one or more of the following:
//  1. The bad state is not something that’s expected to happen occasionally.
//  2. Your code after this point needs to rely on not being in this bad state.
//  3. There’s not a good way to encode this information in the types you use.
// Similarly, panic! is often appropriate if you’re calling external code that is out of your control
// and it returns an invalid state that you have no way of fixing.

// Functions often have contracts: their behavior is only guaranteed if the inputs meet particular
// requirements. Panicking when the contract is violated makes sense because a contract violation
// always indicates a caller-side bug and it’s not a kind of error you want the calling code to have
// to explicitly handle. In fact, there’s no reasonable way for calling code to recover; the calling
// programmers need to fix the code. Contracts for a function, especially when a violation will
// cause a panic, should be explained in the API documentation for the function.