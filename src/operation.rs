#[derive(Debug, PartialEq)]
pub enum Operation {
    Get(String),
    Put(String, String),
    Delete(String),
}
