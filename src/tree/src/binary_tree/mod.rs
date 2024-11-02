use std::{
    cmp::Ordering::{Equal, Greater, Less},
    fmt::Debug,
};

use queue::queue::Queue;

type Link<T> = Option<Box<BinaryTree<T>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryTree<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Clone + Ord + Debug> BinaryTree<T> {
    pub fn new(key: T) -> Self {
        BinaryTree {
            key,
            left: None,
            right: None,
        }
    }

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

    pub fn none_leaf_size(&self) -> usize {
        self.size() - self.leaf_size()
    }

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

    pub fn get_left(&self) -> Link<T> {
        self.left.clone()
    }

    pub fn get_right(&self) -> Link<T> {
        self.right.clone()
    }

    pub fn get_key(&self) -> &T {
        &self.key
    }

    pub fn set_key(&mut self, key: T) {
        self.key = key;
    }

    pub fn min_key(&self) -> Option<&T> {
        match self.left {
            None => Some(&self.key),
            Some(ref node) => node.min_key(),
        }
    }

    pub fn maxkey(&self) -> Option<&T> {
        match self.right {
            None => Some(&self.key),
            Some(ref node) => node.maxkey(),
        }
    }

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

    pub fn pre_order(&self, f: &mut dyn FnMut(&T)) {
        f(&self.key);
        if let Some(ref left_node) = self.left {
            left_node.pre_order(f);
        }
        if let Some(ref right_node) = self.right {
            right_node.pre_order(f);
        }
    }

    pub fn post_order(&self, f: &mut dyn FnMut(&T)) {
        if let Some(ref left_node) = self.left {
            left_node.post_order(f);
        }
        if let Some(ref right_node) = self.right {
            right_node.post_order(f);
        }
        f(&self.key);
    }

    pub fn in_order(&self, f: &mut dyn FnMut(&T)) {
        if let Some(ref left_node) = self.left {
            left_node.in_order(f);
        }
        f(&self.key);
        if let Some(ref right_node) = self.right {
            right_node.in_order(f);
        }
    }

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
