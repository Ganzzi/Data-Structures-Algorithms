use std::cmp::Ordering::*;
use std::fmt::Debug;

use queue::queue::Queue;

type Link<U, T> = Option<Box<BinarySearchTree<U, T>>>;

/// Represents a binary search tree.
///
/// # Generic Parameters
///
/// * `U` - Type of the keys stored in the binary search tree.
/// * `T` - Type of the values stored in the binary search tree.
///
/// # Examples
///
/// ```
/// use tree::binary_search_tree::BinarySearchTree;
///
/// let mut tree = BinarySearchTree::new();
///
/// tree.insert(5, 10);
/// tree.insert(3, 6);
/// tree.insert(7, 14);
/// tree.insert(2, 4);
///
/// assert_eq!(tree.size(), 4);
/// assert_eq!(tree.leaf_size(), 2);
/// assert_eq!(tree.none_leaf_size(), 2);
/// assert_eq!(tree.depth(), 3);
/// assert_eq!(tree.contains(&3), true);
/// assert_eq!(tree.contains(&4), false);
/// assert_eq!(tree.get(&5), Some(&10));
///
/// let (min_key, min_value) = tree.min();
/// assert_eq!(min_key, Some(&2));
/// assert_eq!(min_value, Some(&4));
///
/// let (max_key, max_value) = tree.max();
/// assert_eq!(max_key, Some(&7));
/// assert_eq!(max_value, Some(&14));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BinarySearchTree<U, T> {
    key: Option<U>,
    value: Option<T>,
    left: Link<U, T>,
    right: Link<U, T>,
}

