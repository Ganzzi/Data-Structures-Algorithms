use std::{
    cmp::Ordering::{Equal, Greater, Less},
    fmt::Debug,
};

use queue::queue::Queue;

/// Represents a link to a binary tree node.
type Link<T> = Option<Box<BinaryTree<T>>>;

/// Represents a binary tree.
///
/// # Generic Parameters
///
/// * `T` - Type of elements stored in the binary tree.
///
/// # Examples
///
/// ```
/// use tree::binary_tree::BinaryTree;
///
/// let mut tree = BinaryTree::new(5);
/// tree.push(&3);
/// tree.push(&7);
/// tree.push(&2);
///
/// assert_eq!(tree.size(), 4);
/// assert_eq!(tree.leaf_size(), 2);
/// assert_eq!(tree.none_leaf_size(), 2);
/// assert_eq!(tree.depth(), 3);
/// assert_eq!(tree.min_key(), Some(&2));
/// assert_eq!(tree.max_key(), Some(&7));
/// assert_eq!(tree.contains(&7), true);
/// assert_eq!(tree.contains(&10), false);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryTree<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Clone + Ord + Debug> BinaryTree<T> {
    /// Creates a new binary tree with the specified key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the root node.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_tree::BinaryTree;
    ///
    /// let tree = BinaryTree::new(10);
    /// ```
    pub fn new(key: T) -> Self {
        BinaryTree {
            key,
            left: None,
            right: None,
        }
    }

    /// Inserts a key into the binary tree.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to be inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new(10);
    /// tree.push(&5);
    /// tree.push(&15);
    /// ```
    pub fn push(&mut self, key: &T) {
        match self.key.cmp(key) {
            Equal => (),
            Less => {
                if let Some(ref mut right_node) = self.right {
                    right_node.push(key);
                } else {
                    self.right = Some(Box::new(BinaryTree::new(key.clone())));
                }
            }
            Greater => {
                if let Some(ref mut left_node) = self.left {
                    left_node.push(key);
                } else {
                    self.left = Some(Box::new(BinaryTree::new(key.clone())));
                }
            }
        }
    }

    /// Returns the size of the binary tree.
    ///
    /// # Returns
    ///
    /// The number of nodes in the binary tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new(10);
    /// tree.push(&5);
    /// tree.push(&15);
    /// assert_eq!(tree.size(), 3);
    /// ```
    pub fn size(&self) -> usize {
        self.calc_size(0)
    }

    fn calc_size(&self, mut size: usize) -> usize {
        size += 1;

        if self.left.is_some() {
            size = self.left.as_ref().unwrap().calc_size(size);
        }
        if self.right.is_some() {
            size = self.right.as_ref().unwrap().calc_size(size);
        }

        size
    }

    /// Returns the number of leaf nodes in the binary tree.
    ///
    /// # Returns
    ///
    /// The number of leaf nodes in the binary tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new(10);
    /// tree.push(&5);
    /// tree.push(&15);
    /// assert_eq!(tree.leaf_size(), 2);
    /// ```
    pub fn leaf_size(&self) -> usize {
        if self.left.is_none() && self.right.is_none() {
            1
        } else {
            let left_nodes = match self.left {
                Some(ref node) => node.leaf_size(),
                None => 0,
            };

            let right_nodes = match self.right {
                Some(ref node) => node.leaf_size(),
                None => 0,
            };

            left_nodes + right_nodes
        }
    }

    /// Returns the number of non-leaf nodes in the binary tree.
    ///
    /// # Returns
    ///
    /// The number of non-leaf nodes in the binary tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new(10);
    /// tree.push(&5);
    /// tree.push(&15);
    /// assert_eq!(tree.none_leaf_size(), 1);
    /// ```
    pub fn none_leaf_size(&self) -> usize {
        self.size() - self.leaf_size()
    }

    /// Returns the depth of the binary tree.
    ///
    /// # Returns
    ///
    /// The depth of the binary tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new(10);
    /// tree.push(&5);
    /// tree.push(&15);
    /// assert_eq!(tree.depth(), 2);
    /// ```
    pub fn depth(&self) -> usize {
        let mut left_depth = 1;
        if let Some(left) = &self.left {
            left_depth += left.depth();
        }

        let mut right_depth = 1;
        if let Some(right) = &self.right {
            right_depth += right.depth();
        }

        left_depth.max(right_depth)
    }

    /// Returns a clone of the left subtree.
    ///
    /// # Returns
    ///
    /// A clone of the left subtree.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new(10);
    /// tree.push(&5);
    /// let left_subtree = tree.get_left();
    /// ```
    pub fn get_left(&self) -> Link<T> {
        self.left.clone()
    }

    /// Returns a clone of the right subtree.
    ///
    /// # Returns
    ///
    /// A clone of the right subtree.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new(10);
    /// tree.push(&15);
    /// let right_subtree = tree.get_right();
    /// ```
    pub fn get_right(&self) -> Link<T> {
        self.right.clone()
    }

    /// Returns a reference to the key of the root node.
    ///
    /// # Returns
    ///
    /// A reference to the key of the root node.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_tree::BinaryTree;
    ///
    /// let tree = BinaryTree::new(10);
    /// assert_eq!(tree.get_key(), &10);
    /// ```
    pub fn get_key(&self) -> &T {
        &self.key
    }

    /// Sets the key of the root node.
    ///
    /// # Arguments
    ///
    /// * `key` - The new key of the root node.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new(10);
    /// tree.set_key(20);
    /// assert_eq!(tree.get_key(), &20);
    /// ```
    pub fn set_key(&mut self, key: T) {
        self.key = key;
    }

    /// Returns the minimum key in the binary tree.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the minimum key, or None if the tree is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new(10);
    /// tree.push(&5);
    /// tree.push(&15);
    /// assert_eq!(tree.min_key(), Some(&5));
    /// ```
    pub fn min_key(&self) -> Option<&T> {
        match self.left {
            None => Some(&self.key),
            Some(ref node) => node.min_key(),
        }
    }

    /// Returns the maximum key in the binary tree.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the maximum key, or None if the tree is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new(10);
    /// tree.push(&5);
    /// tree.push(&15);
    /// assert_eq!(tree.max_key(), Some(&15));
    /// ```
    pub fn max_key(&self) -> Option<&T> {
        match self.right {
            None => Some(&self.key),
            Some(ref node) => node.max_key(),
        }
    }

    /// Checks if the binary tree contains the specified key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check for.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the binary tree contains the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new(10);
    /// tree.push(&5);
    /// tree.push(&15);
    /// assert!(tree.contains(&10));
    /// assert!(tree.contains(&5));
    /// assert!(tree.contains(&15));
    /// assert!(!tree.contains(&20));
    /// ```
    pub fn contains(&self, key: &T) -> bool {
        match self.key.cmp(key) {
            Equal => true,
            Less => {
                if let Some(ref right_node) = self.right {
                    right_node.contains(key)
                } else {
                    false
                }
            }
            Greater => {
                if let Some(ref left_node) = self.left {
                    left_node.contains(key)
                } else {
                    false
                }
            }
        }
    }

    /// Performs a pre-order traversal of the binary tree.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a function to be called on each node's key.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new(10);
    /// tree.push(&5);
    /// tree.push(&15);
    ///
    /// let mut keys = vec![];
    /// tree.pre_order(&mut |key| keys.push(*key));
    /// assert_eq!(keys, vec![10, 5, 15]);
    /// ```
    pub fn pre_order(&self, f: &mut dyn FnMut(&T)) {
        f(&self.key);
        if let Some(ref left_node) = self.left {
            left_node.pre_order(f);
        }
        if let Some(ref right_node) = self.right {
            right_node.pre_order(f);
        }
    }

    /// Performs a post-order traversal of the binary tree.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a function to be called on each node's key.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new(10);
    /// tree.push(&5);
    /// tree.push(&15);
    ///
    /// let mut keys = vec![];
    /// tree.post_order(&mut |key| keys.push(*key));
    /// assert_eq!(keys, vec![5, 15, 10]);
    /// ```
    pub fn post_order(&self, f: &mut dyn FnMut(&T)) {
        if let Some(ref left_node) = self.left {
            left_node.post_order(f);
        }
        if let Some(ref right_node) = self.right {
            right_node.post_order(f);
        }
        f(&self.key);
    }

    /// Performs a in-order traversal of the binary tree.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a function to be called on each node's key.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new(10);
    /// tree.push(&5);
    /// tree.push(&15);
    ///
    /// let mut keys = vec![];
    /// tree.in_order(&mut |key| keys.push(*key));
    /// assert_eq!(keys, vec![5, 10, 15]);
    /// ```
    pub fn in_order(&self, f: &mut dyn FnMut(&T)) {
        if let Some(ref left_node) = self.left {
            left_node.in_order(f);
        }
        f(&self.key);
        if let Some(ref right_node) = self.right {
            right_node.in_order(f);
        }
    }

    /// Performs a level-order traversal of the binary tree.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a function to be called on each node's key.
    ///
    /// # Examples
    ///
    /// ```
    /// use tree::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new(10);
    /// tree.push(&5);
    /// tree.push(&15);
    ///
    /// let mut keys = vec![];
    /// tree.level_order(&mut |key| keys.push(*key));
    /// assert_eq!(keys, vec![10, 5, 15]);
    /// ```
    pub fn level_order(&self, f: &mut dyn FnMut(&T)) {
        let mut queue = Queue::new(self.size());
        let _ = queue.enqueue(Box::new(self.clone()));

        while !queue.is_empty() {
            let node = queue.dequeue().unwrap();
            let node = *node;
            f(node.get_key());

            if let Some(left) = node.left {
                let _ = queue.enqueue(left);
            }
            if let Some(right) = node.right {
                let _ = queue.enqueue(right);
            }
        }
    }
}
