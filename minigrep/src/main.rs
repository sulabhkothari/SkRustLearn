use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);

        // The process::exit function will stop the program immediately and return the number that
        // was passed as the exit status code. This is similar to the panic!-based handling we used
        // in Listing 12-8, but we no longer get all the extra output
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // We use if let rather than unwrap_or_else to check whether run returns an Err value and call
    // process::exit(1) if it does. The run function doesn’t return a value that we want to unwrap
    // in the same way that Config::new returns the Config instance. Because run returns () in the
    // success case, we only care about detecting an error, so we don’t need unwrap_or_else to return
    // the unwrapped value because it would only be ().
    // The bodies of the if let and the unwrap_or_else functions are the same in both cases: we print
    // the error and exit.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

// If your program needs to accept arguments containing invalid Unicode, use std::env::args_os instead.
// That function returns an iterator that produces OsString values instead of String values.
// cargo run needle haystack

// Separation of Concerns for Binary Projects
//
//The organizational problem of allocating responsibility for multiple tasks to the main function is
// common to many binary projects. As a result, the Rust community has developed a process to use as
// a guideline for splitting the separate concerns of a binary program when main starts getting large.
// The process has the following steps:
//    1. Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
//    2. As long as your command line parsing logic is small, it can remain in main.rs.
//    3. When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.
//
//The responsibilities that remain in the main function after this process should be limited to the following:
//    1. Calling the command line parsing logic with the argument values
//    2. Setting up any other configuration
//    3. Calling a run function in lib.rs
//    4. Handling the error if run returns an error
//
//This pattern is about separating concerns: main.rs handles running the program, and lib.rs handles
// all the logic of the task at hand. Because you can’t test the main function directly, this structure
// lets you test all of your program’s logic by moving it into functions in lib.rs. The only code that
// remains in main.rs will be small enough to verify its correctness by reading it.

// Writing Error Messages to Standard Error Instead of Standard Output:
// Most terminals provide two kinds of output: standard output (stdout) for general information and
// standard error (stderr) for error messages. This distinction enables users to choose to direct the
// successful output of a program to a file but still print error messages to the screen.
// `cargo run > output.txt` due to `eprintln` sends output to standard error stream (stderr) instead
// of standard output (stdout). This means in case of error nothing will be written to output.txt, and
// error would appear on the terminal due to stderr.
// `cargo run to poem.txt true > output.txt` redirects all output to file because it uses println which
// redirects to stdout, which in case of `>`, sends the output to `output.txt`.