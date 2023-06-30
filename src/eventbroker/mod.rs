use self::{event::Event, subscriber::Subscriber};

pub mod event;
pub mod subscriber;

pub struct EventBroker {
    subscribers: Vec<Box<dyn Subscriber>>,
}

impl EventBroker {
    pub fn new() -> Self {
        EventBroker {
            subscribers: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, subscriber: Box<dyn Subscriber>) {
        self.subscribers.push(subscriber);
    }

    pub fn publish(&mut self, event: &Event) {
        for subscriber in &mut self.subscribers {
            subscriber.notify(event);
        }
    }
}
