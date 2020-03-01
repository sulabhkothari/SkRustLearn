extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn guessing_game() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn five() -> i32 {
    5
}

fn data_types() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let tuple = (10, "shdjvf", true);
    println!("{}", tuple.0);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[0]);
    let a: [i32; 5] = [67; 5];

    let index = 5;
    //println!("{}", a[index])

    let x = five();

    println!("The value of x is: {}", x);
}

fn control_flows() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);


    // Incompatible if and else expression types
    //
    //    let condition = true;
    //
    //    let number = if condition {
    //        5
    //    } else {
    //        "six"
    //    };
    //
    //    println!("The value of number is: {}", number);
}

fn loops() {
    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    println!("============================================");
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    println!("============================================");

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn ownership() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = s.clone();

    println!("{}, world!", s);

    // Rust has a special annotation called the Copy trait that we can place on types like integers
    // that are stored on the stack (we’ll talk more about traits in Chapter 10). If a type has the
    // Copy trait, an older variable is still usable after assignment. Rust won’t let us annotate a
    // type with the Copy trait if the type, or any of its parts, has implemented the Drop trait. If
    // the type needs something special to happen when the value goes out of scope and we add the
    // Copy annotation to that type, we’ll get a compile-time error.
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it’s okay to still
    // use x afterward

    //Causes: error[E0382]: borrow of moved value: `s`
    //println!("{}",s);

    let s1 = gives_ownership();         // gives_ownership moves its return value into s1
    println!("Giives Ownership : {}", s1);

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3
    println!("Takes and gives back ownership: {}", s3);

    let s1 = String::from("hello");

    // Borrow
    let len = calculate_length_by_borrowing(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut sn = String::from("Hello");
    change(&mut sn);
    println!("{}", sn);

    change(&mut sn);
    println!("{}", sn);

    // cannot borrow `s` as mutable more than once at a time
    //    let mut s = String::from("hello");
    //
    //    let r1 = &mut s;
    //    let r2 = &mut s;
    //
    //    println!("{}, {}", r1, r2);


    // As always, we can use curly brackets to create a new scope, allowing for multiple mutable
    // references, just not simultaneous ones:
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    //    let mut s = String::from("hello");
    //
    //    let r1 = &s; // no problem
    //    let r2 = &s; // no problem
    //    let r3 = &mut s; // BIG PROBLEM
    //
    //    println!("{}, {}, and {}", r1, r2, r3);
    // Whew! We also cannot have a mutable reference while we have an immutable one. Users of an
    // immutable reference don’t expect the values to suddenly change out from under them! However,
    // multiple immutable references are okay because no one who is just reading the data has the
    // ability to affect anyone else’s reading of the data.

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
    let reference_to_nothing = dangle();
}

// Because s is created inside dangle, when the code of dangle is finished, s will be deallocated.
// But we tried to return a reference to it. That means this reference would be pointing to an
// invalid String. That’s no good! Rust won’t let us do this.
fn dangle() -> String /*&String*/ {
    let s = String::from("hello");

    // error[E0106]: missing lifetime specifier
    //&s
    s
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length_by_borrowing(s: &String) -> usize {
    s.len()
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}

fn slices() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    //s.clear(); // error!

    println!("{}", word);

    // Literally embedded in the binary, and the reference is a reference to the location in the binary.
    let s = "Hello, world!";
    // The type of s here is &str: it’s a slice pointing to that specific point of the binary. This
    // is also why string literals are immutable; &str is an immutable reference.
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Preprocessor enables printing structures using println!
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Shorthand if function parameter names are exactly same as struct attributes names
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// we want to borrow the struct rather than take ownership of it
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn calculateArea(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Function
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
// The println! macro can do many kinds of formatting, and by default, the curly brackets tell
// println! to use formatting known as Display: output intended for direct end user consumption.
// The primitive types we’ve seen so far implement Display by default, because there’s only one way
// you’d want to show a 1 or any other primitive type to a user. But with structs, the way println!
// should format the output is less clear because there are more display possibilities: Do you want
// commas or not? Do you want to print the curly brackets? Should all the fields be shown? Due to
// this ambiguity, Rust doesn’t try to guess what we want, and structs don’t have a provided
// implementation of Display.
//
// The println! macro call will now look like println!("rect1 is {:?}", rect1);. Putting the
// specifier :? inside the curly brackets tells println! we want to use an output format called Debug.
// The Debug trait enables us to print our struct in a way that is useful for developers so we can
// see its value while we’re debugging our code.
//
// Rust does include functionality to print out debugging information, but we have to explicitly opt
// in to make that functionality available for our struct. To do that, we add the annotation
// #[derive(Debug)] just before the struct definition
//
// Rust has provided a number of traits for us to use with the derive annotation that can add useful
// behavior to our custom types. Those traits and their behaviors are listed in Appendix C
//
// p1.distance(&p2);
//(&p1).distance(&p2);
// The first one looks much cleaner. This automatic referencing behavior works because methods have
// a clear receiver—the type of self. Given the receiver and name of a method, Rust can figure out
// definitively whether the method is reading (&self), mutating (&mut self), or consuming (self).
// The fact that Rust makes borrowing implicit for method receivers is a big part of making
// ownership ergonomic in practice.

fn structures() {
    let sk = build_user(String::from("sk@gmail.com"), String::from("sk"));
    println!("{}", sk.username);
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..sk
    };
    println!("{}", user2.sign_in_count);
    println!("{:?}", user2);

    #[derive(Debug)]
    struct Color(i32, i32, i32);

    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}", black);
    println!("{:?}", origin);
    println!("{}", origin.0);

    let rect1 = Rectangle { width: 30, height: 50 };
    let ar = area(&rect1);
    println!(
        "The area of the rectangle {:#?} is {} square pixels.", rect1, ar
    );

    println!(
        "Calculated Area: {}", rect1.calculateArea()
    );

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("{:?}", Rectangle::square(12))
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    //Struct
    Write(String),
    //String
    ChangeColor(i32, i32, i32),    //Tuple
}

impl Message {
    fn call(&self) {
        println!("{:?}", self)
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Texas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
}

fn enums() {
    let a = Message::Write(String::from("Sulabh kothari"));
    a.call();
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    //let x: Option<i8> = Some(5);
    //let y: Option<i8> = Some(5);
    //let sum = x + y;
    //println!("{}", sum);

    println!("{}", Coin::Nickel.value_in_cents());
    println!("{}", Coin::Quarter(UsState::California).value_in_cents());


    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
            _ => None
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}", six, none);

    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("Three!!")
    }

    let coin = Coin::Quarter(UsState::Alabama);

    let coins = [Coin::Quarter(UsState::Alabama), Coin::Quarter(UsState::Alaska), Coin::Dime, Coin::Nickel, Coin::Penny];

    let mut count = 0;

    for coin in coins.iter() {
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
        }
    }

    println!("Number of non quarter coins: {}", count);
}

fn main() {
    enums();
}

