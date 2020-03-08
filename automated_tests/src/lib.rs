mod mymod;

// The attribute cfg stands for configuration and tells Rust that the following item should only be
// included given a certain configuration option. In this case, the configuration option is test,
// which is provided by Rust for compiling and running tests. By using the cfg attribute, Cargo
// compiles our test code only if we actively run the tests with cargo test. This includes any helper
// functions that might be within this module, in addition to the functions annotated with #[test].
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        // This test will pass because the value we put in the should_panic attribute’s expected
        // parameter is a substring of the message that the Guess::new function panics with. We could
        // have specified the entire panic message that we expect, which in this case would be Guess
        // value must be less than or equal to 100, got 200. What you choose to specify in the expected
        // parameter for should_panic depends on how much of the panic message is unique or dynamic
        // and how precise you want your test to be. In this case, a substring of the panic message
        // is enough to ensure that the code in the test function executes the else if value > 100 case.
        Guess::new(200);
    }

    // So far, we’ve written tests that panic when they fail. We can also write tests that use Result<T, E>!
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    // The it_works function now has a return type, Result<(), String>. In the body of the function,
    // rather than calling the assert_eq! macro, we return Ok(()) when the test passes and an Err
    // with a String inside when the test fails.
    // Writing tests so they return a Result<T, E> enables you to use the question mark operator in
    // the body of tests, which can be a convenient way to write tests that should fail if any
    // operation within them returns an Err variant.

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }


    #[test]
    fn demotest() {
        mymod::demo();
    }
}


mod demo {
    use super::*;

    fn demo() -> () {
        add_two(90);
    }
}

// None of the below types are marked public (no `pub`), still they are accessible from tests

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        println!("NEW GUESS!!");
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}


// When the assertions fail, these macros print their arguments using debug formatting, which means
// the values being compared must implement the PartialEq and Debug traits. All the primitive types
// and most of the standard library types implement these traits. For structs and enums that you
// define, you’ll need to implement PartialEq to assert that values of those types are equal or not
// equal. You’ll need to implement Debug to print the values when the assertion fails.

// cargo test -- --test-threads=1

// The output from the test that failed, appears in the section of the test summary output. If we
// want to see printed values for passing tests as well, we can disable the output capture behavior
// by using the --nocapture flag and --test-threads=1 so that the tests output is not interleaved:
// cargo test -- --nocapture --test-threads=1

// Running Single Tests: cargo test greater_than_100 -- --nocapture
// "5 filtered out"

// Note that the module in which a test appears becomes part of the test’s name, so we can run all
// the tests in a module by filtering on the module’s name.
// Filtering to Run Multiple Tests: cargo test hold
// filter by test module name: cargo test tests

// Ignoring Some Tests Unless Specifically Requested: Use `#[ignore]` annotation
// If we want to run only the ignored tests, we can use: cargo test -- --ignored

// The Rust community thinks about tests in terms of two main categories: unit tests and integration
// tests. Unit tests are small and more focused, testing one module in isolation at a time, and can
// test private interfaces. Integration tests are entirely external to your library and use your code
// in the same way any other external code would, using only the public interface and potentially
// exercising multiple modules per test.
// You’ll put unit tests in the src directory in each file with the code that they’re testing. The
// convention is to create a module named tests in each file to contain the test functions and to
// annotate the module with cfg(test).
// The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only
// when you run cargo test, not when you run cargo build. This saves compile time when you only want
// to build the library and saves space in the resulting compiled artifact because the tests are not
// included. You’ll see that because integration tests go in a different directory, they don’t need
// the #[cfg(test)] annotation. However, because unit tests go in the same files as the code, you’ll
// use #[cfg(test)] to specify that they shouldn’t be included in the compiled result.
// Rust’s privacy rules do allow you to test private functions.