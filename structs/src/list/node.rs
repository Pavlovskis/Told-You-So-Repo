#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

pub type NodeRef<T> = Rc<RefCell<Node<T>>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Node<T> {
    pub val: T,
    pub next: Option<NodeRef<T>>,
    pub prev: Option<NodeRef<T>>,
}

impl<T> Node<T> {
    pub fn to_ref(self) -> NodeRef<T> {
        Rc::new(RefCell::new(self))
    }
    
    pub fn new(val: T) -> Self {
        Self { val, next: None, prev: None }
    }

    pub fn next(&mut self, next: NodeRef<T>) {
        self.next = Some(next);
    }

    pub fn prev(&mut self, prev: NodeRef<T>) {
        self.prev = Some(prev);
    }
}

