use std::fmt::Debug;

use vec::vec::LinkedVec;

/// A generic deque (double-ended queue) data structure implemented using a vector.
///
/// # Generic Parameters
///
/// * `T` - Type of elements stored in the deque.
///
/// # Fields
///
/// * `cap` - Maximum capacity of the deque.
/// * `data` - Internal vector storing the elements of the deque.
///
/// # Examples
///
/// ```
/// use deque::deque::Deque;
///
/// let mut deque: Deque<i32> = Deque::new(5);
///
/// // Add elements to the front and rear of the deque.
/// deque.add_front(1).unwrap();
/// deque.add_rear(2).unwrap();
///
/// // Remove elements from the front and rear of the deque.
/// let removed_front = deque.remove_front();
/// let removed_rear = deque.remove_rear();
///
/// assert_eq!(removed_front, Some(1));
/// assert_eq!(removed_rear, Some(2));
/// ```
#[derive(Debug)]
pub struct Deque<T> {
    cap: usize,
    data: LinkedVec<T>,
}

impl<T: Debug + Clone + PartialEq> Deque<T> {
    /// Creates a new empty deque with the given maximum capacity.
    ///
    /// # Arguments
    ///
    /// * `size` - Maximum capacity of the deque.
    ///
    /// # Returns
    ///
    /// A new empty deque with the specified capacity.
    pub fn new(size: usize) -> Self {
        Deque {
            cap: size,
            data: LinkedVec::new(),
        }
    }

    /// Adds an element to the front of the deque.
    ///
    /// # Arguments
    ///
    /// * `item` - The element to add to the front of the deque.
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, otherwise returns an error if the deque is full.
    pub fn add_front(&mut self, item: T) -> Result<(), String> {
        if self.data.len() == self.cap {
            return Err("Max size!".into());
        }

        self.data.push(item);

        Ok(())
    }

    /// Adds an element to the rear of the deque.
    ///
    /// # Arguments
    ///
    /// * `item` - The element to add to the rear of the deque.
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, otherwise returns an error if the deque is full.
    pub fn add_rear(&mut self, item: T) -> Result<(), String> {
        if self.data.len() == self.cap {
            return Err("Max size!".into());
        }

        self.data.insert(0, item);

        Ok(())
    }

    /// Removes an element from the front of the deque.
    ///
    /// # Returns
    ///
    /// An optional value containing the removed element, or `None` if the deque is empty.
    pub fn remove_front(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Removes an element from the rear of the deque.
    ///
    /// # Returns
    ///
    /// An optional value containing the removed element, or `None` if the deque is empty.
    pub fn remove_rear(&mut self) -> Option<T> {
        match self.data.peek() {
            Some(_) => self.data.remove(0),
            None => None,
        }
    }

    /// Checks if the deque is empty.
    ///
    /// # Returns
    ///
    /// `true` if the deque is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Checks if the deque is full.
    ///
    /// # Returns
    ///
    /// `true` if the deque is full, `false` otherwise.
    pub fn is_full(&self) -> bool {
        self.data.len() == self.cap
    }

    /// Returns the current size of the deque.
    ///
    /// # Returns
    ///
    /// The number of elements currently in the deque.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Clears the deque, removing all elements.
    pub fn clear(&mut self) {
        self.data = LinkedVec::new();
    }

    /// Returns an iterator over the elements of the deque.
    ///
    /// # Returns
    ///
    /// An iterator yielding references to the elements of the deque.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Returns a mutable iterator over the elements of the deque.
    ///
    /// # Returns
    ///
    /// A mutable iterator yielding references to the elements of the deque.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    /// Consumes the deque and returns an iterator over its elements.
    ///
    /// # Returns
    ///
    /// An iterator consuming the deque and yielding its elements.
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.data.into_iter()
    }
}
