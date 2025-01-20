#![allow(dead_code)]

use core::fmt;
use std::{
    borrow::Borrow, 
    fmt::Debug, 
    rc::Rc
};

use crate::list::node::{Node, NodeRef};

#[derive(Clone, Debug, PartialEq)]
pub struct List<T> {
    pub size: usize,
    head: Option<NodeRef<T>>,
    tail: Option<NodeRef<T>>,
}

//Basic Operations for the Lists
pub trait BaseOperations<T> {
    fn empty() -> List<T>;
    fn new(val: T) -> List<T>;

    fn get_head(&self) -> Option<NodeRef<T>>;
    fn get_tail(&self) -> Option<NodeRef<T>>;

    fn head(&self) -> Option<T> where T: Clone;
    fn tail(&self) -> Option<T> where T: Clone;

    fn get_value(&self, pos:usize) -> Option<T> where T:Clone;
    fn find_value(&self, value:T) -> Vec<usize> where T:PartialEq;

    fn append(&mut self, val: T);
    fn detach(&mut self) -> Option<T> where T: Clone;

    fn push(&mut self, val: T);
    fn pop(&mut self) -> Option<T> where T: Clone;

    fn insert(&mut self, pos: usize, val: T) where T: Clone;
    fn delete(&mut self, pos: usize) -> Option<T> where T: Clone;

    fn sort(&mut self);
}

impl<T> BaseOperations<T> for List<T> {

    /// Create an empty Doubly Linked List
    /// 
    /// #Example
    /// ```
    /// let list:List<T> = IndeList::empty();
    /// ```
    fn empty() -> Self {
        Self { size: 0, head: None, tail: None }
    }

    /// Create a Doubly Linked List with a head node
    /// 
    /// #Example
    /// ```
    /// let list:List<&str> = IndeList::new("This is a head");
    /// ```
    fn new(val: T) -> Self {
        Self { size: 1, 
            head: Some(Node::to_ref(Node::new(val))),
            tail: None
        }
    }

    /// Get tail
    fn get_tail(&self) -> Option<NodeRef<T>> {
        self.tail.clone()
    }

    /// Get head
    fn get_head(&self) -> Option<NodeRef<T>> {
        self.head.clone()
    }

    /// Get the value contained in the head node
    /// 
    /// #Example
    /// ```
    /// let list = List::new(1);
    ///
    /// assert_eq!(list.head(), Some(1));
    /// ```
    fn head(&self) -> Option<T> where T: Clone {
        if let Some(head) = &self.head {
            return Some( head.as_ref().borrow().val.clone() );
        }
        None
    }

    /// Get the value contained in the tail node
    /// 
    /// #Example
    /// ```
    /// let list = List::new(1);
    /// list.append(2);
    ///
    /// assert_eq!(list.tail(), Some(2));
    /// ```
    fn tail(&self) -> Option<T> where T: Clone {
        if let Some(tail) = &self.tail {
            return Some( tail.as_ref().borrow().val.clone() )
        }
        None
    }

    /// Get the value in the specified position. Like in array start count at 0;
    /// 
    /// #Example
    /// ```
    /// let list:List<i32> = vec![6,8,10,12,14].to_list();
    /// 
    /// let value_at_position = list.get_value(2);
    /// 
    /// assert_eq!(value_at_position, Some(8));
    /// ```
    fn get_value(&self, pos:usize) -> Option<T> where T:Clone {
        if pos > self.size || pos == 0 { return None; }

        let Some(head) = &self.head.clone() else {
            return None;
        };
        if pos == 0 { return Some( head.as_ref().borrow().val.clone() ); }

        let Some(tail) = &self.tail.clone() else {
            if pos == 0 {
                return Some( head.as_ref().borrow().val.clone() );
            } else { return None; }
        };
        if pos == self.size - 1 { return Some( tail.as_ref().borrow().val.clone() );}
        
        let middle:usize = ((self.size - 1) as f32 / 2.0) as usize;

        let mut count:usize = 0;
        if pos > middle { // start from tail in reverse
            count = self.size - 1;

            let mut cur = tail.as_ref().borrow().prev.clone();

            while let Some(node) = cur {
                count -= 1;

                if count < middle { return None;}
                if pos == count {
                    return Some( node.as_ref().borrow().val.clone() );
                }
                cur = node.as_ref().borrow().prev.clone();
            }

        } else { // start from head
            let mut cur = head.as_ref().borrow().next.clone();

            while let Some(node) = cur {
                count += 1;

                if count > middle { return None; }
                if pos == count {
                    return Some( node.as_ref().borrow().val.clone() );
                }
                cur = node.as_ref().borrow().next.clone();
            }
        }

        None
    }

