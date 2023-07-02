use mockall::automock;

use super::event::Event;

#[cfg_attr(test, automock)]
pub trait Subscriber: Send {
    fn notify(&mut self, event: &Event);
}
