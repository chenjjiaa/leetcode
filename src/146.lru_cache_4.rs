#![allow(unused)]

use std::{
    collections::{
        HashMap,
        hash_map::RandomState
    },
    hash::BuildHasher,
    usize,
};
use usize::MAX;

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct Node<T>
where
    T: Clone + Default,
{
    key: i32,
    value: T,
    pre: usize,
    next: usize,
}

#[derive(Default, Debug, Clone)]
pub struct LRUCache<T>
where
    T: Clone + Default,
{
    map: HashMap<i32, usize, RandomState>,
    vec: Vec<Node<T>>,
    head: usize,
    tail: usize,

    /// free slot.
    free: usize,
    capacity: usize,
}

impl<T> LRUCache<T>
where
    T: Clone + Default,
{
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            map: HashMap::with_capacity_and_hasher(capacity, RandomState::new()),
            vec: Vec::with_capacity(capacity),
            head: MAX,
            tail: MAX,
            free: MAX,
            capacity: capacity,
        }
    }

    // 分配一个新的节点。从 vec 中找到一个空位置 (free slot)，放进一个新的 Node
    // 返回新节点的下标
    fn alloc(&mut self, key: i32, value: T) -> usize {
        if self.free != MAX {
            self.vec[self.free] = Node {
                key: key,
                value: value,
                pre: MAX,
                next: MAX,
            };
            self.free
        } else {
            let len = self.vec.len();
            self.vec.push(Node {
                key,
                value,
                pre: MAX,
                next: MAX,
            });
            len
        }
    }

    // 把下标为 idx 的节点从 “链表” 中摘除
    fn detech(&mut self, idx: usize) {
        let (pre, next) = {
            let p = self.vec[idx].pre;
            let n = self.vec[idx].next;
            (p, n)
        };

        if pre == MAX {
            self.head = next;
        } else {
            self.vec[pre].next = next;
        }

        if next == MAX {
            self.tail = pre;
        } else {
            self.vec[next].pre = pre;
        }

        self.vec[idx].pre = MAX;
        self.vec[idx].next = MAX;
    }

    // 把下标为 idx 的节点添加到 “链表” 末尾
    fn attach_tail(&mut self, idx: usize) {
        self.vec[idx].next = MAX;
        self.vec[idx].pre = self.tail;

        if self.tail != MAX {
            self.vec[self.tail].next = idx;
        } else {
            self.head = idx;
        }

        self.tail = idx;
    }

    pub fn put(&mut self, key: i32, value: T) {
        if let Some(&idx) = self.map.get(&key) {
            self.vec[idx].value = value;
            self.detech(idx);
            self.attach_tail(idx);
            return;
        }

        if self.capacity == self.vec.len() {
            let old_idx = self.head;
            let old_key = self.vec[old_idx].key;

            self.detech(old_idx);
            self.free = old_idx;
            self.map.remove(&old_key);
        }

        let idx = self.alloc(key, value);
        self.attach_tail(idx);
        self.map.insert(key, idx);
    }

    pub fn get(&mut self, key: i32) -> T {
        if let Some(&idx) = self.map.get(&key) {
            self.detech(idx);
            self.attach_tail(idx);
            return self.vec[idx].value.clone();
        }
        T::default()
    }
}
