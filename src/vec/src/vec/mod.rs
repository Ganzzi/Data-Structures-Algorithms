use std::ops::{Index, IndexMut, Range, RangeFrom, RangeTo};
use std::{cmp::Ordering, fmt::Debug};

/// Type alias for a singly linked list node with an optional next node.
pub type Next<T> = Option<Box<Node<T>>>;

/// Represents a node in a singly linked list.
#[derive(Debug, Clone, PartialEq)]
pub struct Node<T> {
    /// The data stored in the node.
    pub data: T,
    /// Pointer to the next node in the linked list.
    pub next: Next<T>,
}

impl<T> Node<T> {
    /// Creates a new node with the given data and next node pointer.
    pub fn new(data: T, next: Next<T>) -> Self {
        Node { data, next }
    }
}

#[macro_export]
macro_rules! linked_vec {
    ($($elem:expr),* $(,)?) => {{
        let mut vec = $crate::vec::LinkedVec::new();
        $(
            vec.push($elem);
        )*
        vec
    }};
    ($elem:expr; $size:expr) => {{
        let mut vec = $crate::vec::LinkedVec::new();
        for _ in 0..$size {
            vec.push($elem.clone());
        }
        vec
    }};
}

/// Represents a singly linked vector.
///
/// # Generic Parameters
///
/// * `T` - Type of elements stored in the linked vector.
///
/// # Fields
///
/// * `size` - The size of the linked vector.
/// * `head` - Pointer to the head node of the linked vector.
///
/// # Examples
///
/// ```
/// use crate::vec::vec::LinkedVec;
///
/// let mut vec: LinkedVec<i32> = LinkedVec::new();
/// vec.push(1);
/// vec.push(2);
/// vec.push(3);
/// let last = vec.pop();
///
/// assert_eq!(vec.len(), 2);
/// assert_eq!(last, Some(3));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct LinkedVec<T> {
    /// The size of the linked vector.
    size: usize,
    /// Pointer to the head node of the linked vector.
    head: Next<T>,
}
impl<T: Clone + Debug + PartialEq> std::ops::Index<usize> for LinkedVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.find(index).expect("Index out of bounds")
    }
}

impl<T: Clone + Debug + PartialEq> std::ops::IndexMut<usize> for LinkedVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.find_mut(index).expect("Index out of bounds")
    }
}

impl<T: Clone + Debug + PartialEq> Index<Range<usize>> for LinkedVec<T> {
    type Output = [T];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        let vec: Vec<T> = range
            .map(|i| self.find(i).expect("Index out of bounds").clone())
            .collect();
        Box::leak(vec.into_boxed_slice())
    }
}

impl<T: Clone + Debug + PartialEq> IndexMut<Range<usize>> for LinkedVec<T> {
    fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
        let mut result = LinkedVec::new();
        for i in range {
            result.push(self.find_mut(i).expect("Index out of bounds").clone());
        }
        Box::leak(result.to_vec().into_boxed_slice())
    }
}

impl<T: Clone + Debug + PartialEq> Index<RangeTo<usize>> for LinkedVec<T> {
    type Output = LinkedVec<T>;

    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        let mut result = LinkedVec::new();
        for i in 0..range.end {
            result.push(self.find(i).expect("Index out of bounds").clone());
        }
        Box::leak(Box::new(result))
    }
}

impl<T: Clone + Debug + PartialEq> IndexMut<RangeTo<usize>> for LinkedVec<T> {
    fn index_mut(&mut self, range: RangeTo<usize>) -> &mut Self::Output {
        let mut result = LinkedVec::new();
        for i in 0..range.end {
            result.push(self.find_mut(i).expect("Index out of bounds").clone());
        }
        Box::leak(Box::new(result))
    }
}

impl<T: Clone + Debug + PartialEq> Index<RangeFrom<usize>> for LinkedVec<T> {
    type Output = LinkedVec<T>;

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        let mut result = LinkedVec::new();
        for i in range.start..self.size {
            result.push(self.find(i).expect("Index out of bounds").clone());
        }
        Box::leak(Box::new(result))
    }
}
impl<T: Clone + Debug + PartialEq> IndexMut<RangeFrom<usize>> for LinkedVec<T> {
    fn index_mut(&mut self, range: RangeFrom<usize>) -> &mut Self::Output {
        let mut result = LinkedVec::new();
        for i in range.start..self.size {
            result.push(self.find_mut(i).expect("Index out of bounds").clone());
        }
        Box::leak(Box::new(result))
    }
}

