use std::fmt::Debug;

use vec::{linked_vec, vec::LinkedVec};

macro_rules! parent {
    ($index:expr) => {
        $index >> 1
    };
}

macro_rules! left_child {
    ($index:expr) => {
        $index << 1
    };
}

macro_rules! right_child {
    ($index:expr) => {
        ($index << 1) + 1
    };
}

/// Represents a binary heap.
///
/// # Generic Parameters
///
/// * `T` - Type of elements stored in the binary heap.
///
/// # Examples
/// ```
/// use crate::tree::binary_heap::BinaryHeap;
/// use vec::linked_vec;
///
/// let mut heap = BinaryHeap::new();

/// heap.push(10);
/// heap.push(4);
/// heap.push(15);
/// heap.push(20);
/// heap.push(0);
///
/// assert_eq!(heap.size(), 5);
/// assert_eq!(heap.min(), Some(&0));
///
/// while !heap.is_empty() {
///     println!("Popped element: {:?}", heap.pop());
/// }
///
/// let data = linked_vec![3, 1, 6, 5, 2, 4];
/// heap.build_new(&data);
/// assert_eq!(heap.size(), 6);
///
/// let additional_data = linked_vec![7, 8, 9];
/// heap.build_add(additional_data);
/// assert_eq!(heap.size(), 9);
///
/// ```
#[derive(Debug)]
pub struct BinaryHeap<T> {
    data: LinkedVec<T>,
    size: usize,
}

impl<T: Debug + Default + Ord + Clone> BinaryHeap<T> {
    /// Creates a new empty binary heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::binary_heap::BinaryHeap;
    /// use vec::linked_vec;
    ///
    /// let heap: BinaryHeap<i32> = BinaryHeap::new();
    /// ```
    pub fn new() -> Self {
        BinaryHeap {
            data: linked_vec![T::default()],
            size: 0,
        }
    }

    /// Returns the size of the binary heap.
    ///
    /// # Returns
    ///
    /// The number of elements in the binary heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::binary_heap::BinaryHeap;
    ///
    /// let heap: BinaryHeap<i32> = BinaryHeap::new();
    /// assert_eq!(heap.size(), 0);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Checks if the binary heap is empty.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the binary heap is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::binary_heap::BinaryHeap;
    ///
    /// let heap: BinaryHeap<i32> = BinaryHeap::new();
    /// assert!(heap.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns a reference to the minimum element in the binary heap.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the minimum element, or None if the heap is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::binary_heap::BinaryHeap;
    ///
    /// let mut heap = BinaryHeap::new();
    /// heap.push(10);
    /// heap.push(5);
    /// heap.push(15);
    /// assert_eq!(heap.min(), Some(&5));
    /// ```
    pub fn min(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.data.find(1)
        }
    }

    /// Returns a reference to the maximum element in the binary heap.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the maximum element, or None if the heap is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::binary_heap::BinaryHeap;
    ///
    /// let mut heap = BinaryHeap::new();
    /// heap.push(10);
    /// heap.push(5);
    /// heap.push(15);
    /// assert_eq!(heap.max(), Some(&15));
    /// ```
    pub fn max(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.data.find(self.size)
        }
    }

    /// Inserts a value into the binary heap.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be inserted into the binary heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::binary_heap::BinaryHeap;
    ///
    /// let mut heap = BinaryHeap::new();
    /// heap.push(10);
    /// heap.push(5);
    /// heap.push(15);
    /// assert_eq!(heap.min(), Some(&5));
    /// ```
    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.size += 1;
        self.move_up(self.size);
    }

    fn move_up(&mut self, mut index: usize) {
        loop {
            let parent_index = parent!(index);
            if parent_index == 0 {
                break;
            }

            if self.data.find(parent_index).unwrap() > self.data.find(index).unwrap() {
                self.data.swap(parent_index, index);
            }

            index = parent_index;
        }
    }

    /// Removes and returns the minimum element from the binary heap.
    ///
    /// # Returns
    ///
    /// An option containing the minimum element, or None if the heap is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::binary_heap::BinaryHeap;
    ///
    /// let mut heap = BinaryHeap::new();
    /// heap.push(10);
    /// heap.push(5);
    /// heap.push(15);
    /// assert_eq!(heap.pop(), Some(5));
    /// assert_eq!(heap.pop(), Some(10));
    /// assert_eq!(heap.pop(), Some(15));
    /// assert_eq!(heap.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if 0 == self.size {
            None
        } else {
            self.data.swap(1, self.size);
            let value = self.data.pop();
            self.size -= 1;
            self.move_down(1);
            value
        }
    }

    fn move_down(&mut self, mut index: usize) {
        loop {
            if left_child!(index) > self.size {
                break;
            }

            let min_child = self.min_child(index);
            if self.data.find(min_child).unwrap() < self.data.find(index).unwrap() {
                self.data.swap(min_child, index);
            }

            index = min_child;
        }
    }

    fn min_child(&self, index: usize) -> usize {
        let (left, right) = (left_child!(index), right_child!(index));

        if right > self.size {
            left
        } else if self.data.find(left).unwrap() < self.data.find(right).unwrap() {
            left
        } else {
            right
        }
    }

    /// Builds a new binary heap from the given data.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to a `LinkedVec` containing the data to build the heap from.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::binary_heap::BinaryHeap;
    /// use vec::linked_vec;
    ///
    /// let data = linked_vec![10, 5, 15];
    /// let mut heap = BinaryHeap::new();
    /// heap.build_new(&data);
    /// assert_eq!(heap.min(), Some(&5));
    /// ```
    pub fn build_new(&mut self, data: &LinkedVec<T>) {
        for _ in 0..self.size {
            let _ = self.data.pop();
        }

        for d in data.iter() {
            self.push(d.clone());
        }

        self.size = data.len();

        let mut parent = parent!(self.size);
        while parent > 0 {
            self.move_down(parent);
            parent -= 1;
        }
    }

    /// Adds the given data to the existing binary heap.
    ///
    /// # Arguments
    ///
    /// * `data` - A `LinkedVec` containing the data to be added to the heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::binary_heap::BinaryHeap;
    /// use vec::linked_vec;
    ///
    /// let data = linked_vec![10, 5, 15];
    /// let mut heap = BinaryHeap::new();
    /// heap.build_add(data);
    /// assert_eq!(heap.min(), Some(&5));
    /// ```
    pub fn build_add(&mut self, data: LinkedVec<T>) {
        for value in data.iter() {
            self.push(value.clone());
        }
    }
}
