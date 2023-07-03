#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    Get { key: String, result: String },
    Put { key: String, value: String },
    Delete { key: String },
}
