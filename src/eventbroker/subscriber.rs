use super::event::Event;

#[cfg_attr(test, mockall::automock)]
pub trait Subscriber: Send {
    fn notify(&mut self, event: &Event);
}
