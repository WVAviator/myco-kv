use super::event::Event;

pub trait Subscriber: Send {
    fn notify(&self, event: &Event);
}
