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

#[cfg(test)]
mod test {

    use super::subscriber::MockSubscriber;
    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    #[test]
    fn test_can_subscribe_and_be_notified() {
        let event = Event::Get {
            key: "test".to_string(),
            result: "test".to_string(),
        };
        let mut mock_subscriber = MockSubscriber::new();
        mock_subscriber
            .expect_notify()
            .with(eq(event.clone()))
            .times(1)
            .return_const(());

        let mut event_broker = EventBroker::new();
        event_broker.subscribe(Box::new(mock_subscriber));
        event_broker.publish(&event);
    }

    #[test]
    fn test_can_notify_multiple_subscribers() {
        let event = Event::Get {
            key: "test".to_string(),
            result: "test".to_string(),
        };
        let mut mock_subscriber_1 = MockSubscriber::new();
        mock_subscriber_1
            .expect_notify()
            .with(eq(event.clone()))
            .times(1)
            .return_const(());

        let mut mock_subscriber_2 = MockSubscriber::new();
        mock_subscriber_2
            .expect_notify()
            .with(eq(event.clone()))
            .times(1)
            .return_const(());

        let mut event_broker = EventBroker::new();
        event_broker.subscribe(Box::new(mock_subscriber_1));
        event_broker.subscribe(Box::new(mock_subscriber_2));
        event_broker.publish(&event);
    }
}