impl<U, T> BinarySearchTree<U, T>
where
    U: Copy + Debug + Ord,
    T: Copy + Debug,
{
    /// Creates a new empty binary search tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_search_tree::BinarySearchTree;
    ///
    /// let tree: BinarySearchTree<i32, &str> = BinarySearchTree::new();
    /// ```
    pub fn new() -> Self {
        BinarySearchTree {
            key: None,
            value: None,
            left: None,
            right: None,
        }
    }

    /// Checks if the binary search tree is empty.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the binary search tree is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_search_tree::BinarySearchTree;
    ///
    /// let tree: BinarySearchTree<i32, &str> = BinarySearchTree::new();
    /// assert!(tree.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.key.is_none()
    }

    /// Returns the size of the binary search tree.
    ///
    /// # Returns
    ///
    /// The number of nodes in the binary search tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_search_tree::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(10, "ten");
    /// tree.insert(5, "five");
    /// tree.insert(15, "fifteen");
    /// assert_eq!(tree.size(), 3);
    /// ```
    pub fn size(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            1 + self.left.as_ref().map_or(0, |node| node.size())
                + self.right.as_ref().map_or(0, |node| node.size())
        }
    }

    /// Returns the number of leaf nodes in the binary search tree.
    ///
    /// # Returns
    ///
    /// The number of leaf nodes in the binary search tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_search_tree::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(10, "ten");
    /// tree.insert(5, "five");
    /// tree.insert(15, "fifteen");
    /// assert_eq!(tree.leaf_size(), 2);
    /// ```
    pub fn leaf_size(&self) -> usize {
        if self.is_empty() {
            0
        } else if self.left.is_none() && self.right.is_none() {
            1
        } else {
            self.left.as_ref().map_or(0, |node| node.leaf_size())
                + self.right.as_ref().map_or(0, |node| node.leaf_size())
        }
    }

    /// Returns the number of non-leaf nodes in the binary search tree.
    ///
    /// # Returns
    ///
    /// The number of non-leaf nodes in the binary search tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_search_tree::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(10, "ten");
    /// tree.insert(5, "five");
    /// tree.insert(15, "fifteen");
    /// assert_eq!(tree.none_leaf_size(), 1);
    /// ```
    pub fn none_leaf_size(&self) -> usize {
        self.size() - self.leaf_size()
    }

    /// Returns the depth of the binary search tree.
    ///
    /// # Returns
    ///
    /// The depth of the binary search tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_search_tree::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(10, "ten");
    /// tree.insert(5, "five");
    /// tree.insert(15, "fifteen");
    /// assert_eq!(tree.depth(), 2);
    /// ```
    pub fn depth(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            1 + self
                .left
                .as_ref()
                .map_or(0, |node| node.depth())
                .max(self.right.as_ref().map_or(0, |node| node.depth()))
        }
    }

    /// Inserts a key-value pair into the binary search tree.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to be inserted.
    /// * `value` - The value to be inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_search_tree::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(10, "ten");
    /// tree.insert(5, "five");
    /// tree.insert(15, "fifteen");
    /// ```
    pub fn insert(&mut self, key: U, value: T) {
        if self.is_empty() {
            self.key = Some(key);
            self.value = Some(value);
        } else {
            match self.key.as_ref().unwrap().cmp(&key) {
                Equal => self.value = Some(value),
                Greater => match self.left.as_mut() {
                    None => {
                        self.left = Some(Box::new(BinarySearchTree::new()));
                        self.left.as_mut().unwrap().insert(key, value);
                    }
                    Some(left) => left.insert(key, value),
                },
                Less => match self.right.as_mut() {
                    None => {
                        self.right = Some(Box::new(BinarySearchTree::new()));
                        self.right.as_mut().unwrap().insert(key, value);
                    }
                    Some(right) => right.insert(key, value),
                },
            }
        }
    }

    /// Checks if the binary search tree contains the specified key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check for.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the binary search tree contains the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_search_tree::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(10, "ten");
    /// tree.insert(5, "five");
    /// tree.insert(15, "fifteen");
    /// assert!(tree.contains(&10));
    /// assert!(tree.contains(&5));
    /// assert!(tree.contains(&15));
    /// assert!(!tree.contains(&20));
    /// ```
    pub fn contains(&self, key: &U) -> bool {
        if self.is_empty() {
            false
        } else {
            match self.key.as_ref().unwrap().cmp(key) {
                Equal => true,
                Greater => self.left.as_ref().map_or(false, |node| node.contains(key)),
                Less => self.right.as_ref().map_or(false, |node| node.contains(key)),
            }
        }
    }

    /// Returns a reference to the value associated with the specified key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the value, or None if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_search_tree::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(10, "ten");
    /// tree.insert(5, "five");
    /// tree.insert(15, "fifteen");
    /// assert_eq!(tree.get(&10), Some(&"ten"));
    /// assert_eq!(tree.get(&5), Some(&"five"));
    /// assert_eq!(tree.get(&15), Some(&"fifteen"));
    /// assert_eq!(tree.get(&20), None);
    /// ```
    pub fn get(&self, key: &U) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            match self.key.as_ref().unwrap().cmp(key) {
                Equal => self.value.as_ref(),
                Greater => self.left.as_ref().and_then(|node| node.get(key)),
                Less => self.right.as_ref().and_then(|node| node.get(key)),
            }
        }
    }

    /// Returns the minimum key and its associated value in the binary search tree.
    ///
    /// # Returns
    ///
    /// A tuple containing an option with a reference to the minimum key and an option with a reference to the associated value.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_search_tree::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(10, "ten");
    /// tree.insert(5, "five");
    /// tree.insert(15, "fifteen");
    /// assert_eq!(tree.min(), (Some(&5), Some(&"five")));
    /// ```
    pub fn min(&self) -> (Option<&U>, Option<&T>) {
        match self.left.as_ref() {
            Some(left) => left.min(),
            None => (self.key.as_ref(), self.value.as_ref()),
        }
    }

    /// Returns the maximum key and its associated value in the binary search tree.
    ///
    /// # Returns
    ///
    /// A tuple containing an option with a reference to the maximum key and an option with a reference to the associated value.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_search_tree::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(10, "ten");
    /// tree.insert(5, "five");
    /// tree.insert(15, "fifteen");
    /// assert_eq!(tree.max(), (Some(&15), Some(&"fifteen")));
    /// ```
    pub fn max(&self) -> (Option<&U>, Option<&T>) {
        match self.right.as_ref() {
            Some(right) => right.max(),
            None => (self.key.as_ref(), self.value.as_ref()),
        }
    }
}

