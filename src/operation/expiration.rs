use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expiration {
    pub key: String,
    pub timestamp: i64,
}

impl Expiration {
    pub fn new(key: String, timestamp: i64) -> Self {
        Expiration { key, timestamp }
    }
}

impl Ord for Expiration {
    fn cmp(&self, other: &Self) -> Ordering {
        other.timestamp.cmp(&self.timestamp)
    }
}

impl PartialOrd for Expiration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn orders_in_reverse() {
        let a = Expiration::new("a".to_string(), 1);
        let b = Expiration::new("b".to_string(), 2);
        assert!(b < a);
    }
}
