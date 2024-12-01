use std::cmp::Ordering::*;
use std::fmt::Debug;
use std::mem::replace;

use queue::queue::Queue;

/// Represents a node in an AVL tree.
///
/// # Generic Parameters
///
/// * `T` - Type of elements stored in the AVL tree.
#[derive(Debug, PartialEq)]
pub struct AVLNode<T> {
    key: T,
    left: AVLTree<T>,
    right: AVLTree<T>,
    bfactor: i8,
}

/// Represents an AVL tree.
///
/// # Generic Parameters
///
/// * `T` - Type of elements stored in the AVL tree.
///
/// # Examples
///
/// ```
/// use crate::tree::avl_tree::AVLTree;
///
/// let mut tree = AVLTree::new();
/// tree.insert(10);
/// tree.insert(5);
/// tree.insert(15);
///
/// assert_eq!(tree.size(), 3);
/// assert_eq!(tree.leaf_size(), 2);
/// assert_eq!(tree.non_leaf_size(), 1);
/// assert_eq!(tree.depth(), 2);
/// assert_eq!(tree.min(), Some(&5));
/// assert_eq!(tree.max(), Some(&15));
/// assert!(tree.contains(&10));
/// assert!(!tree.contains(&20));
/// ```
#[derive(Debug, PartialEq)]
pub enum AVLTree<T> {
    Null,
    Tree(Box<AVLNode<T>>),
}

impl<T: Ord + Debug + Clone> AVLTree<T> {
    /// Creates a new empty AVL tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::avl_tree::AVLTree;
    ///
    /// let tree: AVLTree<i32> = AVLTree::new();
    /// ```
    pub fn new() -> Self {
        Self::Null
    }

    /// Checks if the AVL tree is balanced.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the AVL tree is balanced.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::avl_tree::AVLTree;
    ///
    /// let mut tree = AVLTree::new();
    /// tree.insert(10);
    /// tree.insert(5);
    /// tree.insert(15);
    /// assert!(tree.is_balanced());
    /// ```
    pub fn is_balanced(&self) -> bool {
        fn check_balance<T: Ord>(tree: &AVLTree<T>) -> (bool, i8) {
            match tree {
                AVLTree::Null => (true, 0),
                AVLTree::Tree(node) => {
                    let (left_balanced, left_height) = check_balance(&node.left);
                    let (right_balanced, right_height) = check_balance(&node.right);
                    let balanced =
                        left_balanced && right_balanced && (left_height - right_height).abs() <= 1;
                    let height = 1 + left_height.max(right_height);
                    (balanced, height)
                }
            }
        }
        check_balance(self).0
    }

    /// Inserts a key into the AVL tree.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to be inserted.
    ///
    /// # Returns
    ///
    /// A tuple containing two boolean values:
    /// - The first boolean indicates whether the key was inserted.
    /// - The second boolean indicates whether the height of the tree increased.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::avl_tree::AVLTree;
    ///
    /// let mut tree = AVLTree::new();
    /// tree.insert(10);
    /// tree.insert(5);
    /// tree.insert(15);
    /// ```
    pub fn insert(&mut self, key: T) -> (bool, bool) {
        let inserted_increased_tupple = match self {
            Self::Null => {
                let node = AVLNode {
                    key,
                    left: Self::Null,
                    right: Self::Null,
                    bfactor: 0,
                };
                *self = Self::Tree(Box::new(node));
                (true, true)
            }
            Self::Tree(ref mut node) => match node.key.cmp(&key) {
                Equal => (false, false),
                Less => {
                    let (inserted, increased) = node.right.insert(key);

                    if !increased {
                        (inserted, false)
                    } else {
                        let increased = match node.bfactor {
                            0 => true,
                            _ => false,
                        };

                        node.bfactor += 1;
                        (inserted, increased)
                    }
                }
                Greater => {
                    let (inserted, increased) = node.left.insert(key);

                    if !increased {
                        (inserted, false)
                    } else {
                        let increased = match node.bfactor {
                            0 => true,
                            _ => false,
                        };

                        node.bfactor -= 1;
                        (inserted, increased)
                    }
                }
            },
        };

        self.rebalance();

        inserted_increased_tupple
    }

