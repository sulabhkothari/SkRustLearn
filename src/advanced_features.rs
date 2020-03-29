// To switch to unsafe Rust, use the unsafe keyword and then start a new block that holds the unsafe
// code. You can take four actions in unsafe Rust, called unsafe superpowers, that you can’t in safe
// Rust. Those superpowers include the ability to:
//    1. Dereference a raw pointer
//    2. Call an unsafe function or method
//    3. Access or modify a mutable static variable
//    4. Implement an unsafe trait
//    5. Access fields of unions
//It’s important to understand that unsafe doesn’t turn off the borrow checker or disable any other
// of Rust’s safety checks: if you use a reference in unsafe code, it will still be checked. The unsafe
// keyword only gives you access to these four features that are then not checked by the compiler
// for memory safety. You’ll still get some degree of safety inside of an unsafe block.

// Different from references and smart pointers, raw pointers:
//    1. Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or
//      multiple mutable pointers to the same location
//    2. Aren’t guaranteed to point to valid memory
//    3. Are allowed to be null
//    4. Don’t implement any automatic cleanup

pub fn advanced_features_main(){
    println!("########################Raw Pointers############################");

    let mut num = 5;

    // We’ve created raw pointers by using as to cast an immutable and a mutable reference into their
    // corresponding raw pointer types. Because we created them directly from references guaranteed
    // to be valid, we know these particular raw pointers are valid, but we can’t make that assumption
    // about just any raw pointer.
    // With raw pointers, we can create a mutable pointer and an immutable pointer to the same location
    // and change data through the mutable pointer, potentially creating a data race. Be careful!
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        dangerous();
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    //let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut2(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    //likely_crash();

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    advanced_traits();
    advanced_functions_and_closures();
}

unsafe fn dangerous() {
    println!("Unsafe Call!");
}

fn split_at_mut2(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    // We have a mutable slice to i32 values, as_mut_ptr returns a raw pointer with the type *mut i32
    let ptr = slice.as_mut_ptr();

    // Ensures unsafe code will work.
    assert!(mid <= len);

    // Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice; it
    // only knows that we’re borrowing from the same slice twice. Borrowing different parts of a slice
    // is fundamentally okay because the two slices aren’t overlapping, but Rust isn’t smart enough
    // to know this. When we know code is okay, but Rust doesn’t, it’s time to reach for unsafe code.
    //(&mut slice[..mid], &mut slice[mid..])

    use std::slice;
    unsafe {
        // the slice::from_raw_parts_mut function takes a raw pointer and a length, and it creates a slice
        // The function slice::from_raw_parts_mut is unsafe because it takes a raw pointer and must
        // trust that this pointer is valid.
        // The offset method on raw pointers is also unsafe, because it must trust that the offset
        // location is also a valid pointer.
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

fn likely_crash() {
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let slice: &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 100000)
    };

    // [1]    99298 segmentation fault
    for s in slice {
        println!("SLICE --> {}", s);
    }
}

// Sometimes, your Rust code might need to interact with code written in another language. For this,
// Rust has a keyword, extern, that facilitates the creation and use of a Foreign Function Interface
// (FFI). An FFI is a way for a programming language to define functions and enable a different (foreign)
// programming language to call those functions.

// Within the extern "C" block, we list the names and signatures of external functions from another
// language we want to call. The "C" part defines which application binary interface (ABI) the external
// function uses: the ABI defines how to call the function at the assembly level. The "C" ABI is the
// most common and follows the C programming language’s ABI.

extern "C" {
    fn abs(input: i32) -> i32;
}


// Calling Rust Functions from Other Languages:
// Rust function to be nameable by other languages, we must disable the Rust compiler’s name mangling.
// We make the call_from_c function accessible from C code, after it’s compiled to a shared library
// and linked from C:
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
//This usage of extern does not require unsafe.

// Accessing or Modifying a Mutable Static Variable:
// Until now, we’ve not talked about global variables, which Rust does support but can be problematic
// with Rust’s ownership rules. If two threads are accessing the same mutable global variable, it can
// cause a data race. In Rust, global variables are called static variables.
// SCREAMING_SNAKE_CASE by convention
static HELLO_WORLD: &str = "Hello, world!";
// Constants and immutable static variables might seem similar, but a subtle difference is that values
// in a static variable have a fixed address in memory. Using the value will always access the same
// data. Constants, on the other hand, are allowed to duplicate their data whenever they’re used.
// Another difference between constants and static variables is that static variables can be mutable.
// Accessing and modifying mutable static variables is unsafe.
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
// Any code that reads or writes from COUNTER must be within an unsafe block.
// Having multiple threads access COUNTER would likely result in data races.
// With mutable data that is globally accessible, it’s difficult to ensure there are no data races,
// which is why Rust considers mutable static variables to be unsafe.

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
// By using unsafe impl, we’re promising that we’ll uphold the invariants that the compiler can’t verify.

// If we implement a type that contains a type that is not Send or Sync, such as raw pointers, and
// we want to mark that type as Send or Sync, we must use unsafe. Rust can’t verify that our type
// upholds the guarantees that it can be safely sent across threads or accessed from multiple threads;
// therefore, we need to do those checks manually and indicate as such with unsafe.

