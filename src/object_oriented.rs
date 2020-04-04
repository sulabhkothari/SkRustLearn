pub fn object_oriented_main() {
    println!("########################object oriented rust############################");
    println!("OOPS");
    let s = Ss {};

    // Disambiguate the method call for candidate:  object_oriented::T::mm(&s) or  object_oriented::U::mm(&s);
    T::mm(&s);

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    let mut typedPost = TypedPost::new();

    typedPost.add_text("I ate a salad for lunch today");

    let typedPost = typedPost.request_review();

    let typedPost = typedPost.approve();

    assert_eq!("I ate a salad for lunch today", typedPost.content());
}

pub trait Draw {
    fn draw(&self);

    // Below line will fail compilation: the trait `object_oriented::Draw` cannot be made into an object
    //fn m<R>(&self, r: R) -> R;
}

// We can define a vector that takes a trait object. A trait object points to both an instance of a
// type implementing our specified trait as well as a table used to look up trait methods on that
// type at runtime. We create a trait object by specifying some sort of pointer, such as a & reference
// or a Box<T> smart pointer, then the dyn keyword, and then specifying the relevant trait.
// Wherever we use a trait object, Rust’s type system will ensure at compile time that any value used
// in that context will implement the trait object’s trait. Consequently, we don’t need to know all
// the possible types at compile time.
// Box<dyn Draw>, which is a trait object; it’s a stand-in for any type inside a Box that implements
// the Draw trait.

// When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t know all the types
// that might be used with the code that is using trait objects, so it doesn’t know which method
// implemented on which type to call. Instead, at runtime, Rust uses the pointers inside the trait
// object to know which method to call. There is a runtime cost when this lookup happens that doesn’t
// occur with static dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a
// method’s code, which in turn prevents some optimizations.

// Object Safety Is Required for Trait Objects:
// You can only make object-safe traits into trait objects. Some complex rules govern all the properties
// that make a trait object safe, but in practice, only two rules are relevant. A trait is object safe
// if all the methods defined in the trait have the following properties:
//    1. The return type isn’t Self.
//    2. There are no generic type parameters.
// The Self keyword is an alias for the type we’re implementing the traits or methods on. Trait objects
// must be object safe because once you’ve used a trait object, Rust no longer knows the concrete type
// that’s implementing that trait. If a trait method returns the concrete Self type, but a trait object
// forgets the exact type that Self is, there is no way the method can use the original concrete type.
// The same is true of generic type parameters that are filled in with concrete type parameters when
// the trait is used: the concrete types become part of the type that implements the trait. When the
// type is forgotten through the use of a trait object, there is no way to know what types to fill
// in the generic type parameters with.
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("Drawing Button {:?}!!", self);
    }
}

// If someone using our library decides to implement a SelectBox struct that has width, height, and
// options fields, they implement the Draw trait on the SelectBox type as well.
#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("Drawing Select Box {:?}!!", self);
    }
}

trait T {
    fn mm(&self);
}

trait U {
    fn mm(&self);
}

struct Ss {}

impl T for Ss {
    fn mm(&self) {
        println!("Inside T - Ss");
    }
}

impl U for Ss {
    fn mm(&self) {
        println!("Inside U - Ss");
    }
}

pub struct Post {
    // To consume the old state, the request_review method needs to take ownership of the state value.
    // This is where the Option in the state field of Post comes in: we call the take method to take
    // the Some value out of the state field and leave a None in its place, because Rust doesn’t let
    // us have unpopulated fields in structs. This lets us move the state value out of Post rather
    // than borrowing it. Then we’ll set the post’s state value to the result of this operation.
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        // We call the as_ref method on the Option because we want a reference to the value inside
        // the Option rather than ownership of the value. Because state is an Option<Box<dyn State>>,
        // when we call as_ref, an Option<&Box<dyn State>> is returned. If we didn’t call as_ref, we
        // would get an error because we can’t move state out of the borrowed &self of the function
        // parameter. (unwrap on option)
        // At this point, when we call content on the &Box<dyn State>, deref coercion will take effect
        // on the & and the Box so the content method will ultimately be called on the type that
        // implements the State trait. That means we need to add content to the State trait definition,
        // and that is where we’ll put the logic for what content to return depending on which state
        // we have.
        self.state.as_ref().unwrap().content(self)
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    // Note that rather than having self, &self, or &mut self as the first parameter of the method,
    // we have self: Box<Self>. This syntax means the method is only valid when called on a Box holding
    // the type. This syntax takes ownership of Box<Self>, invalidating the old state so the state
    // value of the Post can transform into a new state.
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

pub struct TypedPost {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl TypedPost {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

impl DraftPost {
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> TypedPost {
        TypedPost {
            content: self.content,
        }
    }
}