    /// Find all the specified value in the list return a vector of positions
    fn find_value(&self, value:T) -> Vec<usize> 
        where T: PartialEq 
    {
        let mut res:Vec<usize> = Vec::new();

        let Some(head) = self.head.clone() else {
            return res;
        };

        let mut cur = head.as_ref().borrow().next.clone();

        let mut count:usize = 0;
        while let Some(node) = cur {
            count += 1;
            if value == node.as_ref().borrow().val {
                res.push(count);
            }
            
            cur = node.as_ref().borrow().next.clone(); 
        }

        res
    }

    /// Append value to the end of the list creating a new tail node, if list is empty creates a head node.
    /// 
    /// #Example
    /// ```
    /// let mut list = List::new(1);
    ///
    /// assert_eq!(list.head(), Some(1));
    /// assert_eq!(list.tail(), None);
    /// assert_eq!(list.size(), 1);
    /// 
    /// list.append(2);
    /// list.append(10);
    ///
    /// assert_eq!(list.head(), Some(1));
    /// assert_eq!(list.tail(), Some(10));
    /// assert_eq!(list.size(), 3);
    /// ```
    fn append(&mut self, val: T) {
        if let Some(head) = self.head.clone() {
            let mut new_node:Node<T> = Node::new(val);
            
            if let Some(tail) = self.tail.clone() {
                new_node.prev = Some(tail.clone());

                let new_node_ref:NodeRef<T> = Node::to_ref(new_node);

                tail.as_ref().borrow_mut().next = Some(new_node_ref.clone());
                self.tail = Some(new_node_ref);
            } else { // head but no tail
                new_node.prev = Some(head.clone());

                let new_node_ref:NodeRef<T> = Node::to_ref(new_node);

                head.as_ref().borrow_mut().next = Some(new_node_ref.clone());
                self.tail = Some(new_node_ref);
            }
        } else { // list is empty 
            let new_node = Node::new(val);
            self.head = Some(Node::to_ref(new_node));
        }
        self.size += 1;
    }

    /// Remove final node of the list.
    /// 
    /// #Example
    /// ```
    /// let mut list = List::new(2);
    /// 
    /// list.append(4);
    /// list.append(6);
    ///
    /// assert_eq!(list.head(), Some(2));
    /// assert_eq!(list.tail(), Some(6));
    /// assert_eq!(list.size(), 3);
    /// 
    /// list.detach();
    /// 
    /// assert_eq!(list.head(), Some(2));
    /// assert_eq!(list.tail(), Some(4));
    /// assert_eq!(list.size(), 2);
    /// ```
    fn detach(&mut self) -> Option<T> where T: Clone {
        if let Some(head) = self.head.clone() {
            if let Some(tail) = self.tail.clone() {
                let tail_val = tail.as_ref().borrow().val.clone();
                if self.size == 2 { // head and tail
                    self.tail = None;
                    head.as_ref().borrow_mut().next = None;
                }
                if self.size > 2 {
                    let prev = tail.as_ref().borrow_mut().clone().prev.unwrap();
                    
                    self.tail = Some(prev.clone());
                    tail.as_ref().borrow_mut().clone().prev.unwrap().as_ref().borrow_mut().next = None;
                }
                self.size -= 1;
                return Some(tail_val)
            } else { // head but no tail
                self.head = None;
                self.size -= 1;
                return Some( head.as_ref().borrow().val.clone() );
            }
        } else { // list is empty
            return None
        }
    }

