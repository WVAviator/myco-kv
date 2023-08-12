use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub struct HeapData<T> {
    key: String,
    data: T,
    valid: Mutex<bool>,
}

impl<T> HeapData<T> {
    pub fn new(key: String, data: T) -> Self {
        HeapData {
            key,
            data,
            valid: Mutex::new(true),
        }
    }
}

impl<T> PartialEq for HeapData<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data) && self.key.eq(&other.key)
    }
}

impl<T> Eq for HeapData<T> where T: Eq {}

impl<T> Clone for HeapData<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        HeapData {
            key: self.key.clone(),
            data: self.data.clone(),
            valid: Mutex::new(*self.valid.lock().unwrap()),
        }
    }
}

impl<T> Ord for HeapData<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl<T> PartialOrd for HeapData<T>
where
    T: Ord + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct AtomicHeap<T>
where
    T: Ord + Clone,
{
    heap: BinaryHeap<Arc<HeapData<T>>>,
    map: HashMap<String, Arc<HeapData<T>>>,
}

impl<T> AtomicHeap<T>
where
    T: Ord + Clone,
{
    pub fn new() -> Self {
        AtomicHeap {
            heap: BinaryHeap::new(),
            map: HashMap::new(),
        }
    }

    pub fn push(&mut self, key: String, data: T) {
        let heap_data = Arc::new(HeapData::new(key.clone(), data));

        if let Some(old_heap_data) = self.map.get(&key) {
            *old_heap_data.valid.lock().unwrap() = false;
        }

        self.heap.push(heap_data.clone());
        self.map.insert(key, heap_data);
    }

    pub fn peek(&mut self) -> Option<T> {
        while let Some(heap_data) = self.heap.peek().cloned() {
            if *heap_data.valid.lock().unwrap() {
                return Some(heap_data.data.clone());
            } else {
                self.heap.pop();
            }
        }

        None
    }

    pub fn pop(&mut self) -> Option<T> {
        while let Some(heap_data) = self.heap.pop() {
            if *heap_data.valid.lock().unwrap() {
                self.map.remove(&heap_data.key);
                return Some(heap_data.data.clone());
            }
        }

        None
    }

    pub fn invalidate(&mut self, key: &str) {
        if let Some(heap_data) = self.map.get(key) {
            *heap_data.valid.lock().unwrap() = false;
        }

        self.map.remove(key);
    }

    pub fn clear(&mut self) {
        self.heap.clear();
        self.map.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn invalidates_older_keys() {
        let mut heap = AtomicHeap::new();
        heap.push("a".to_string(), 1);
        heap.push("b".to_string(), 2);
        heap.push("a".to_string(), 3);
        heap.push("b".to_string(), 4);
        heap.push("c".to_string(), 5);

        assert_eq!(heap.peek(), Some(5));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.peek(), Some(4));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.peek(), Some(3));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.peek(), None);
        assert_eq!(heap.pop(), None);
    }
}