impl<U, T> BinarySearchTree<U, T>
where
    U: Copy + Debug + Ord,
    T: Copy + Debug + PartialEq,
{
    /// Performs a pre-order traversal of the binary search tree.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a function to be called on each node's key and value.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_search_tree::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(10, "ten");
    /// tree.insert(5, "five");
    /// tree.insert(15, "fifteen");
    ///
    /// let mut keys_values = vec![];
    /// tree.pre_order(&mut |key, value| keys_values.push((*key, *value)));
    /// assert_eq!(keys_values, vec![(10, "ten"), (5, "five"), (15, "fifteen")]);
    /// ```
    pub fn pre_order(&self, f: &mut dyn FnMut(&U, &T)) {
        if self.is_empty() {
            return;
        }
        f(self.key.as_ref().unwrap(), self.value.as_ref().unwrap());
        self.left.as_ref().map(|node| node.pre_order(f));
        self.right.as_ref().map(|node| node.pre_order(f));
    }

    /// Performs an in-order traversal of the binary search tree.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a function to be called on each node's key and value.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_search_tree::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(10, "ten");
    /// tree.insert(5, "five");
    /// tree.insert(15, "fifteen");
    ///
    /// let mut keys_values = vec![];
    /// tree.in_order(&mut |key, value| keys_values.push((*key, *value)));
    /// assert_eq!(keys_values, vec![(5, "five"), (10, "ten"), (15, "fifteen")]);
    /// ```
    pub fn in_order(&self, f: &mut dyn FnMut(&U, &T)) {
        if self.is_empty() {
            return;
        }
        self.left.as_ref().map(|node| node.in_order(f));
        f(self.key.as_ref().unwrap(), self.value.as_ref().unwrap());
        self.right.as_ref().map(|node| node.in_order(f));
    }

    /// Performs a post-order traversal of the binary search tree.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a function to be called on each node's key and value.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_search_tree::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(10, "ten");
    /// tree.insert(5, "five");
    /// tree.insert(15, "fifteen");
    ///
    /// let mut keys_values = vec![];
    /// tree.post_order(&mut |key, value| keys_values.push((*key, *value)));
    /// assert_eq!(keys_values, vec![(5, "five"), (15, "fifteen"), (10, "ten")]);
    /// ```
    pub fn post_order(&self, f: &mut dyn FnMut(&U, &T)) {
        if self.is_empty() {
            return;
        }
        self.left.as_ref().map(|node| node.post_order(f));
        self.right.as_ref().map(|node| node.post_order(f));
        f(self.key.as_ref().unwrap(), self.value.as_ref().unwrap());
    }

    /// Performs a level-order traversal of the binary search tree.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a function to be called on each node's key and value.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_search_tree::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(10, "ten");
    /// tree.insert(5, "five");
    /// tree.insert(15, "fifteen");
    ///
    /// let mut keys_values = vec![];
    /// tree.level_order(&mut |key, value| keys_values.push((*key, *value)));
    /// assert_eq!(keys_values, vec![(10, "ten"), (5, "five"), (15, "fifteen")]);
    /// ```
    pub fn level_order(&self, f: &mut dyn FnMut(&U, &T)) {
        if self.is_empty() {
            return;
        }
        let mut queue = Queue::new(self.size());
        let _ = queue.enqueue(Box::new(self.clone()));

        while !queue.is_empty() {
            let node = *queue.dequeue().unwrap();

            f(node.key.as_ref().unwrap(), node.value.as_ref().unwrap());

            if let Some(left) = node.left {
                let _ = queue.enqueue(left);
            }
            if let Some(right) = node.right {
                let _ = queue.enqueue(right);
            }
        }
    }
}