    /// Push value to the biginning of the list
    /// 
    /// #Example
    /// ```
    /// let mut list = List::new(10);
    /// 
    /// assert_eq!(list.head(), Some(10));
    /// assert_eq!(list.tail(), None);
    /// assert_eq!(list.size(), 1);
    /// 
    /// list.push(12);
    /// 
    /// assert_eq!(list.head(), Some(12));
    /// assert_eq!(list.tail(), Some(10));
    /// assert_eq!(list.size(), 2);
    /// ```
    fn push(&mut self, val: T) {
        if let Some(_) = self.head.clone() {
            let mut new_head:Node<T> = Node::new(val);

            if let Some(_) = self.tail.clone() {
                new_head.next = self.head.clone();

                let new_head_ref:NodeRef<T> = Node::to_ref(new_head);
                
                self.head.as_ref().borrow().clone().unwrap().as_ref().borrow_mut().prev = Some(new_head_ref.clone());

                self.head = Some(new_head_ref);

            } else { // head with no tail 
                new_head.next = self.head.clone();

                let new_head_ref:NodeRef<T> = Node::to_ref(new_head);

                self.head.as_ref().borrow().clone().unwrap().as_ref().borrow_mut().prev = Some(new_head_ref.clone());

                self.tail = self.head.clone();
                self.head = Some(new_head_ref);

            }
        } else { // list is empty 
            self.head = Some( Node::to_ref(Node::new(val)) );
        }
        self.size += 1;
    }

    /// Remove the head node from the list
    /// 
    /// #Example
    /// ```
    /// let mut list:List<&str> = List::new("Head");
    /// 
    /// list.append("This is a node");
    /// list.append("This is another node");
    ///
    /// assert_eq!(list.head(), Some("Head"));
    /// assert_eq!(list.tail(), Some("This is another node"));
    /// assert_eq!(list.size(), 3);
    /// 
    /// list.pop();
    /// 
    /// assert_eq!(list.head(), Some("This is a node"));
    /// assert_eq!(list.tail(), Some("This is another node"));
    /// assert_eq!(list.size(), 2);
    /// 
    /// ```
    fn pop(&mut self) -> Option<T> where T: Clone {
        if let Some(head) = self.head.clone() {
            let head_val:T = head.as_ref().borrow().val.clone();

            if let Some(_) = self.tail.clone() {
                if self.size == 2 { // head and tail case
                    self.head = self.tail.clone();
                    self.tail = None;
                }
                if self.size > 2 { // head tail and other nodes
                    self.head = head.as_ref().borrow().next.clone();
                }

            } else { // head but no tail
                self.head = None;
            }

            self.size -= 1;
            return Some(head_val);
        } 
        else { return None; } // list is empty        
    }
    
    /// Insert a value at the specified position. Start counting at 1.
    /// 
    /// #Example 
    /// ```
    /// let mut list:List<i32> = vec![1,2,3,4,5].to_list();
    ///
    /// list.insert(2, 69);
    /// 
    /// assert_eq!()
    /// 
    /// ```
    fn insert(&mut self, pos: usize, val: T) where T: Clone {
        if pos > self.size { return; }
        if pos == self.size { Self::append(self, val.clone()) }
       
        let new_node: NodeRef<T> = Node::to_ref(Node::new(val.clone()));

        if let Some(_) = self.head.clone() {
            if let Some(_) = self.tail.clone() {
                let new_node:NodeRef<T> = Node::to_ref(Node::new(val));
                
                let mut current:Option<NodeRef<T>> = self.head.clone();

                let mut i:usize = 0;
                while let Some(node) = current {
                    if i == pos - 1 {
                        new_node.as_ref().borrow_mut().prev = node.as_ref().borrow().prev.clone();
                        new_node.as_ref().borrow_mut().next = Some(node.clone());
                        
                        node.as_ref().borrow_mut().prev.clone().unwrap().as_ref().borrow_mut().next = Some(new_node.clone());
                        node.as_ref().borrow_mut().prev = Some(new_node.clone());
                        break;
                    }

                    i += 1;
                    current = node.as_ref().borrow().next.clone();
                }

            } else { // Head but no tail
                if pos == 0 { Self::push(self, val); }
                else { Self::append(self, val) }
            }
        } else { // Empty list
            self.head = Some(new_node);
        }
        self.size += 1;
    }
    
