/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.items.push(value);
        self.count += 1;
        let mut index = self.len();
        //? swim operation, 调整堆的结构
        while index > 1 && (self.comparator)(&self.items[index], &self.items[self.parent_idx(index)]){
            // self.items.swap(index, self.parent_idx(index));
            let fa_index = self.parent_idx(index);
            self.items.swap(index, fa_index);
            index = fa_index;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        self.items.swap(1, self.count);
        let top = self.items.pop().unwrap();
        self.count -= 1;

        //? heapify operation
        //? maximum heap, comparator为|a: &T, b: &T| a > b
        //? minimum heap, comparator为|a: &T, b: &T| a < b
        let mut index = 1;
        while self.children_present(index) {
            let mut ch_idx = self.left_child_idx(index);
            if ch_idx + 1 <= self.count && (self.comparator)(&self.items[ch_idx + 1], &self.items[ch_idx]) {
                ch_idx += 1;
            }
            if !(self.comparator)(&self.items[index], &self.items[ch_idx]) {
                self.items.swap(index, ch_idx);
            }
            index = ch_idx;
        } 
        Some(top)
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {   //? index of the left child
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {  //? index of the right index
        self.left_child_idx(idx) + 1                  //? 2 * idx + 1
    }

    // fn smallest_child_idx(&self, idx: usize) -> usize {
    //     //TODO
    //     0
        // if !self.children_present(idx) {
        //     return idx;
        // }
		// let left_idx = self.left_child_idx(idx);
        // let right_idx = self.right_child_idx(idx);
        // if self.items[left_idx] <= self.items[right_idx] {
        //     return left_idx;
        // } else {
        //     return right_idx;
        // }
    // }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		self.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}