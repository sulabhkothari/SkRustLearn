pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            //let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));

            // Notice that the code panicked with the message already borrowed: BorrowMutError. This
            // is how RefCell<T> handles violations of the borrowing rules at runtime.
            //two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// The borrow method returns the smart pointer type Ref<T>, and borrow_mut returns the smart pointer
// type RefMut<T>. Both types implement Deref, so we can treat them like regular references.

// The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are currently active.
// Every time we call borrow, the RefCell<T> increases its count of how many immutable borrows are
// active. When a Ref<T> value goes out of scope, the count of immutable borrows goes down by one.
// Just like the compile-time borrowing rules, RefCell<T> lets us have many immutable borrows or one
// mutable borrow at any point in time.

// Your code would incur a small runtime performance penalty as a result of keeping track of the
// borrows at runtime rather than compile time. However, using RefCell<T> makes it possible to write
// a mock object that can modify itself to keep track of the messages it has seen while you’re using
// it in a context where only immutable values are allowed. You can use RefCell<T> despite its
// trade-offs to get more functionality than regular references provide.
