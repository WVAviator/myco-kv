use super::event::Event;

pub trait Subscriber: Send {
    fn notify(&mut self, event: &Event);
}
