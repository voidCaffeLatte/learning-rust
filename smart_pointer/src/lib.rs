use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[cfg(test)]
mod tests
{
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
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

pub trait Messenger
{
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger>
{
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T>
    {
        LimitTracker
        {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 
        {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } 
        else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 
        {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } 
        else if percentage_of_max >= 1.0 
        {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

#[derive(Debug)]
pub enum List3 {
    Cons(std::rc::Rc<std::cell::RefCell<i32>>, std::rc::Rc<List3>),
    Nil,
}

#[derive(Debug)]
pub enum List4 {
    Cons(i32, RefCell<Rc<List4>>),
    Nil,
}

impl List4 {
    pub fn tail(&self) -> Option<&RefCell<Rc<List4>>> {
        match *self 
        {
            List4::Cons(_, ref item) => Some(item),
            List4::Nil => None,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}
