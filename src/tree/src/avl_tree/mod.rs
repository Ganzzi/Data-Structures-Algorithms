use std::cmp::Ordering::*;
use std::mem::replace;

use queue::queue::Queue;

#[derive(Debug, PartialEq)]
pub enum AVLTree<T> {
    Null,
    Tree(Box<AVLNode<T>>),
}

#[derive(Debug, PartialEq)]
pub struct AVLNode<T> {
    key: T,
    left: AVLTree<T>,
    right: AVLTree<T>,
    bfactor: i8,
}

impl<T: Ord> AVLTree<T> {
    pub fn new() -> Self {
        Self::Null
    }

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

    pub fn size(&self) -> usize {
        match self {
            AVLTree::Null => 0,
            AVLTree::Tree(node) => 1 + node.left.size() + node.right.size(),
        }
    }

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

    pub fn non_leaf_size(&self) -> usize {
        self.size() - self.leaf_size()
    }

    pub fn depth(&self) -> usize {
        match self {
            AVLTree::Null => 0,
            AVLTree::Tree(node) => 1 + node.left.depth().max(node.right.depth()),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            AVLTree::Null => true,
            AVLTree::Tree(_) => false,
        }
    }

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
