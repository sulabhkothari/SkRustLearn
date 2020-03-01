use std::string::ToString;

fn main() {
    vectors();
    strings();
    hashmaps();
}

fn vectors() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    match v.binary_search(&16) {
        Ok(pos) => v.insert(pos, 16),
        Err(_) => v.push(16)
    }

    match v.binary_search(&12) {
        Ok(pos) => v.insert(pos, 12),
        Err(pos) => v.insert(pos, 12)
    }

    println!("Binary Search -> {:?}", v);

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // When the program has a valid reference, the borrow checker enforces the ownership and
    // borrowing rules (covered in Chapter 4) to ensure this reference and any other references to
    // the contents of the vector remain valid. Recall the rule that states you can’t have mutable
    // and immutable references in the same scope. That rule applies in Listing 8-7, where we hold
    // an immutable reference to the first element in a vector and try to add an element to the end,
    // which won’t work.
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    //Below line causes Compilation Error
    //println!("The first element is: {}", first);

    // This error is due to the way vectors work: adding a new element onto the end of the vector
    // might require allocating new memory and copying the old elements to the new space, if there
    // isn’t enough room to put all the elements next to each other where the vector currently is.
    // In that case, the reference to the first element would be pointing to deallocated memory.
    // The borrowing rules prevent programs from ending up in that situation.

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // To change the value that the mutable reference refers to, we have to use the dereference
    // operator (*) to get to the value in i before we can use the += operator.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }


    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn strings() {
    let mut s = String::new();
    let m = String::from("sdfsdf");


    let data = "initial contents";

    let s = data.to_string();

// the method also works on a literal directly:
    let s = "initial contents".to_string();


    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    use std::ops::Add;

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // The reason we’re able to use &s2 in the call to add is that the compiler can coerce the
    // &String argument into a &str. When we call the add method, Rust uses a deref coercion, which
    // here turns &s2 into &s2[..]. We’ll discuss deref coercion in more depth in Chapter 15.
    // Because add does not take ownership of the s parameter, s2 will still be a valid String after
    // this operation.
    // looks like it will copy both strings and create a new one, this statement actually takes
    // ownership of s1, appends a copy of the contents of s2, and then returns ownership of the
    // result. In other words, it looks like it’s making a lot of copies but isn’t; the
    // implementation is more efficient than copying.
    //let s3 = s1.add(&s2);
    let s3 = s1 + &s2;

    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    //let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    // The version of the code using format! is much easier to read and doesn’t take ownership of
    // any of its parameters.
    println!("{}", s1);

    // A String is a wrapper over a Vec<u8>
    let len = String::from("Hola").len();
    // In this case, len will be 4, which means the vector storing the string “Hola” is 4 bytes long.
    // Each of these letters takes 1 byte when encoded in UTF-8
    println!("{}", len);

    let len = String::from("Здравствуйте").len();
    println!("{}", len);
    // It takes 24 bytes to encode “Здравствуйте” in UTF-8, because each Unicode scalar value in that string
    // takes 2 bytes of storage. Therefore, an index into the string’s bytes will not always
    // correlate to a valid Unicode scalar value. To demonstrate, consider this invalid Rust code:
    // let hello = "Здравствуйте";
    // let answer = &hello[0];
    // println!("{}", answer);
    // error[E0277]: the type `str` cannot be indexed by `{integer}`

    // Another point about UTF-8 is that there are actually three relevant ways to look at strings
    // from Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to
    // what we would call letters).
    // “नमस्ते”
    // Bytes: [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // Unicode scalar values (Rust's char type): ['न', 'म', 'स', '्', 'त', 'े']
    // There are six char values here, but the fourth and sixth are not letters: they’re diacritics
    // that don’t make sense on their own
    // Grapheme clusters: ["न", "म", "स्", "ते"]
    let namaste = "नमस्ते";
    println!("{}", &namaste[0..12]);
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        print!("{},", b);
    }

    // But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.
    // Getting grapheme clusters from strings is complex, so this functionality is not provided by
    //  the standard library. Crates are available on crates.io if this is the functionality you need.
}

fn hashmaps() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // Note that we need to first use the HashMap from the collections portion of the standard
    // library. Of our three common collections, this one is the least often used, so it’s not
    // included in the features brought into scope automatically in the prelude.

    // The type annotation HashMap<_, _> is needed here because it’s possible to collect into many
    // different data structures and Rust doesn’t know which you want unless you specify. For the
    // parameters for the key and value types, however, we use underscores, and Rust can infer the
    // types that the hash map contains based on the types of the data in the vectors.
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    println!("");
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    for (k, v) in &scores {
        println!("{},{}", k, v);
    }
    let score = scores.get(&String::from("Blue"));
    match score {
        Some(s) => println!("{}", s),
        None => ()
    }

    // For types that implement the Copy trait, like i32, the values are copied into the hash map.
    // For owned values like String, the values will be moved and the hash map will be the owner of
    // those values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    //error[E0382]: borrow of moved value: `field_name`
    //println!("{}", field_name);

    // If we insert references to values into the hash map, the values won’t be moved into the hash
    // map. The values that the references point to must be valid for at least as long as the hash
    // map is valid.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Here, score will have the value that’s associated with the Blue team, and the result will be
    // Some(&10). The result is wrapped in Some because get returns an Option<&V>
    let team_name = String::from("Blue");
    // get borrows key so its passed using &
    let score = scores.get(&team_name);
    match score {
        Some(num) => println!("{}", num),
        None => ()
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwriting a Value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Only Inserting a Value If the Key Has No Value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    // The or_insert method actually returns a mutable reference (&mut V) to the value for this key.
    // Here we store that mutable reference in the count variable, so in order to assign to that
    // value, we must first dereference count using the asterisk (*). The mutable reference goes out
    // of scope at the end of the for loop, so all of these changes are safe and allowed by the
    // borrowing rules.

    // Hashing Functions
    // By default, HashMap uses a “cryptographically strong”1 hashing function that can provide
    // resistance to Denial of Service (DoS) attacks. This is not the fastest hashing algorithm
    // available, but the trade-off for better security that comes with the drop in performance is
    // worth it. If you profile your code and find that the default hash function is too slow for
    // your purposes, you can switch to another function by specifying a different hasher. A hasher
    // is a type that implements the BuildHasher trait. We’ll talk about traits and how to implement
    // them in Chapter 10. You don’t necessarily have to implement your own hasher from scratch;
    // crates.io has libraries shared by other Rust users that provide hashers implementing many
    // common hashing algorithms.
}