    /// Delete value at the specified position. Start counting at 1.
    fn delete(&mut self, pos: usize) -> Option<T> where T: Clone {
        if pos > self.size - 1 { return None; }
        
        if let Some(head) = self.head.clone() {
            if let Some(_) = self.tail.clone() {
                if pos == self.size - 1 { return Self::detach(self); }
                if pos == 0 { return Self::pop(self); }
                
                let mut current:Option<NodeRef<T>> = self.head.clone();

                let mut i:usize = 0;
                while let Some(node) = current {
                    if i == pos {
                        node.as_ref().borrow().prev.clone().unwrap().as_ref().borrow_mut().next = node.as_ref().borrow().next.clone();
                        node.as_ref().borrow().next.clone().unwrap().as_ref().borrow_mut().prev = node.as_ref().borrow().prev.clone();
                        return Some(node.as_ref().borrow().val.clone());
                    }
                    i += 1;
                    current = node.as_ref().borrow().next.clone();
                }
                self.size -= 1;

                return None;
            } else { // Head but no tail
                let val = head.as_ref().borrow().val.clone();
                self.head = None;
                self.size -= 1;

                return Some(val);
            }
        } else { // Empty list
            return None;
        }
    }
    
    fn sort(&mut self) {
        todo!()
    }

}

impl<T> List<T> {
    pub fn to_vec(self) -> Vec<T> 
        where T: Clone
    {
        let mut res:Vec<T> = Vec::with_capacity(self.size);
    
        let mut cur:Option<NodeRef<T>> = self.head;
    
        while let Some(node) = cur {
            res.push(node.as_ref().borrow().val.clone());

            cur = node.as_ref().borrow().next.clone();
        }

        res
    }

    pub fn close(&mut self) {
        let Some(_) = self.head.clone() else {
            return; 
        };
        let Some(_) = self.tail.clone() else {
            return; 
        };

        self.head.clone().unwrap().as_ref().borrow_mut().prev = self.tail.clone();
        self.tail.clone().unwrap().as_ref().borrow_mut().next = self.head.clone();
    }

    pub fn open(&mut self) {
        let Some(_) = self.head.clone() else {
            return; 
        };
        let Some(_) = self.tail.clone() else {
            return; 
        };

        self.head.clone().unwrap().as_ref().borrow_mut().prev = None;
        self.tail.clone().unwrap().as_ref().borrow_mut().next = None;
    }

    pub fn split(mut self) -> (List<T>, List<T>) 
        where T: Debug 
    {
        let Some(head) = self.head.clone() else {
            return (self, List::empty());
        };
        let Some(_) = self.head.clone() else {
            return (self, List::empty());
        };

        let is_closed:bool = if let Some(_) = head.as_ref().borrow().prev {
            true
        } else { false };

        let mut new_list:List<T> = List::empty();

        let mut cur_next = Some(head.clone());
        let mut cur = Some(head.clone());
        loop {
            if let Some(node) = cur_next.clone() {
                let Some(ref node_next) = node.as_ref().borrow().next else {
                    break;
                };
                if Rc::ptr_eq(&head, &node_next) { break; }

                let Some(ref node_ne_next) = node_next.as_ref().borrow().next else {
                    break;
                };
                if Rc::ptr_eq(&head, &node_ne_next) { break; }

                cur_next = Some(node_ne_next.clone());
                cur = cur.unwrap().as_ref().borrow().next.clone();
            } else { break; }
        }
        
        new_list.tail = self.tail.clone();
        new_list.head = cur.clone().unwrap().as_ref().borrow().next.clone();
        if is_closed {
            Self::close(&mut new_list);
        } else { Self::open(&mut new_list); }

        self.tail = cur.clone();
        self.tail.as_ref().unwrap().as_ref().borrow_mut().next = None;
        if is_closed {
            Self::close(&mut self);
        }else { Self::open(&mut self); }

        (self, new_list)
    }

