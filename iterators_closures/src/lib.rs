pub fn iterators_demo() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // We didn’t need to make v1_iter mutable when we used a for loop because the loop took ownership
    // of v1_iter and made it mutable behind the scenes.
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // Also note that the values we get from the calls to next are immutable references to the values
    // in the vector. The iter method produces an iterator over immutable references. If we want to
    // create an iterator that takes ownership of v1 and returns owned values, we can call into_iter
    // instead of iter. Similarly, if we want to iterate over mutable references, we can call iter_mut
    // instead of iter.

    // Other methods defined on the Iterator trait, known as iterator adaptors, allow you to change
    // iterators into different kinds of iterators. You can chain multiple calls to iterator adaptors
    // to perform complex actions in a readable way. But because all iterators are lazy, you have to
    // call one of the consuming adaptor methods to get results from calls to iterator adaptors.
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("{:?}", v2);
    assert_eq!(v2, vec![2, 3, 4]);
}

pub trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}


#[derive(PartialEq, Debug)]
pub struct Shoe {
    pub size: u32,
    pub style: String,
}

pub fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

//
//#[cfg(test)]
//mod tests {
//    use super::*;
//

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // calling next changes internal state of the iterator so it needs to be mutable
    // We didn’t need to make v1_iter mutable when we used a for loop because the loop took ownership
    // of v1_iter and made it mutable behind the scenes.
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}


// Methods that call next are called consuming adaptors, because calling them uses up the iterator.
// One example is the sum method, which takes ownership of the iterator and iterates through the
// items by repeatedly calling next, thus consuming the iterator.
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    // We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the
    // iterator we call it on.
}

#[test]
fn filters_by_size() {
    //use iterators_closures::{Shoe,shoes_in_my_size};
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}


#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);
}


#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

// For a more comprehensive benchmark, you should check using various texts of various sizes as the
// contents, different words and words of different lengths as the query, and all kinds of other
// variations. The point is this: iterators, although a high-level abstraction, get compiled down to
// roughly the same code as if you’d written the lower-level code yourself. Iterators are one of
// Rust’s zero-cost abstractions, by which we mean using the abstraction imposes no additional runtime
// overhead. This is analogous to how Bjarne Stroustrup, the original designer and implementor of C++,
// defines zero-overhead in “Foundations of C++” (2012):
// In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don’t
// pay for. And further: What you do use, you couldn’t hand code any better.

// Audio Decoder example: There’s no loop at all corresponding to the iteration over the values in
// coefficients: Rust knows that there are 12 iterations, so it “unrolls” the loop. Unrolling is an
// optimization that removes the overhead of the loop controlling code and instead generates repetitive
// code for each iteration of the loop.
// All of the coefficients get stored in registers, which means accessing the values is very fast.
// There are no bounds checks on the array access at runtime. All these optimizations that Rust is
// able to apply make the resulting code extremely efficient.