impl<T: Debug + Clone + PartialEq> LinkedVec<T> {
    /// Creates a new empty linked vector with the specified size.
    ///
    /// # Returns
    ///
    /// A new empty linked vector with the specified size.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let vec: LinkedVec<i32> = LinkedVec::new();
    /// assert_eq!(vec.len(), 0);
    /// ```
    pub fn new() -> Self {
        LinkedVec {
            size: 0,
            head: None,
        }
    }

    /// Pushes an element onto the front of the linked vector.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be pushed onto the linked vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new();
    /// vec.push(1);
    /// assert_eq!(vec.len(), 1);
    /// ```
    pub fn push(&mut self, data: T) {
        let node = Node::new(data, None);
        if self.is_empty() {
            self.head = Some(Box::new(node));
        } else {
            let mut current_node = self.head.as_mut().unwrap();
            for _i in 0..self.size - 1 {
                current_node = current_node.next.as_mut().unwrap();
            }
            current_node.next = Some(Box::new(node));
        }
        self.size += 1;
    }

    /// Appends the elements of another LinkedVec to the end of this LinkedVec, clearing the other LinkedVec.
    ///
    /// # Arguments
    ///
    /// * `other` - A mutable reference to another LinkedVec whose elements will be appended to this LinkedVec.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut linked_vec1 = LinkedVec::new();
    /// linked_vec1.push(1);
    /// linked_vec1.push(2);
    ///
    /// let mut linked_vec2 = LinkedVec::new();
    /// linked_vec2.push(3);
    /// linked_vec2.push(4);
    ///
    /// linked_vec1.append(&mut linked_vec2);
    ///
    /// assert_eq!(linked_vec1.len(), 4);
    /// assert_eq!(linked_vec2.len(), 0);
    /// ```
    pub fn append(&mut self, other: &mut Self) {
        while let Some(node) = other.head.as_mut().take() {
            self.push(node.data.clone());
            other.head = node.next.take();
        }
        other.clear();
    }

    /// Inserts an element at the specified index in the LinkedVec.
    ///
    /// If the specified index is greater than the size of the LinkedVec, the element will be inserted at the end.
    ///
    /// # Arguments
    ///
    /// * `index` - The index at which to insert the element.
    /// * `data` - The element to be inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut linked_vec = LinkedVec::new();
    /// linked_vec.push(1);
    /// linked_vec.push(3);
    ///
    /// linked_vec.insert(1, 2);
    ///
    /// assert_eq!(linked_vec.len(), 3);
    /// assert_eq!(linked_vec.pop(), Some(3));
    /// assert_eq!(linked_vec.pop(), Some(2));
    /// assert_eq!(linked_vec.pop(), Some(1));
    /// ```
    pub fn insert(&mut self, mut index: usize, data: T) {
        if self.size < index {
            index = self.size;
        }

        let mut node = Node::new(data, None);
        if self.is_empty() {
            self.head = Some(Box::new(node));
        } else if index == 0 {
            node.next = self.head.take();
            self.head = Some(Box::new(node));
        } else {
            let mut node_before = self.head.as_mut().unwrap();
            for _i in 0..index - 1 {
                node_before = node_before.next.as_mut().unwrap();
            }
            node.next = node_before.next.take();
            node_before.next = Some(Box::new(node));
        }

        self.size += 1;
    }