    pub fn merge(mut self, list:List<T>) -> List<T> {
        if list.head.is_none() && self.head.is_none() { return self; }
        if list.head.is_none() { return self; }
        if self.head.is_none() { return list; }

        self.tail.clone().unwrap().as_ref().borrow_mut().next = Some(list.head.unwrap());
        self.tail.unwrap().as_ref().borrow().next.clone().unwrap().as_ref().borrow_mut().prev = self.tail.clone();
        
        self.tail = list.tail;

        self
    }

    pub fn count(&self, val: T) -> usize 
        where T: PartialEq 
    {
        let mut count: usize = 0;

        let mut current:Option<NodeRef<T>> = self.head.clone();
        while let Some(node) = current {
            if node.as_ref().borrow().next == self.head { break; }

            if node.as_ref().borrow().val == val { count += 1; }

            current = node.as_ref().borrow().next.clone();            
        }

        count
    }

    pub fn remove_duplicates(&mut self)
        where T: PartialEq
    {
        todo!();
    }
}

pub trait FindSmallest<T> {
    fn find_smallest(&self, k:usize) -> List<T>;
}

impl<T> FindSmallest<T> for Vec<T> 
    where T: Clone + PartialOrd<T> + std::fmt::Debug
{
    fn find_smallest(&self, k:usize) -> List<T> {
        let mut list:List<T> = List::new(self[0].clone());

        if self[1] < list.head().unwrap() {
            list.push(self[1].clone());
        } else { list.append(self[1].clone()); }

        for num in self.iter().skip(2) {
            if num > &list.tail().unwrap() { 
                if list.size >= k { continue; }
                
                list.append(num.clone());
            }
            else if num <= &list.head().unwrap() {
                if list.size >= k { list.detach(); }
                
                list.push(num.clone());
            }
            else if num >= &list.head().unwrap() || num < &list.tail().unwrap() {
                let mut cur:Option<NodeRef<T>> = list.get_head();

                while let Some(node) = cur {
                    if &node.as_ref().borrow().val >= num {
                        let new_node:NodeRef<T> = Node::new(num.clone()).to_ref();

                        new_node.as_ref().borrow_mut().prev = node.as_ref().borrow().prev.clone();
                        new_node.as_ref().borrow_mut().next = Some(node.clone());

                        node.as_ref().borrow_mut().prev.clone().unwrap().as_ref().borrow_mut().next = Some(new_node.clone());
                        node.as_ref().borrow_mut().prev = Some(new_node.clone());
                        list.size += 1;
                        break;
                    }
                    cur = node.as_ref().borrow().next.clone();
                }
                if list.size >= k { list.detach(); }
            }
            // println!("{}", list);
        }
        list
    }
}  

impl<T> fmt::Display for List<T> 
    where T: Debug + PartialEq 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Some(head) = self.head.clone() else {
            return write!(f, "[]");
        };
        if let Some(head_prev) = head.clone().as_ref().borrow().prev.clone() {
            print!("T[{:?}] <=> ", head_prev.as_ref().borrow().val);
        } else { print!("[{}]: None <=> ", self.size)}

        let Some(_) = &self.tail else {
            return write!(f, "None <=> [{:?}] <=> None", head.clone().as_ref().borrow().val);
        };

        let mut current= head.as_ref().borrow().next.clone();
        print!("H[{:?}] <=> ", head.as_ref().borrow().val);

        loop {
            if let Some(node) = current.clone() {
                print!("[{:?}] <=> ", node.as_ref().borrow().val);
                current = node.as_ref().borrow().next.clone();

                let Some(current_next) = current.clone() else {
                    println!("None");
                    break;   
                };  
                if Rc::ptr_eq(&head, &current_next) { break; }
            } else { 
                println!("None");
                break;
            }
        }
        write!(f, "")
    }
}