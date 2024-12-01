/// Type alias for a singly linked list node with an optional next node.
pub type Next<T> = Option<Box<Node<T>>>;

/// Represents a node in a singly linked list.
#[derive(Debug)]
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

/// Represents a singly linked list.
///
/// # Generic Parameters
///
/// * `T` - Type of elements stored in the linked list.
///
/// # Fields
///
/// * `size` - The size of the linked list.
/// * `head` - Pointer to the head node of the linked list.
///
/// # Examples
///
/// ```
/// use crate::linked_list::linked_list::LinkedList;
///
/// let mut list: LinkedList<i32> = LinkedList::new();
/// list.push(1);
/// list.push(2);
/// let last = list.pop();
///
/// assert_eq!(list.size(), 1);
/// assert_eq!(last, Some(2));
/// ```
pub struct LinkedList<T> {
    /// The size of the linked list.
    size: usize,
    /// Pointer to the head node of the linked list.
    head: Next<T>,
}

impl<T> LinkedList<T> {
    /// Creates a new empty linked list with the specified size.
    ///
    /// # Returns
    ///
    /// A new empty linked list with the specified size.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::linked_list::linked_list::LinkedList;
    ///
    /// let list: LinkedList<i32> = LinkedList::new();
    /// assert_eq!(list.size(), 0);
    /// ```
    pub fn new() -> Self {
        LinkedList {
            size: 0,
            head: None,
        }
    }

    /// Pushes an element onto the front of the linked list.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be pushed onto the linked list.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::linked_list::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// assert_eq!(list.peek(), Some(&1));
    /// ```
    pub fn push(&mut self, data: T) {
        let new_head = Box::new(Node::new(data, self.head.take()));
        self.head = Some(new_head);
        self.size += 1;
    }

    /// Pops an element from the front of the linked list.
    ///
    /// # Returns
    ///
    /// An option containing the popped data, or None if the linked list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::linked_list::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// assert_eq!(list.pop(), Some(1));
    /// assert_eq!(list.is_empty(), true);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.data
        })
    }

    /// Returns a reference to the data at the front of the linked list.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the front data, or None if the linked list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::linked_list::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// assert_eq!(list.peek(), Some(&1));
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    /// Returns a mutable reference to the data at the front of the linked list.
    ///
    /// # Returns
    ///
    /// An option containing a mutable reference to the front data, or None if the linked list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::linked_list::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// if let Some(data) = list.peek_mut() {
    ///     *data = 2;
    /// }
    /// assert_eq!(list.peek(), Some(&2));
    /// ```
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    /// Checks if the linked list is empty.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the linked list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::linked_list::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.is_empty(), true);
    /// list.push(1);
    /// assert_eq!(list.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns the size of the linked list.
    ///
    /// # Returns
    ///
    /// The number of elements in the linked list.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::linked_list::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.size(), 0);
    /// list.push(1);
    /// assert_eq!(list.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Clears the linked list, removing all elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::linked_list::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.clear();
    /// assert_eq!(list.is_empty(), true);
    /// ```
    pub fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }

    /// Returns an iterator over the elements of the linked list.
    ///
    /// # Returns
    ///
    /// An iterator yielding references to the elements of the linked list.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::linked_list::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    /// Returns a mutable iterator over the elements of the linked list.
    ///
    /// # Returns
    ///
    /// A mutable iterator yielding references to the elements of the linked list.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::linked_list::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    /// let mut iter = list.iter_mut();
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

    /// Consumes the linked list and returns an iterator over its elements.
    ///
    /// # Returns
    ///
    /// An iterator consuming the linked list and yielding its elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::linked_list::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    /// let mut iter = list.into_iter();
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

/// Iterator over the elements of a linked list.
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

/// Mutable iterator over the elements of a linked list.
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

/// Consuming iterator over the elements of a linked list.
pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

/// Custom drop implementation for releasing linked list memory.
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}
