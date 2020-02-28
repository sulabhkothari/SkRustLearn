mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// The front_of_house module isn’t public, but because the eat_at_restaurant function is defined in
// the same module as front_of_house (that is, eat_at_restaurant and front_of_house are siblings),
// we can refer to front_of_house from eat_at_restaurant.
// Next is the hosting module marked with pub. We can access the parent module of hosting, so we can
// access hosting. Finally, the add_to_waitlist function is marked with pub and we can access its
// parent module, so this function call works!
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// We can also construct relative paths that begin in the parent module by using super at the start
// of the path. This is like starting a filesystem path with the .. syntax.
// The fix_incorrect_order function is in the back_of_house module, so we can use super to go to the
// parent module of back_of_house, which in this case is crate, the root.
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}


mod back_of_house2 {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house2::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

// In contrast to structs, if we make an enum public, all of its variants are then public. We only
// need the pub before the enum keyword
mod back_of_house3 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant3() {
    let order1 = back_of_house3::Appetizer::Soup;
    let order2 = back_of_house3::Appetizer::Salad;
}

// Enums aren’t very useful unless their variants are public; it would be annoying to have to
// annotate all enum variants with pub in every case, so the default for enum variants is to be
// public. Structs are often useful without their fields being public, so struct fields follow the
// general rule of everything being private by default unless annotated with pub.


//use crate::front_of_house::hosting;
use front_of_house::hosting;
//instead of `use front_of_house::hosting::add_to_waitlist`
// Bringing the function’s parent module into scope with use so we have to specify the parent module
// when calling the function makes it clear that the function isn’t locally defined while still
// minimizing repetition of the full path.

pub fn eat_at_restaurant4() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// use std::collections::HashMap;
// On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to
// specify the full path. There’s no strong reason behind this idiom: it’s just the convention that
// has emerged, and folks have gotten used to reading and writing Rust code this way.

// The exception to this idiom is if we’re bringing two items with the same name into scope with use
// statements, because Rust doesn’t allow that.
//use std::fmt::Result;
//use std::io::Result as IoResult;
//
//fn function1() -> fmt::Result {
//    // --snip--
//}
//
//fn function2() -> io::Result<()> {
//    // --snip--
//}

// When we bring a name into scope with the use keyword, the name available in the new scope is
// private. To enable the code that calls our code to refer to that name as if it had been defined
// in that code’s scope, we can combine pub and use. This technique is called re-exporting because
// we’re bringing an item into scope but also making that item available for others to bring into
// their scope.

mod front_of_house5 {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house5::hosting as hosting_advanced;

pub fn eat_at_restaurant5() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

//Using Nested Paths to Clean Up Large use Lists
//use std::{cmp::Ordering, io};

//To merge:
// use std::io;
// use std::io::Write;
//We can specify in following way:
// use std::io::{self, Write};

// The Glob Operator
// use std::collections::*;
// Be careful when using the glob operator! Glob can make it harder to tell what names are in scope
// and where a name used in your program was defined.