    fn rebalance(&mut self) {
        match self {
            Self::Null => (),
            Self::Tree(_) => match self.node().bfactor {
                -2 => {
                    let lbf = self.left_subtree().node().bfactor;

                    match lbf {
                        1 => {
                            let (a, b) = match self.left_subtree().right_subtree().node().bfactor {
                                -1 => (1, 0),
                                0 => (0, 0),
                                1 => (0, -1),
                                _ => unreachable!(),
                            };

                            self.left_subtree().rotate_left();
                            self.rotate_right();

                            self.right_subtree().node().bfactor = a;
                            self.left_subtree().node().bfactor = b;
                            self.node().bfactor = 0;
                        }
                        _ => {
                            let (a, b) = if lbf == -1 { (0, 0) } else { (-1, 1) };

                            self.rotate_right();

                            self.right_subtree().node().bfactor = a;
                            self.node().bfactor = b;
                        }
                    }
                }
                2 => {
                    let rbf = self.right_subtree().node().bfactor;
                    match rbf {
                        -1 => {
                            let (a, b) = match self.right_subtree().left_subtree().node().bfactor {
                                1 => (-1, 0),
                                0 => (0, 0),
                                -1 => (0, 1),
                                _ => unreachable!(),
                            };

                            self.right_subtree().rotate_right();
                            self.rotate_left();

                            self.left_subtree().node().bfactor = a;
                            self.right_subtree().node().bfactor = b;
                            self.node().bfactor = 0;
                        }
                        _ => {
                            let (a, b) = if rbf == 1 { (0, 0) } else { (1, -1) };

                            self.rotate_left();

                            self.left_subtree().node().bfactor = a;
                            self.node().bfactor = b;
                        }
                    }
                }
                _ => (),
            },
        }
    }

    fn rotate_left(&mut self) {
        let mut root = replace(self, Self::Null);
        let mut right = replace(root.right_subtree(), Self::Null);
        let right_then_left = replace(right.left_subtree(), Self::Null);

        *root.right_subtree() = right_then_left;
        *right.left_subtree() = root;
        *self = right;
    }

    fn rotate_right(&mut self) {
        let mut root = replace(self, Self::Null);
        let mut left = replace(root.left_subtree(), Self::Null);
        let left_then_right = replace(left.right_subtree(), Self::Null);

        *root.left_subtree() = left_then_right;
        *left.right_subtree() = root;
        *self = left;
    }

    fn node(&mut self) -> &mut AVLNode<T> {
        match self {
            Self::Null => panic!("Trying to get node from empty tree!"),
            Self::Tree(node) => node,
        }
    }

    fn left_subtree(&mut self) -> &mut Self {
        match self {
            Self::Null => panic!("Trying to get left subtree from empty tree!"),
            Self::Tree(node) => &mut node.left,
        }
    }

    fn right_subtree(&mut self) -> &mut Self {
        match self {
            Self::Null => panic!("Trying to get right subtree from empty tree!"),
            Self::Tree(node) => &mut node.right,
        }
    }

    /// Returns the size of the AVL tree.
    ///
    /// # Returns
    ///
    /// The number of nodes in the AVL tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::avl_tree::AVLTree;
    ///
    /// let mut tree = AVLTree::new();
    /// tree.insert(10);
    /// tree.insert(5);
    /// tree.insert(15);
    /// assert_eq!(tree.size(), 3);
    /// ```
    pub fn size(&self) -> usize {
        match self {
            AVLTree::Null => 0,
            AVLTree::Tree(node) => 1 + node.left.size() + node.right.size(),
        }
    }

