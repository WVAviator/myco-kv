use super::event::Event;

pub trait Subscriber {
    fn notify(&self, event: &Event);
}
