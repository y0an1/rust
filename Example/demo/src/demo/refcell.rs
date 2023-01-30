/// RefCell<T> 例子
#[allow(unused)]
mod demo1 {
    pub(super) trait Messenger {
        fn send(&self, msg: &str);
    }

    pub(super) struct LimitTracker<'a, T: 'a + Messenger> {
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
            if (percentage_of_max >= 1.0) {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!")
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use std::cell::RefCell;

        use super::*;

        struct MockMessenger {
            // sent_messages: Vec<String>,
            sent_messages: RefCell<Vec<String>>,
        }

        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger {
                    // sent_messages: vec![],
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }

        impl Messenger for MockMessenger {
            // fn send(&mut self, msg: &str) {
            fn send(&self, msg: &str) {
                //62 |             fn send(&mut self, msg: &str) {
                //   |                     ^^^^^^^^^
                //   |                     |
                //   |                     types differ in mutability
                //   |                     help: change the self-receiver type to match the trait: `self: &MockMessenger`
                // self.sent_messages.push(String::from(msg));
                // borrow_mut -> 将借用临时更改外可变的
                self.sent_messages.borrow_mut().push(String::from(msg));
            }
        }

        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

            limit_tracker.set_value(80);
            // assert_eq!(mock_messenger.sent_messages.len(), 1);
            assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        }
    }

    pub(super) fn main() {}
}

/// Rc<T> 和 RefCell<T> 结合使用
#[allow(unused)]
mod demo2{
    use std::rc::Rc;
    use std::cell::RefCell;
    use List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil
    }

    pub fn main() {
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}


pub fn main() {
    // demo2::main();
}
