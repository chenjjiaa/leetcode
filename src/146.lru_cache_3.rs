#![allow(unused)]

use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct Node<T>
where
    T: Clone + Default,
{
    key: i32,
    value: T,
    pre: Option<usize>,
    next: Option<usize>,
}

#[derive(Default, Debug, Clone)]
pub struct LRUCache<T>
where
    T: Clone + Default,
{
    map: HashMap<i32, usize>,
    vec: Vec<Node<T>>,
    head: Option<usize>,
    tail: Option<usize>,

    /// free slot.
    free: Option<usize>,
    capacity: usize,
}

impl<T> LRUCache<T>
where
    T: Clone + Default,
{
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            map: HashMap::new(),
            vec: Vec::new(),
            head: None,
            tail: None,
            free: None,
            capacity: capacity,
        }
    }

    // 分配一个新的节点。从 vec 中找到一个空位置 (free slot)，放进一个新的 Node
    // 返回新节点的下标
    fn alloc(&mut self, key: i32, value: T) -> usize {
        if let Some(idx) = self.free.take() {
            self.vec[idx] = Node {
                key: key,
                value: value,
                pre: None,
                next: None,
            };
            idx
        } else {
            let len = self.vec.len();
            self.vec.push(Node {
                key,
                value,
                pre: None,
                next: None,
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

        if let Some(p) = pre {
            self.vec[p].next = next;
        } else {
            self.head = next;
        };

        if let Some(n) = next {
            self.vec[n].pre = pre;
        } else {
            self.tail = pre;
        }

        self.vec[idx].pre = None;
        self.vec[idx].next = None;
    }

    // 把下标为 idx 的节点添加到 “链表” 末尾
    fn attach_tail(&mut self, idx: usize) {
        self.vec[idx].next = None;
        self.vec[idx].pre = self.tail;

        if let Some(t) = self.tail {
            self.vec[t].next = Some(idx);
        } else {
            self.head = Some(idx);
        }
        self.tail = Some(idx);
    }

    pub fn put(&mut self, key: i32, value: T) {
        if let Some(&idx) = self.map.get(&key) {
            self.vec[idx].value = value;
            self.detech(idx);
            self.attach_tail(idx);
            return;
        }

        if self.capacity == self.vec.len() {
            let old_idx = self.head.unwrap();
            let old_key = self.vec[old_idx].key;

            self.detech(old_idx);
            self.free = Some(old_idx);
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
