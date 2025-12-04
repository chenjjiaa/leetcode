#![allow(unused)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug, Default)]
struct Node<T>
where
    T: Clone + Default,
{
    value: T,
    pre: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T>
where
    T: Clone + Default,
{
    pub fn new(value: T) -> Self {
        Node {
            value,
            pre: None,
            next: None,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct LRUCache<T>
where
    T: Clone + Default,
{
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    map: HashMap<i32, Option<Rc<RefCell<Node<T>>>>>,
    capacity: usize,
}

impl<T> LRUCache<T>
where
    T: Clone + Default,
{
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            head: None,
            tail: None,
            map: HashMap::new(),
            capacity: capacity,
        }
    }

    pub fn put(&mut self, value: T) {}

    pub fn get(&mut self, key: i32) -> T {
        if !self.map.contains_key(&key) {
            return T::default();
        }

        let node_ref: Option<&Rc<RefCell<Node<T>>>> = self.map.get(&key).unwrap().as_ref();
        if let Some(v) = node_ref {}

        // let mut wrapped_node;
        // if let Some(v) = node_ref {
        //     self.remove(v.borrow().as_ref().unwrap());
        //     self.move_to_tail(v.borrow().as_ref().unwrap());
        //     wrapped_node = Rc::clone(&v);
        //     return wrapped_node.borrow().as_ref().unwrap().value.clone();
        // }

        T::default()
    }

    fn remove(&mut self, mut node: &Node<T>) {}

    fn remove_head(&mut self, mut node: &Node<T>) {}

    fn move_to_tail(&mut self, mut node: &Node<T>) {}
}
