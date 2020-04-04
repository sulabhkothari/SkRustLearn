pub fn pattern_matching_main() {
    println!("########################Pattern Matching############################");
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        // new shadowed age variable
        // We need to place the if age > 30 condition within that block: we can’t combine these two
        // conditions into if let Ok(age) = age && age > 30. The shadowed age we want to compare to
        // 30 isn’t valid until the new scope starts with the curly bracket.
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    //let PATTERN = EXPRESSION;
    let (x, y, z) = (1, 2, 3);
    //let (x, y) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);

    matching_literals(3);

    matching_named_variables();
    multiple_patterns();
    matching_ranges();
    destructuring();
    destructuring_enums();

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    foo(3, 4);
    ignoring_values_in_a_pattern();
    match_guards();
    bindings();
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

// Patterns that will match for any possible value passed are irrefutable. An example would be x in
// the statement let x = 5; because x matches anything and therefore cannot fail to match. Patterns
// that can fail to match for some possible value are refutable. An example would be Some(x) in the
// expression if let Some(x) = a_value because if the value in the a_value variable is None rather
// than Some, the Some(x) pattern will not match.
// Function parameters, let statements, and for loops can only accept irrefutable patterns, because
// the program cannot do anything meaningful when values don’t match. The if let and while let expressions
// accept refutable and irrefutable patterns, but the compiler warns against irrefutable patterns because
// by definition they’re intended to handle possible failure: the functionality of a conditional is
// in its ability to perform differently depending on success or failure.

fn matching_literals(x: i32) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_named_variables() {
    // Because match starts a new scope, variables declared as part of a pattern inside the match expression
    // will shadow those with the same name outside the match construct, as is the case with all variables.

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_ranges() {
    let x = 5; // ranges are inclusive => 1|2|3|4|5

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destructuring() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Because having variable names match the fields is common and because writing let Point { x: x, y: y } = p;
    // contains a lot of duplication, there is a shorthand for patterns that match struct fields: you
    // only need to list the name of the struct field, and the variables created from the pattern will
    // have the same names.
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

fn destructuring_enums() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
        NestedColor(Color),
    }
    let msg = Message::ChangeColor(0, 160, 255);
    let msg = Message::NestedColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::NestedColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::NestedColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
    }
}

// There are a few ways to ignore entire values or parts of values in a pattern: using the _ pattern
// (which you’ve seen), using the _ pattern within another pattern, using a name that starts with an
// underscore, or using .. to ignore remaining parts of a value.

// Implementing a trait when you need a certain type signature but the function body in your
// implementation doesn’t need one of the parameters. The compiler will then not warn about unused
// function parameters, as it would if you used a name instead.
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn ignoring_values_in_a_pattern() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
    }
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));

    // We’ll receive an error because the s value will still be moved into _s, which prevents us from
    // using s again. However, using the underscore by itself doesn’t ever bind to the value.
    //if let Some(_s) = s {
    // This code works just fine because we never bind s to anything; it isn’t moved.
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let Point { x: a, .. } = Point { x: 0, y: 0, z: 0 };
    println!("a is {}", a);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

fn match_guards() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),

        // The match guard if n == y is not a pattern and therefore doesn’t introduce new variables.
        // This y is the outer y rather than a new shadowed y
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;

    match x {
        // The important part of this example is that the if y match guard applies to 4, 5, and 6,
        // even though it might look like if y only applies to 6.
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn bindings() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 6 };

    // Using @ lets us test a value and save it in a variable within one pattern.
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
