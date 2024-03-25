
pub trait Messenger {
    fn send(&self, mymsg: &str);
}

pub struct LimitTracker <'a, T: Messenger>
{
    pub messenger: &'a T,
    pub max_size : usize,
    pub current_size : usize,
} 

impl<'a, T> LimitTracker<'a, T> where
T: Messenger {
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger: messenger,
            max_size: max,
            current_size: 0,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.current_size = value;

        let percent_value = self.current_size as f64 / self.max_size as f64;

        if percent_value >= 1.0 {
            self.messenger.send("Error: You are over your quota");
        } else if percent_value >= 0.9 {
            self.messenger.send("Urgent warning: you have used up over 90% of your quota");
        } else if percent_value >= 0.75 {
            self.messenger.send("Warning: you have used up over 75% of your quota");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::*;

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
        fn send(&self, mymsg: &str) {
            self.sent_messages.borrow_mut().push(String::from(mymsg));
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


