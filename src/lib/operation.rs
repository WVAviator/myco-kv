#[derive(Debug)]
pub enum Operation {
    Get(String),
    Put(String, String),
    Delete(String),
}
