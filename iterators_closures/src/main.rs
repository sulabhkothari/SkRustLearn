use std::thread;
use std::time::Duration;
use std::clone::Clone;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );

    let add_one_v4 = |x| x + 1;
    println!("{}", add_one_v4(90));
    //println!("{}", add_one_v4(99.98));

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));

    // Note that we haven’t added any type annotations to the definition: if we then try to call the
    // closure twice, using a String as an argument the first time and a u32 the second time, we’ll
    // get an error.
    //let n = example_closure(5);

    // The first time we call example_closure with the String value, the compiler infers the type of
    // x and the return type of the closure to be String. Those types are then locked in to the
    // closure in example_closure, and we get a type error if we try to use a different type with the
    // same closure.

    iterators_closures::iterators_demo();
}

// memoization or lazy evaluation.
//To define structs, enums, or function parameters that use closures, we use generics and trait bounds
// The Fn traits are provided by the standard library. All closures implement at least one of the
// traits: Fn, FnMut, or FnOnce.
// We add types to the Fn trait bound to represent the types of the parameters and return values the
// closures must have to match this trait bound. In this case, our closure has a parameter of type
// u32 and returns a u32, so the trait bound we specify is Fn(u32) -> u32
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

// The trait bounds on T specify that it’s a closure by using the Fn trait. Any closure we want to
// store in the calculation field must have one u32 parameter (specified within the parentheses after
// Fn) and must return a u32 (specified after the ->).
// Note: Functions can implement all three of the Fn (Fn, FnMut, or FnOnce) traits too. If what we want to do doesn’t require
// capturing a value from the environment, we can use a function rather than a closure where we need
// something that implements an Fn trait.
// The value field is of type Option<u32>. Before we execute the closure, value will be None. When
// code using a Cacher asks for the result of the closure, the Cacher will execute the closure at that
// time and store the result within a Some variant in the value field. Then if the code asks for the
// result of the closure again, instead of executing the closure again, the Cacher will return the
// result held in the Some variant.
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


fn generate_workout(intensity: u32, random_number: u32) {
    // Closures are usually short and relevant only within a narrow context rather than in any
    // arbitrary scenario. Within these limited contexts, the compiler is reliably able to infer the
    // types of the parameters and the return type, similar to how it’s able to infer the types of
    // most variables. Making programmers annotate the types in these small, anonymous functions
    // would be annoying and largely redundant with the information the compiler already has available.
    // As with variables, we can add type annotations if we want to increase explicitness and clarity
    // at the cost of being more verbose than is strictly necessary.
    //let expensive_closure = |num| {
    //    println!("calculating slowly...");
    //    thread::sleep(Duration::from_secs(2));
    //    num
    //};

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

// Capturing the Environment with Closures:
// When a closure captures a value from its environment, it uses memory to store the values for use
// in the closure body. This use of memory is overhead that we don’t want to pay in more common cases
// where we want to execute code that doesn’t capture its environment. Because functions are never
// allowed to capture their environment, defining and using functions will never incur this overhead.
// Closures can capture values from their environment in three ways, which directly map to the three
// ways a function can take a parameter: taking ownership, borrowing mutably, and borrowing immutably.
// These are encoded in the three Fn traits as follows:
//    1. FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s
//      environment. To consume the captured variables, the closure must take ownership of these
//      variables and move them into the closure when it is defined. The Once part of the name
//      represents the fact that the closure can’t take ownership of the same variables more than
//      once, so it can be called only once.
//    2. FnMut can change the environment because it mutably borrows values.
//    3. Fn borrows values from the environment immutably.
// When you create a closure, Rust infers which trait to use based on how the closure uses the values
// from the environment. All closures implement FnOnce because they can all be called at least once.
// Closures that don’t move the captured variables also implement FnMut, and closures that don’t need
// mutable access to the captured variables also implement Fn.
// If you want to force the closure to take ownership of the values it uses in the environment, you
// can use the move keyword before the parameter list. This technique is mostly useful when passing
// a closure to a new thread to move the data so it’s owned by the new thread.


#[test]
fn test_fn() {
    // the equal_to_x closure borrows x immutably (so equal_to_x has the Fn trait) because the body
    // of the closure only needs to read the value in x
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}

#[test]
fn closure_ownership_via_move() {
    let x = vec![1, 2, 3];
    let k = x.clone();

    let equal_to_x = |z| z == x;
    //let equal_to_m = move || x;


    //error
    //println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];
    let z = vec![1, 2, 3];
    //let nb = vec![1, 2, 3];
    //let mb = vec![1, 2, 3];


    assert!(equal_to_x(y));
    assert!(equal_to_x(k));
    //assert!(equal_to_x(z));
    //assert_eq!(equal_to_m(), nb);
    //assert_eq!(equal_to_m(), mb);
    //assert!(equal_to_x(x));

    //error
    //assert!(equal_to_x(y));
}

// cargo test pass --bin iterators_closures

