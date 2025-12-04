#![allow(unused)]

use std::collections::HashMap;
use std::collections::VecDeque;

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
struct LRUCache {
    map: HashMap<i32, i32>,
    queue: VecDeque<i32>,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            map: HashMap::new(),
            queue: VecDeque::with_capacity(capacity as usize),
            capacity: capacity as usize,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if !self.map.contains_key(&key) {
            return -1;
        }

        if let Some(pos) = self.queue.iter().position(|&x| x == key) {
            self.queue.remove(pos);
            self.queue.push_back(key);
        }

        self.map.get(&key).unwrap_or(&-1).clone()
    }

    fn put(&mut self, key: i32, value: i32) {
        if !self.map.contains_key(&key) {
            if self.capacity > self.queue.len() {
                self.queue.push_back(key);
                self.map.insert(key, value);
            } else {
                if let Some(oldest) = self.queue.pop_front() {
                    self.map.remove(&oldest);
                }
                self.queue.push_back(key);
                self.map.insert(key, value);
            }
        } else {
            self.map.insert(key, value);
            if let Some(pos) = self.queue.iter().position(|&x| x == key) {
                self.queue.remove(pos);
            }
            self.queue.push_back(key);
        }
    }
}