    /// Returns the number of leaf nodes in the AVL tree.
    ///
    /// # Returns
    ///
    /// The number of leaf nodes in the AVL tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::avl_tree::AVLTree;
    ///
    /// let mut tree = AVLTree::new();
    /// tree.insert(10);
    /// tree.insert(5);
    /// tree.insert(15);
    /// assert_eq!(tree.leaf_size(), 2);
    /// ```
    pub fn leaf_size(&self) -> usize {
        match self {
            AVLTree::Null => 0,
            AVLTree::Tree(node) => {
                if node.left == AVLTree::Null && node.right == AVLTree::Null {
                    1
                } else {
                    let left_leaf_size = match node.left {
                        AVLTree::Null => 0,
                        AVLTree::Tree(_) => node.left.leaf_size(),
                    };
                    let right_leaf_size = match node.right {
                        AVLTree::Null => 0,
                        AVLTree::Tree(_) => node.right.leaf_size(),
                    };

                    left_leaf_size + right_leaf_size
                }
            }
        }
    }

    /// Returns the number of non-leaf nodes in the AVL tree.
    ///
    /// # Returns
    ///
    /// The number of non-leaf nodes in the AVL tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::avl_tree::AVLTree;
    ///
    /// let mut tree = AVLTree::new();
    /// tree.insert(10);
    /// tree.insert(5);
    /// tree.insert(15);
    /// assert_eq!(tree.non_leaf_size(), 1);
    /// ```
    pub fn non_leaf_size(&self) -> usize {
        self.size() - self.leaf_size()
    }

    /// Returns the depth of the AVL tree.
    ///
    /// # Returns
    ///
    /// The depth of the AVL tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::avl_tree::AVLTree;
    ///
    /// let mut tree = AVLTree::new();
    /// tree.insert(10);
    /// tree.insert(5);
    /// tree.insert(15);
    /// assert_eq!(tree.depth(), 2);
    /// ```
    pub fn depth(&self) -> usize {
        match self {
            AVLTree::Null => 0,
            AVLTree::Tree(node) => 1 + node.left.depth().max(node.right.depth()),
        }
    }

