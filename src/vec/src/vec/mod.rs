use std::fmt::Debug;

use linked_list::linked_list::{Next, Node};

/// Represents a singly linked vector.
#[derive(Debug)]
pub struct LinkedVec<T> {
    /// The size of the linked vector.
    size: usize,
    /// Pointer to the head node of the linked vector.
    head: Next<T>,
}

impl<T: Debug + Clone> LinkedVec<T> {
    /// Creates a new empty linked vector with the specified size.
    ///
    /// # Returns
    ///
    /// A new empty linked vector with the specified size.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec::LinkedVec;
    ///
    /// let vec: LinkedVec<i32> = LinkedVec::new();
    /// assert_eq!(vec.size(), 0);
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
    /// use vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new(0);
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
    /// use vec::LinkedVec;
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
    /// assert_eq!(linked_vec1.size(), 4);
    /// assert_eq!(linked_vec2.size(), 0);
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
    /// use vec::LinkedVec;
    ///
    /// let mut linked_vec = LinkedVec::new();
    /// linked_vec.push(1);
    /// linked_vec.push(3);
    ///
    /// linked_vec.insert(1, 2);
    ///
    /// assert_eq!(linked_vec.size(), 3);
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
    /// use vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new(0);
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
    /// use vec::LinkedVec;
    ///
    /// let mut linked_vec = LinkedVec::new();
    /// linked_vec.push(1);
    /// linked_vec.push(2);
    /// linked_vec.push(3);
    ///
    /// assert_eq!(linked_vec.remove(1), Some(2));
    /// assert_eq!(linked_vec.size(), 2);
    /// assert_eq!(linked_vec.remove(1), Some(3));
    /// assert_eq!(linked_vec.size(), 1);
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
    /// use vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new(0);
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
    /// use vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new(0);
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
    /// use vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new(0);
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
    /// use vec::LinkedVec;
    ///
    /// let mut linked_vec = LinkedVec::new();
    /// linked_vec.push(1);
    /// linked_vec.push(2);
    /// linked_vec.push(3);
    ///
    /// assert_eq!(linked_vec.find(1), Some(2));
    /// assert_eq!(linked_vec.find(3), None);
    /// ```
    pub fn find(&mut self, index: usize) -> Option<T> {
        if self.size <= index || self.is_empty() {
            None
        } else {
            let mut node = self.head.as_mut().unwrap();
            for _i in 0..index {
                node = node.next.as_mut().unwrap();
            }
            Some(node.data.clone())
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
    /// use vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new(0);
    /// vec.push(1);
    /// vec.push(2);
    /// let mut iter = vec.iter();
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&1));
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
    /// use vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new(0);
    /// vec.push(1);
    /// vec.push(2);
    /// let mut iter = vec.iter_mut();
    /// if let Some(data) = iter.next() {
    ///     *data = 3;
    /// }
    /// assert_eq!(iter.next(), Some(&mut 1));
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
    /// use vec::LinkedVec;
    ///
    /// let mut vec = LinkedVec::new(0);
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
    /// use vec::LinkedVec;
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
pub struct IntoIter<T: Clone + Debug>(LinkedVec<T>);

impl<T: Clone + Debug> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
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
