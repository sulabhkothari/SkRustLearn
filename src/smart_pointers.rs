pub fn test_smart_pointers() {
    println!("TEST SMART_POINTERS")
}

// Boxes don’t have performance overhead, other than storing their data on the heap instead of on the
// stack. But they don’t have many extra capabilities either. You’ll use them most often in these
// situations:
//    1. When you have a type whose size can’t be known at compile time and you want to use a value
//       of that type in a context that requires an exact size
//    2. When you have a large amount of data and you want to transfer ownership but ensure the data
//       won’t be copied when you do so
//    3. When you want to own a value and you care only that it’s a type that implements a particular
//       trait rather than being of a specific type

fn loan(spd: CustomSmartPointer) {
    //my stuff is dropped at the end of this function
    //  because it was transferred the ownership.
}

pub fn smart_pointers_main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
    // Normally d would be dropped before c, because drop follows reverse order from construction.

    loan(c);

    // We can’t disable the automatic insertion of drop when a value goes out of scope, and we can’t
    // call the drop method explicitly. So, if we need to force a value to be cleaned up early, we
    // can use the std::mem::drop function.
    std::mem::drop(d);
    let b = Box::new(5);
    println!("b = {}", b);

    use List::{Cons, Nil};
    //let list = Box::new(Cons(12, Box::new(Cons(44, Box::new(Nil)))));

    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let m = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *m);

    let m = MyBox::new(String::from("Rust"));

    // Because we implemented the Deref trait on MyBox<T> in Listing 15-10, Rust can turn &MyBox<String>
    // into &String by calling deref (because of the deref definition which requires reference to MyBox:
    // fn deref(&self) -> &T ). The standard library provides an implementation of Deref on String
    // that returns a string slice, and this is in the API documentation for Deref. Rust calls deref
    // again to turn the &String into &str, which matches the hello function’s definition.
    hello(&m);

    let a = Rc::new(Cons(Rc::new(RefCell::new(5)), Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // multiple mutable borrows to the same place can cause data races and inconsistencies


    // We create a value that is an instance of Rc<RefCell<i32>> and store it in a variable named
    // value so we can access it directly later.
    // We need to clone value so both a and value have ownership of the inner 5 value rather than
    // transferring ownership from value to a or having a borrow from value.
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // After we’ve created the lists in a, b, and c, we add 10 to the value in value. We do this by
    // calling borrow_mut on value, which uses the automatic dereferencing feature
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    memory_leaking();

    tree_run();

    visualize_changes_to_strong_weak_count();
}

//use crate::List::{Cons, Nil};
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::smart_pointers::MemoryLeakingList::{MemoryLeakingCons, MemoryLeakingNil};


impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// Without the Deref trait, the compiler can only dereference & references. The deref method gives
// the compiler the ability to take a value of any type that implements Deref and call the deref
// method to get a & reference that it knows how to dereference.
//When we entered *y in Listing 15-9, behind the scenes Rust actually ran this code:
//*(y.deref())

// The reason the deref method returns a reference to a value, and that the plain dereference outside
// the parentheses in *(y.deref()) is still necessary, is the ownership system. If the deref method
// returned the value directly instead of a reference to the value, the value would be moved out of
// self. We don’t want to take ownership of the inner value inside MyBox<T> in this case or in most
// cases where we use the dereference operator.

// Implicit Deref Coercions with Functions and Methods:
// Deref coercion is a convenience that Rust performs on arguments to functions and methods. Deref
// coercion converts a reference to a type that implements Deref into a reference to a type that Deref
// can convert the original type into. Deref coercion happens automatically when we pass a reference
// to a particular type’s value as an argument to a function or method that doesn’t match the parameter
// type in the function or method definition. A sequence of calls to the deref method converts the type
// we provided into the type the parameter needs.

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// When the Deref trait is defined for the types involved, Rust will analyze the types and use
// Deref::deref as many times as necessary to get a reference to match the parameter’s type. The
// number of times that Deref::deref needs to be inserted is resolved at compile time, so there is
// no runtime penalty for taking advantage of deref coercion!

// How Deref Coercion Interacts with Mutability:
// Rust does deref coercion when it finds types and trait implementations in three cases -
//    1. From &T to &U when T: Deref<Target=U>
//    2. From &mut T to &mut U when T: DerefMut<Target=U>
//    3. From &mut T to &U when T: Deref<Target=U>

struct CustomSmartPointer {
    data: String,
}

//The Drop trait is included in the prelude
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// You also don’t have to worry about problems resulting from accidentally cleaning up values still
// in use: the ownership system that makes sure references are always valid also ensures that drop
// gets called only once when the value is no longer being used.

// Reasons to choose Box<T>, Rc<T>, or RefCell<T>:
//    1. Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
//    2. Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only
//       immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows
//       checked at runtime.
//    3. Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value
//       inside the RefCell<T> even when the RefCell<T> is immutable.
// Mutating the value inside an immutable value is the interior mutability pattern.

// The standard library has other types that provide interior mutability, such as Cell<T>, which is
// similar except that instead of giving references to the inner value, the value is copied in and
// out of the Cell<T>.

enum LL {
    Node(i32, Box<LL>),
    Nil,
}

impl LL {
    fn last(&self) {
        use crate::smart_pointers::LL::Node;

        match self {
            Node(x, y) => println!(""),
            Nil => println!("")
        }
    }
}

#[derive(Debug)]
enum MemoryLeakingList {
    MemoryLeakingCons(i32, RefCell<Rc<MemoryLeakingList>>),
    MemoryLeakingNil,
}

impl MemoryLeakingList {
    fn tail(&self) -> Option<&RefCell<Rc<MemoryLeakingList>>> {
        use crate::smart_pointers::MemoryLeakingList::MemoryLeakingCons;
        match self {
            MemoryLeakingCons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn memory_leaking() {
    println!("=================================================================================================");
    let a = Rc::new(MemoryLeakingCons(5, RefCell::new(Rc::new(MemoryLeakingNil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(MemoryLeakingCons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        // We use the borrow_mut method on the RefCell<Rc<List>> to change the value inside from an
        // Rc<List> that holds a Nil value to the Rc<List> in b.
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    //println!("a next item = {:?}", a.tail());

    // The reference count of the Rc<List> instances in both a and b are 2 after we change the list
    // in a to point to b. At the end of main, Rust will try to drop b first, which will decrease
    // the count of the Rc<List> instance in b by 1.
    // However, because a is still referencing the Rc<List> that was in b, that Rc<List> has a count
    // of 1 rather than 0, so the memory the Rc<List> has on the heap won’t be dropped. The memory
    // will just sit there with a count of 1, forever.
    /*
     _ _      _____
    | a |--->| 5|  |
    |_ _|    |__|__|
     /\           |
    __|____      _v_
    |  |10|<----| b|
    |__|__|     |__|

    */
    // If a more complex program allocated lots of memory in a cycle and held onto it for a long
    // time, the program would use more memory than it needed and might overwhelm the system, causing
    // it to run out of available memory.
    // If you have RefCell<T> values that contain Rc<T> values or similar nested combinations of types
    // with interior mutability and reference counting, you must ensure that you don’t create cycles;
    // you can’t rely on Rust to catch them.
    // A solution for avoiding reference cycles is reorganizing your data structures so that some
    // references express ownership and some references don’t. As a result, you can have cycles made
    // up of some ownership relationships and some non-ownership relationships, and only the ownership
    // relationships affect whether or not a value can be dropped.
}

// Weak_count doesn’t need to be 0 for the Rc<T> instance to be cleaned up.
// Strong references are how you can share ownership of an Rc<T> instance. Weak references don’t
// express an ownership relationship. They won’t cause a reference cycle because any cycle involving
// some weak references will be broken once the strong reference count of values involved is 0.
// Because the value that Weak<T> references might have been dropped, to do anything with the value
// that a Weak<T> is pointing to, you must make sure the value still exists. Do this by calling the
// upgrade method on a Weak<T> instance, which will return an Option<Rc<T>>

// Tree:
#[derive(Debug)]
// We also want to modify the nodes which are children of another node, so we have a RefCell<T> in
// children around the Vec<Rc<Node>>.
// Thinking about the relationships another way, a parent node should own its children: if a parent
// node is dropped, its child nodes should be dropped as well. However, a child should not own its
// parent: if we drop a child node, the parent should still exist. This is a case for weak references!
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn tree_run() {
    println!("=================================================================================================");

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!("strongcount={}, weakcount={}", Rc::strong_count(&leaf), Rc::weak_count(&branch))
}

fn visualize_changes_to_strong_weak_count() {
    println!("#########################################################################################");

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

// Use smart pointers to make different guarantees and trade-offs from those Rust makes by default
// with regular references. The Box<T> type has a known size and points to data allocated on the heap.
// The Rc<T> type keeps track of the number of references to data on the heap so that data can have
// multiple owners. The RefCell<T> type with its interior mutability gives us a type that we can use
// when we need an immutable type but need to change an inner value of that type; it also enforces
// the borrowing rules at runtime instead of at compile time.

// https://doc.rust-lang.org/stable/nomicon/
// https://rust-unofficial.github.io/too-many-lists/