// Default Generic Type Parameters and Operator Overloading:
// RHS=Self: this syntax is called default type parameters.
// trait Add<RHS=Self> {
//    type Output;
//
//    fn add(self, rhs: RHS) -> Self::Output;
//}

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// When we implemented Add for Point, we used the default for RHS because we wanted to add two Point instances.
impl Add for Point {
    // Associated Type:
    // In other words, when a trait has a generic parameter, it can be implemented for a type multiple
    // times, changing the concrete types of the generic type parameters each time.
    // With associated types, we don’t need to annotate types because we can’t implement a trait on
    // a type multiple times.
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// We can implement Add for Millimeters with Meters as the RHS
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
// You’ll use default type parameters in two main ways:
//    1. To extend a type without breaking existing code
//    2. To allow customization in specific cases most users won’t need


// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name:
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// Associated functions that are part of traits don’t have a self parameter
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}


// Using Supertraits to Require One Trait’s Functionality Within Another Trait:
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
// Because we’ve specified that OutlinePrint requires the Display trait, we can use the to_string
// function that is automatically implemented for any type that implements Display.

impl OutlinePrint for Point {}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


// Using the Newtype Pattern to Implement External Traits on External Types:
// (Newtype is a term that originates from the Haskell programming language.)
// Orphan rule: We’re allowed to implement a trait on a type as long as either the trait or the type
// are local to our crate. It’s possible to get around this restriction using the newtype pattern,
// which involves creating a new type in a tuple struct.
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
// If we wanted the new type to have every method the inner type has, implementing the Deref trait
// on the Wrapper to return the inner type would be a solution.

// Creating Type Synonyms with Type Aliases:
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;
type Result<T> = std::result::Result<T, std::io::Error>;

// The Never Type that Never Returns:
// Rust has a special type named ! that’s known in type theory lingo as the empty type because it has
// no values. We prefer to call it the never type because it stands in the place of the return type
// when a function will never return.
// Functions that return never are called diverging functions. We can’t create values of the type !
// so bar can never possibly return.
// continue has a ! value. That is, when Rust computes the type of guess, it looks at both match arms,
// the former with a value of u32 and the latter with a ! value. Because ! can never have a value,
// Rust decides that the type of guess is u32.
// Example:
// let guess: u32 = match guess.trim().parse() {
//    Ok(num) => num,
//    Err(_) => continue,
//};
// The formal way of describing this behavior is that expressions of type ! can be coerced into any
// other type. We’re allowed to end this match arm with continue because continue doesn’t return a
// value; instead, it moves control back to the top of the loop, so in the Err case, we never assign
// a value to guess.
// Another Example: Option's unwrap method
// impl<T> Option<T> {
//    pub fn unwrap(self) -> T {
//        match self {
//            Some(val) => val,
//            None => panic!("called `Option::unwrap()` on a `None` value"),
//        }
//    }
//}

// Dynamically Sized Types and the Sized Trait:
// The concept of dynamically sized types. Sometimes referred to as DSTs or unsized types, these types
// let us write code using values whose size we can know only at runtime.
// So although a &T is a single value that stores the memory address of where the T is located, a &str
// is two values: the address of the str and its length. As such, we can know the size of a &str value
// at compile time: it’s twice the length of a usize.
// We can combine str with all kinds of pointers: for example, Box<str> or Rc<str>. In fact, you’ve
// seen this before but with a different dynamically sized type: traits. Every trait is a dynamically
// sized type we can refer to by using the name of the trait.
// &dyn Trait or Box<dyn Trait> (Rc<dyn Trait> would work too).
// To work with DSTs, Rust has a particular trait called the Sized trait to determine whether or not
// a type’s size is known at compile time. This trait is automatically implemented for everything
// whose size is known at compile time. In addition, Rust implicitly adds a bound on Sized to every
// generic function.
// fn generic<T>(t: T) {
//    // --snip--
//}
// is actually treated as though we had written this:
//
//fn generic<T: Sized>(t: T) {
//    // --snip--
//}

// fn generic<T: ?Sized>(t: &T) {
//    // --snip--
//}
// A trait bound on ?Sized is the opposite of a trait bound on Sized: we would read this as “T may
// or may not be Sized.” This syntax is only available for Sized, not any other traits. Also note that
// we switched the type of the t parameter from T to &T. Because the type might not be Sized, we need
// to use it behind some kind of pointer.


fn advanced_traits() {
    println!("############################################Advanced Traits & Types########################################");
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("A baby dog is called a {}", Dog::baby_name());

    // Because Animal::baby_name is an associated function rather than a method, and thus doesn’t
    // have a self parameter, Rust can’t figure out which implementation of Animal::baby_name we want.
    // println!("A baby dog is called a {}", Animal::baby_name());

    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("[fully qualified syntax] A baby dog is called a {}", <Dog as Animal>::baby_name());

    let p = Point{x:14, y:809};
    p.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

// Advanced functions and closures:
// The fn type is called a function pointer.
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), so you can
// always pass a function pointer as an argument for a function that expects a closure. It’s best to
// write functions using a generic type and one of the closure traits so your functions can accept
// either functions or closures.

// fn returns_closure() -> Fn(i32) -> i32 {
//    |x| x + 1
// }
// The error references the Sized trait again! Rust doesn’t know how much space it will need to store
// the closure.
// We can use a trait object:
// fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
//    Box::new(|x| x + 1)
// }

fn advanced_functions_and_closures(){
    println!("############################################Advanced functions and closures########################################");

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();

    // These types use () as initializer syntax
    // We can use these initializer functions as function pointers that implement the closure traits,
    // which means we can specify the initializer functions as arguments for methods that take closures
    enum Status {
        Value(u32),
        Stop,
        Quit{a:i32}
    }

    let list_of_statuses: Vec<Status> =
        (0u32..20)
            .map(Status::Value)
            .collect();
}