    /// Checks if the AVL tree is empty.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the AVL tree is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::avl_tree::AVLTree;
    ///
    /// let tree: AVLTree<i32> = AVLTree::new();
    /// assert!(tree.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        match self {
            AVLTree::Null => true,
            AVLTree::Tree(_) => false,
        }
    }

    /// Returns the minimum key in the AVL tree.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the minimum key, or None if the tree is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::avl_tree::AVLTree;
    ///
    /// let mut tree = AVLTree::new();
    /// tree.insert(10);
    /// tree.insert(5);
    /// tree.insert(15);
    /// assert_eq!(tree.min(), Some(&5));
    /// ```
    pub fn min(&self) -> Option<&T> {
        match self {
            AVLTree::Null => None,
            AVLTree::Tree(node) => {
                if node.left.is_empty() {
                    Some(&node.key)
                } else {
                    node.left.min()
                }
            }
        }
    }

    /// Returns the maximum key in the AVL tree.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the maximum key, or None if the tree is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::avl_tree::AVLTree;
    ///
    /// let mut tree = AVLTree::new();
    /// tree.insert(10);
    /// tree.insert(5);
    /// tree.insert(15);
    /// assert_eq!(tree.max(), Some(&15));
    /// ```
    pub fn max(&self) -> Option<&T> {
        match self {
            AVLTree::Null => None,
            AVLTree::Tree(node) => {
                if node.right.is_empty() {
                    Some(&node.key)
                } else {
                    node.right.max()
                }
            }
        }
    }

    /// Checks if the AVL tree contains the specified key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check for.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the AVL tree contains the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::avl_tree::AVLTree;
    ///
    /// let mut tree = AVLTree::new();
    /// tree.insert(10);
    /// tree.insert(5);
    /// tree.insert(15);
    /// assert!(tree.contains(&10));
    /// assert!(tree.contains(&5));
    /// assert!(tree.contains(&15));
    /// assert!(!tree.contains(&20));
    /// ```
    pub fn contains(&self, key: &T) -> bool {
        match self {
            AVLTree::Null => false,
            AVLTree::Tree(node) => match node.key.cmp(key) {
                Equal => true,
                Greater => node.left.contains(key),
                Less => node.right.contains(key),
            },
        }
    }

    /// Performs a pre-order traversal of the AVL tree.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a function to be called on each node's key.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::avl_tree::AVLTree;
    /// use vec::linked_vec;
    ///
    /// let mut tree = AVLTree::new();
    /// tree.insert(10);
    /// tree.insert(5);
    /// tree.insert(15);
    ///
    /// let mut keys = linked_vec![];
    /// tree.pre_order(&mut |key| keys.push(*key));
    /// assert_eq!(keys, linked_vec![10, 5, 15]);
    /// ```
    pub fn pre_order(&self, f: &mut dyn FnMut(&T)) {
        match self {
            AVLTree::Null => (),
            AVLTree::Tree(node) => {
                f(&node.key);
                node.left.pre_order(f);
                node.right.pre_order(f);
            }
        }
    }

    /// Performs an in-order traversal of the AVL tree.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a function to be called on each node's key.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::avl_tree::AVLTree;
    /// use vec::linked_vec;
    ///
    /// let mut tree = AVLTree::new();
    /// tree.insert(10);
    /// tree.insert(5);
    /// tree.insert(15);
    ///
    /// let mut keys = linked_vec![];
    /// tree.in_order(&mut |key| keys.push(*key));
    /// assert_eq!(keys, linked_vec![5, 10, 15]);
    /// ```
    pub fn in_order(&self, f: &mut dyn FnMut(&T)) {
        match self {
            AVLTree::Null => (),
            AVLTree::Tree(node) => {
                node.left.in_order(f);
                f(&node.key);
                node.right.in_order(f);
            }
        }
    }

    /// Performs a post-order traversal of the AVL tree.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a function to be called on each node's key.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::avl_tree::AVLTree;
    /// use vec::linked_vec;
    ///
    /// let mut tree = AVLTree::new();
    /// tree.insert(10);
    /// tree.insert(5);
    /// tree.insert(15);
    ///
    /// let mut keys = linked_vec![];
    /// tree.post_order(&mut |key| keys.push(*key));
    /// assert_eq!(keys, linked_vec![5, 15, 10]);
    /// ```
    pub fn post_order(&self, f: &mut dyn FnMut(&T)) {
        match self {
            AVLTree::Null => (),
            AVLTree::Tree(node) => {
                node.left.post_order(f);
                node.right.post_order(f);
                f(&node.key);
            }
        }
    }

    /// Performs a level-order traversal of the AVL tree.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a function to be called on each node's key.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tree::avl_tree::AVLTree;
    /// use vec::linked_vec;
    ///
    /// let mut tree = AVLTree::new();
    /// tree.insert(10);
    /// tree.insert(5);
    /// tree.insert(15);
    ///
    /// let mut keys = linked_vec![];
    /// tree.level_order(&mut |key| keys.push(*key));
    /// assert_eq!(keys, linked_vec![10, 5, 15]);
    /// ```
    pub fn level_order(&self, f: &mut dyn FnMut(&T)) {
        if self.is_empty() {
            return;
        }

        let mut queue = Queue::new(self.size());
        let _ = queue.enqueue(self);

        while !queue.is_empty() {
            let node = queue.dequeue().unwrap();

            match node {
                AVLTree::Null => (),
                AVLTree::Tree(node) => {
                    f(&node.key);
                    let _ = queue.enqueue(&node.left);
                    let _ = queue.enqueue(&node.right);
                }
            }
        }
    }
}
