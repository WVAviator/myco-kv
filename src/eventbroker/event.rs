pub enum Event {
  Get { key: String, result: String },
  Set { key: String, value: String },
  Delete { key: String },
}