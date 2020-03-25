use adder;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

// We can still run a particular integration test function by specifying the test function’s name as
// an argument to cargo test. To run all the tests in a particular integration test file, use the
// --test argument of cargo test followed by the name of the file
// cargo test --test integration_test

// Each file in the tests directory is compiled as its own separate crate.
// Treating each integration test file as its own crate is useful to create separate scopes that are
// more like the way end users will be using your crate. However, this means files in the tests
// directory don’t share the same behavior as files in src do, as you learned in Chapter 7 regarding
// how to separate code into modules and files.

// If our project is a binary crate that only contains a src/bin.rs file and doesn’t have a src/lib.rs
// file, we can’t create integration tests in the tests directory and bring functions defined in the
// src/bin.rs file into scope with a use statement. Only library crates expose functions that other
// crates can use; binary crates are meant to be run on their own.

// This is one of the reasons Rust projects that provide a binary have a straightforward src/bin.rs
// file that calls logic that lives in the src/lib.rs file. Using that structure, integration tests
// can test the library crate with use to make the important functionality available. If the important
// functionality works, the small amount of code in the src/bin.rs file will work as well, and that
// small amount of code doesn’t need to be tested.