    /// Pops an element from the front of the linked vector.
    ///
    /// # Returns
    ///
    /// An option containing the popped data, or None if the linked vector is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new();
    /// vec.push(1);
    /// assert_eq!(vec.pop(), Some(1));
    /// assert_eq!(vec.is_empty(), true);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.remove(self.size - 1)
        }
    }

    /// Returns a reference to the last element in the linked vector, or None if it is empty.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the last element, or None if the linked vector is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new();
    /// vec.push(1);
    /// assert_eq!(vec.peek(), Some(1));
    /// vec.push(2);
    /// assert_eq!(vec.peek(), Some(2));
    /// vec.pop();
    /// assert_eq!(vec.peek(), Some(1));
    /// vec.pop();
    /// assert_eq!(vec.peek(), None);
    /// ```
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.find(self.size - 1)
        }
    }

    /// Returns a mutable reference to the last element in the linked vector, or None if it is empty.
    ///
    /// # Returns
    ///
    /// An option containing a mutable reference to the last element, or None if the linked vector is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new();
    /// vec.push(1);
    /// if let Some(data) = vec.peek_mut() {
    ///     *data = 2;
    /// }
    /// assert_eq!(vec.peek(), Some(2));
    /// ```
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            self.find_mut(self.size - 1)
        }
    }

    /// Removes the element at the specified index from the LinkedVec, returning it.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the element to be removed.
    ///
    /// # Returns
    ///
    /// The removed element, or None if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut linked_vec = LinkedVec::new();
    /// linked_vec.push(1);
    /// linked_vec.push(2);
    /// linked_vec.push(3);
    ///
    /// assert_eq!(linked_vec.remove(1), Some(2));
    /// assert_eq!(linked_vec.len(), 2);
    /// assert_eq!(linked_vec.remove(1), Some(3));
    /// assert_eq!(linked_vec.len(), 1);
    /// ```
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if self.size < index {
            return None;
        }

        let mut node;
        if 0 == index {
            node = self.head.take().unwrap();
            self.head = node.next.take();
        } else {
            let mut node_before = self.head.as_mut().unwrap();
            for _i in 0..index - 1 {
                node_before = node_before.next.as_mut().unwrap();
            }
            node = node_before.next.take().unwrap();
            node_before.next = node.next.take();
        }
        self.size -= 1;

        Some(node.data)
    }

    /// Checks if the linked vector is empty.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the linked vector is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new();
    /// assert_eq!(vec.is_empty(), true);
    /// vec.push(1);
    /// assert_eq!(vec.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns the size of the linked vector.
    ///
    /// # Returns
    ///
    /// The number of elements in the linked vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new();
    /// assert_eq!(vec.len(), 0);
    /// vec.push(1);
    /// assert_eq!(vec.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.size
    }

    /// Clears the linked vector, removing all elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new();
    /// vec.push(1);
    /// vec.clear();
    /// assert_eq!(vec.is_empty(), true);
    /// ```
    pub fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }

    /// Finds and returns the element at the specified index in the LinkedVec.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the element to be found.
    ///
    /// # Returns
    ///
    /// The element at the specified index, or None if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut linked_vec = LinkedVec::new();
    /// linked_vec.push(1);
    /// linked_vec.push(2);
    /// linked_vec.push(3);
    ///
    /// assert_eq!(linked_vec.find(1), Some(2));
    /// assert_eq!(linked_vec.find(3), None);
    /// ```
    pub fn find(&self, index: usize) -> Option<&T> {
        if self.size <= index || self.is_empty() {
            None
        } else {
            let mut node = self.head.as_ref().unwrap();
            for _i in 0..index {
                node = node.next.as_ref().unwrap();
            }
            Some(&node.data)
        }
    }

    pub fn contains(&self, data: &T) -> bool {
        let mut node = self.head.as_ref();
        while let Some(n) = node {
            if n.data == *data {
                return true;
            }
            node = n.next.as_ref();
        }
        false
    }

    /// Finds and returns a mutable reference to the element at the specified index in the LinkedVec.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the element to be found.
    ///
    /// # Returns
    ///
    /// A mutable reference to the element at the specified index, or None if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut linked_vec = LinkedVec::new();
    /// linked_vec.push(1);
    /// linked_vec.push(2);
    /// linked_vec.push(3);
    ///
    /// if let Some(data) = linked_vec.find_mut(1) {
    ///     *data = 4;
    /// }
    /// assert_eq!(linked_vec.find(1), Some(4));
    /// ```
    pub fn find_mut(&mut self, index: usize) -> Option<&mut T> {
        if self.size <= index || self.is_empty() {
            None
        } else {
            let mut node = self.head.as_mut().unwrap();
            for _i in 0..index {
                node = node.next.as_mut().unwrap();
            }
            Some(&mut node.data)
        }
    }

    /// Returns an iterator over the elements of the linked vector.
    ///
    /// # Returns
    ///
    /// An iterator yielding references to the elements of the linked vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new();
    /// vec.push(1);
    /// vec.push(2);
    /// let mut iter = vec.iter();
    ///
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    /// Returns a mutable iterator over the elements of the linked vector.
    ///
    /// # Returns
    ///
    /// A mutable iterator yielding references to the elements of the linked vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new();
    /// vec.push(1);
    /// vec.push(2);
    /// let mut iter = vec.iter_mut();
    /// if let Some(data) = iter.next() {
    ///     *data = 3;
    /// }
    ///
    /// assert_eq!(iter.next(), Some(&mut 2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }

    /// Consumes the linked vector and returns an iterator over its elements.
    ///
    /// # Returns
    ///
    /// An iterator consuming the linked vector and yielding its elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new();
    /// vec.push(1);
    /// vec.push(2);
    /// let mut iter = vec.into_iter();
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    /// Prints the elements of the linked vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut linked_vec = LinkedVec::new();
    /// linked_vec.push(1);
    /// linked_vec.push(2);
    /// linked_vec.print();
    /// ```
    pub fn print(&self) {
        for data in self.iter() {
            print!("{:?} ", data);
        }
    }

    /// Converts the linked vector into a standard Vec.
    ///
    /// # Returns
    ///
    /// A Vec containing all elements of the linked vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut linked_vec = LinkedVec::new();
    /// linked_vec.push(1);
    /// linked_vec.push(2);
    /// let vec = linked_vec.to_vec();
    /// assert_eq!(vec, vec![1, 2]);
    /// ```
    pub fn to_vec(&self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.size);
        for data in self.iter() {
            vec.push(data.clone());
        }
        vec
    }

    /// Swaps the elements at the specified indices in the LinkedVec.
    ///
    /// # Arguments
    ///
    /// * `index1` - The index of the first element.
    /// * `index2` - The index of the second element.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut linked_vec = LinkedVec::new();
    /// linked_vec.push(1);
    /// linked_vec.push(2);
    /// linked_vec.push(3);
    ///
    /// linked_vec.swap(0, 2);
    ///
    /// assert_eq!(linked_vec.find(0), Some(&3));
    /// assert_eq!(linked_vec.find(2), Some(&1));
    /// ```
    pub fn swap(&mut self, index1: usize, index2: usize) {
        if index1 >= self.size || index2 >= self.size {
            return;
        }

        if index1 == index2 {
            return;
        }

        let data1 = self.find(index1).unwrap().clone();
        let data2 = self.find(index2).unwrap().clone();

        self.remove(index1);
        self.insert(index1, data2);
        self.remove(index2);
        self.insert(index2, data1);
    }

    /// Reverses the order of elements in the LinkedVec.
    ///
    /// # Returns
    ///
    /// A new LinkedVec with the elements in reverse order.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut linked_vec = LinkedVec::new();
    /// linked_vec.push(1);
    /// linked_vec.push(2);
    /// linked_vec.push(3);
    ///
    /// let reversed_vec = linked_vec.reverse();
    ///
    /// assert_eq!(reversed_vec.find(0), Some(&3));
    /// assert_eq!(reversed_vec.find(1), Some(&2));
    /// assert_eq!(reversed_vec.find(2), Some(&1));
    /// ```
    pub fn reverse(&self) -> Self {
        let mut reversed_vec = LinkedVec::new();
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            reversed_vec.push_front(node.data.clone());
            current = node.next.as_ref();
        }

        reversed_vec
    }

    fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            data: value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    /// Drains the elements from the linked vector, returning an iterator over the removed elements.
    ///
    /// # Returns
    ///
    /// An iterator yielding the removed elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new();
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    ///
    /// let mut drain_iter = vec.drain();
    /// assert_eq!(drain_iter.next(), Some(1));
    /// assert_eq!(drain_iter.next(), Some(2));
    /// assert_eq!(drain_iter.next(), Some(3));
    /// assert_eq!(drain_iter.next(), None);
    /// assert_eq!(vec.len(), 0);
    /// ```
    pub fn drain(&mut self) -> Drain<T> {
        Drain { linked_vec: self }
    }

    pub fn extend(&mut self, other: LinkedVec<T>) {
        for data in other.iter() {
            self.push(data.clone());
        }
    }

    pub fn binary_search_by(&self, f: impl Fn(&T) -> Ordering) -> Result<usize, usize> {
        let mut low = 0;
        let mut high = self.size;

        while low < high {
            let mid = (low + high) / 2;
            match f(self.find(mid).unwrap()) {
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
                Ordering::Equal => return Ok(mid),
            }
        }

        Err(low)
    }

    pub fn as_mut(&mut self) -> &mut Self {
        self
    }

    pub fn copy_from_slice(&mut self, other: &LinkedVec<T>) {
        self.clear();
        for data in other.iter() {
            self.push(data.clone());
        }
    }
}

/// Iterator over the drained elements of a linked vector.
pub struct Drain<'a, T: 'a> {
    pub linked_vec: &'a mut LinkedVec<T>,
}

impl<'a, T: Clone + Debug + PartialEq> Iterator for Drain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.linked_vec.pop()
    }
}

/// Iterator over the elements of a linked vector.
pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

/// Mutable iterator over the elements of a linked vector.
pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
    }
}

/// Consuming iterator over the elements of a linked vector.
pub struct IntoIter<T: Clone + Debug + PartialEq>(LinkedVec<T>);

impl<T: Clone + Debug + PartialEq> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T: Debug + Clone + PartialEq> FromIterator<T> for LinkedVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut linked_vec = LinkedVec::new();
        for item in iter {
            linked_vec.push(item);
        }
        linked_vec
    }
}

/// Custom drop implementation for releasing linked vector memory.
impl<T> Drop for LinkedVec<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}
