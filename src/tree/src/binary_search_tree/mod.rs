use std::cmp::Ordering::*;
use std::fmt::Debug;

use queue::queue::Queue;

type Link<U, T> = Option<Box<BinarySearchTree<U, T>>>;

#[derive(Debug, Clone)]
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
    pub fn new() -> Self {
        BinarySearchTree {
            key: None,
            value: None,
            left: None,
            right: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.key.is_none()
    }

    pub fn size(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            1 + self.left.as_ref().map_or(0, |node| node.size())
                + self.right.as_ref().map_or(0, |node| node.size())
        }
    }

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

    pub fn none_leaf_size(&self) -> usize {
        self.size() - self.leaf_size()
    }

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

    pub fn min(&self) -> (Option<&U>, Option<&T>) {
        match self.left.as_ref() {
            Some(left) => left.min(),
            None => (self.key.as_ref(), self.value.as_ref()),
        }
    }

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
    T: Copy + Debug,
{
    pub fn pre_order(&self, f: &mut dyn FnMut(&U, &T)) {
        if self.is_empty() {
            return;
        }
        f(self.key.as_ref().unwrap(), self.value.as_ref().unwrap());
        self.left.as_ref().map(|node| node.pre_order(f));
        self.right.as_ref().map(|node| node.pre_order(f));
    }

    pub fn in_order(&self, f: &mut dyn FnMut(&U, &T)) {
        if self.is_empty() {
            return;
        }
        self.left.as_ref().map(|node| node.in_order(f));
        f(self.key.as_ref().unwrap(), self.value.as_ref().unwrap());
        self.right.as_ref().map(|node| node.in_order(f));
    }

    pub fn post_order(&self, f: &mut dyn FnMut(&U, &T)) {
        if self.is_empty() {
            return;
        }
        self.left.as_ref().map(|node| node.post_order(f));
        self.right.as_ref().map(|node| node.post_order(f));
        f(self.key.as_ref().unwrap(), self.value.as_ref().unwrap());
    }

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
