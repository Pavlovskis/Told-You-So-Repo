use core::fmt;
use std::{fmt::Debug, usize::MAX};

pub struct MaxHeap<T> {
    pub max_size:usize,
    vals:Vec<T>,
    min:Option<T> 
}

impl<T> MaxHeap<T> 
    where T: PartialOrd + Clone
{
    pub fn new() -> Self {
        Self { vals: Vec::new(), min: None, max_size: MAX }
    }

    /// Create new heap with fixed size
    pub fn with_capacity(cap:usize) -> Self {
        Self { 
            max_size: cap, 
            vals: Vec::with_capacity(cap), 
            min: None
        }
    }

    /// Get the unordered values from the heap
    pub fn to_vec(self) -> Vec<T> {
        self.vals
    }

    pub fn size(&self) -> usize {
        self.vals.len()
    }

    /// Get the root 
    pub fn get_max(&self) -> Option<&T> {
        if self.vals.is_empty() { return None; }
        return Some( &self.vals[0] )
    }

    /// Cahnge the root 
    pub fn set_max(&mut self, val:&T) {
        if let None = self.get_max() {
            self.push(val.clone());
            return; 
        };

        if self.vals.len() < self.max_size {
            self.vals.push(val.clone());
            self.bubble_up(self.vals.len() - 1);
        }
        else {
            self.vals[0] = val.clone();
            self.bubble_down(0);
        }
    }

    pub fn get_min(&self) -> Option<&T> {
        self.min.as_ref()
    }

    fn set_min(&mut self, val:&T) {
        self.min = Some(val.clone());
    }

    /// Pop the root
    pub fn extract(&mut self) -> Option<T> {
        if self.vals.is_empty() { return None; }
        
        let first:T = self.vals[0].clone();
        let last:T = self.vals.pop().unwrap();
        if self.vals.is_empty() { return Some(last); }

        self.vals[0] = last;

        self.bubble_down(0);

        Some(first)
    }

    ///Insert new value to the end of the heap
    pub fn push(&mut self, val:T) {
        if self.vals.is_empty() { 
            self.vals.push(val.clone());
            self.set_min(&val);
            return;
        }

        if self.vals.len() < self.max_size {
            self.vals.push(val.clone());
            self.set_min(&val);
                        
            self.bubble_up(self.vals.len() - 1);
        }
        else {
            if &val >= self.get_max().unwrap() { return; }
            if &val < self.get_min().unwrap() {
                self.set_max(&val);
            }
        }
    }

    /// Sort
    pub fn sort(&mut self) {
        let mut new:Vec<T> = Vec::with_capacity(self.vals.len());

        for _ in 0..self.vals.len() {
            if let Some(root) = self.extract() {
                new.push(root);
            }else { break; }
        }
        self.vals = new;
    }


    fn bubble_up(&mut self, mut idx:usize) {
        while idx > 0 {
            let parent_idx:usize = if idx % 2 == 0 { 
                (idx / 2) - 1
            } else { idx / 2 };
    
            if self.vals[idx] <= self.vals[parent_idx] { break; }
            
            self.swap(idx, parent_idx);

            idx = parent_idx;
        }
    }

    fn bubble_down(&mut self, mut idx:usize) {
        let hlen:usize = self.vals.len();

        while idx < self.vals.len() - 1 {
            if hlen == 2 {
                if self.vals[0] < self.vals[1] { self.swap(0,1); }
                break;
            }
            
            let (lc, rc) = ( (idx * 2) + 1, (idx * 2) + 2 );
            
            if rc >= hlen && lc >= hlen { // no left and right child
                break;
            }
            if rc >= hlen && lc < hlen { //left but not right child
                if self.vals[idx] < self.vals[lc] {
                    self.swap(idx, lc);
                }
                break;
            }
            
            if self.vals[rc] == self.vals[lc] && self.vals[idx] < self.vals[lc] {
                self.swap(idx, lc);
                idx = lc;
            }
            // left is bigger
            else if self.vals[lc] > self.vals[rc] && self.vals[idx] < self.vals[lc] {
                self.swap(idx, lc);
                idx = lc;
            }
            // right is bigger
            else if self.vals[rc] > self.vals[lc] && self.vals[idx] < self.vals[rc]  {
                self.swap(idx, rc);
                idx = rc;
            }else { break; }

        }
    }

    fn swap(&mut self, i1:usize, i2:usize) {
        let temp:T = self.vals[i1].clone();
        self.vals[i1] = self.vals[i2].clone();
        self.vals[i2] = temp;
    }
}

pub trait Heapify<T> {
    fn heapify(self) -> MaxHeap<T>;
}

impl<T> Heapify<T> for Vec<T> 
    where T: PartialOrd + Clone
{
    /// Convert Vec into a Heap
    fn heapify(self) -> MaxHeap<T> {
        let mut heap:MaxHeap<T> = MaxHeap::with_capacity(self.len());

        for val in self {
            heap.set_min(&val);
            heap.push(val);
        }
        heap
    }
}

pub trait FindSmallest<T> {
    fn find_smallest(&self, k: usize) -> Vec<T>;
}

impl<T> FindSmallest<T> for Vec<T> 
    where T: PartialOrd + Clone + Debug
{
    fn find_smallest(&self, k: usize) -> Vec<T> {
        if k == 0 || k >= self.len() { return Vec::new()}
        if self.is_empty() { return Vec::new(); }

        let mut heap:MaxHeap<T> = MaxHeap::with_capacity(k);

        for i in 0..k { 
            heap.push(self[i].clone()); 
        }

        for i in k..self.len() {
            if &self[i] >= heap.get_max().unwrap() { continue; }

            heap.set_max(&self[i]);
        }

        heap.sort();

        heap.to_vec()
    }
}

impl<T> fmt::Display for MaxHeap<T> 
    where T: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.vals)
    }
}
