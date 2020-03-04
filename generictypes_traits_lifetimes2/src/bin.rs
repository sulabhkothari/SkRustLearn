// Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

use aggregator::{Tweet, Summary, NewsArticle, notify, largest, largest2};

struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest2(&char_list);
    println!("The largest char is {}", result);

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());

    notify(article);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    let result;
    let result2;
    {
        let string2 = String::from("xyz");
        result = longest(string2.as_str(), string1.as_str());
        result2 = longest2(string1.as_str(), string1.as_str());
        println!("The longest string is {}", result);
    }
    println!("{}", result2);
    // Moving result out of inner scope
    //      println!("The longest string is {}", result);
    //causes below error:
    //      error[E0597]: `string2` does not live long enough

    // The error shows that for result to be valid for the println! statement, string2 would need to
    // be valid until the end of the outer scope. Rust knows this because we annotated the lifetimes
    // of the function parameters and return values using the same lifetime parameter 'a.


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence};

    // The Static Lifetime:
    // One special lifetime we need to discuss is 'static, which means that this reference can live
    // for the entire duration of the program. All string literals have the 'static lifetime, which
    // we can annotate as follows:
    let s: &'static str = "I have a static lifetime.";
    // The text of this string is stored directly in the program’s binary, which is always available.
    // Therefore, the lifetime of all string literals is 'static.
}

// Lifetime parameter is only expected in case of functions with multiple parameters
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

//When we’re defining this function (longest without lifetime annotation), we don’t know the concrete
// values that will be passed into this function, so we don’t know whether the if case or the else
// case will execute. We also don’t know the concrete lifetimes of the references that will be passed
// in, so we can’t look at the scopes. The borrow checker can’t determine this either, because it
// doesn’t know how the lifetimes of x and y relate to the lifetime of the return value. To fix this
// error, we’ll add generic lifetime parameters that define the relationship between the references
// so the borrow checker can perform its analysis.

// Lifetime annotations don’t change how long any of the references live. Just as functions can
// accept any type when the signature specifies a generic type parameter, functions can accept
// references with any lifetime by specifying a generic lifetime parameter. Lifetime annotations
// describe the relationships of the lifetimes of multiple references to each other without
// affecting the lifetimes.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// The function signature now tells Rust that for some lifetime 'a, the function takes two parameters,
// both of which are string slices that live at least as long as lifetime 'a. The function signature
// also tells Rust that the string slice returned from the function will live at least as long as
// lifetime 'a. In practice, it means that the lifetime of the reference returned by the longest
// function is the same as the smaller of the lifetimes of the references passed in.
//  Remember, when we specify the lifetime parameters in this function signature, we’re not changing
// the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker
// should reject any values that don’t adhere to these constraints. Note that the longest function
// doesn’t need to know exactly how long x and y will live, only that some scope can be substituted
// for 'a that will satisfy this signature.
//      When annotating lifetimes in functions, the annotations go in the function signature, not in
// the function body. Rust can analyze the code within the function without any help. However, when
// a function has references to or from code outside that function, it becomes almost impossible for
// Rust to figure out the lifetimes of the parameters or return values on its own. The lifetimes might
// be different each time the function is called. This is why we need to annotate the lifetimes manually.
//      When we pass concrete references to longest, the concrete lifetime that is substituted for
// 'a is the part of the scope of x that overlaps with the scope of y. In other words, the generic
// lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and
// y. Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned
// reference will also be valid for the length of the smaller of the lifetimes of x and y.


// When returning a reference from a function, the lifetime parameter for the return type needs to
// match the lifetime parameter for one of the parameters. If the reference returned does not refer
// to one of the parameters, it must refer to a value created within this function, which would be a
// dangling reference because the value will go out of scope at the end of the function.
//fn longest3<'a>(x: &str, y: &str) -> &'a str {
//    let result = String::from("really long string");
//    result.as_str()
//}
// Here, even though we’ve specified a lifetime parameter 'a for the return type, this implementation
// will fail to compile because the return value lifetime is not related to the lifetime of the
// parameters at all.
// In this case, the best fix would be to return an owned data type rather than a reference so the
// calling function is then responsible for cleaning up the value.
fn longest3(x: &str, y: &str) -> String {
    let result = String::from("really long string");
    result
}
// Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return
// values of functions. Once they’re connected, Rust has enough information to allow memory-safe
// operations and disallow operations that would create dangling pointers or otherwise violate memory
// safety.


// This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// After writing a lot of Rust code, the Rust team found that Rust programmers were entering the same
// lifetime annotations over and over in particular situations. These situations were predictable and
// followed a few deterministic patterns. The developers programmed these patterns into the compiler’s
// code so the borrow checker could infer the lifetimes in these situations and wouldn’t need explicit
// annotations. Example -
// fn first_word(s: &str) -> &str {
//    let bytes = s.as_bytes();
//
//    for (i, &item) in bytes.iter().enumerate() {
//        if item == b' ' {
//            return &s[0..i];
//        }
//    }
//
//    &s[..]
//}
// Earlier -
// fn first_word<'a>(s: &'a str) -> &'a str {
// This piece of Rust history is relevant because it’s possible that more deterministic patterns will
// emerge and be added to the compiler. In the future, even fewer lifetime annotations might be required.
// The patterns programmed into Rust’s analysis of references are called the lifetime elision rules.
// These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler
// will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.
// The elision rules don’t provide full inference. If Rust deterministically applies the rules but
// there is still ambiguity as to what lifetimes the references have, the compiler won’t guess what
// the lifetime of the remaining references should be. In this case, instead of guessing, the compiler
// will give you an error that you can resolve by adding the lifetime annotations that specify how
// the references relate to each other.
// Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return
// values are called output lifetimes.
// Lifetime elision rules apply to fn definitions as well as impl blocks. The first rule applies to
// input lifetimes, and the second and third rules apply to output lifetimes. If the compiler gets
// to the end of the three rules and there are still references for which it can’t figure out lifetimes,
// the compiler will stop with an error.
// 1. The first rule is that each parameter that is a reference gets its own lifetime parameter. In
//      other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
//      a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32,
//      y: &'b i32); and so on.
// 2. The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned
//      to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
// 3. The third rule is if there are multiple input lifetime parameters, but one of them is &self or
//      &mut self because this is a method, the lifetime of self is assigned to all output lifetime
//      parameters. This third rule makes methods much nicer to read and write because fewer symbols
//      are necessary.


use std::fmt::Display;
// Because lifetimes are a type of generic, the declarations of the lifetime parameter 'a and the
// generic type parameter T go in the same list inside the angle brackets after the function name.
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Generic type parameters let you apply the code to different types. Traits and trait bounds ensure
// that even though the types are generic, they’ll have the behavior the code needs. You learned how
// to use lifetime annotations to ensure that this flexible code won’t have any dangling references.