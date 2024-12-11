use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

use hash_map::hash_map::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    pub data: String,
}

impl ToString for Node {
    fn to_string(&self) -> String {
        self.data.clone()
    }
}

pub struct Ring<T> {
    replicas: usize,
    nodes: HashMap<u64, T>,
}

fn hash<T: Hash>(key: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

const DEFAULT_REPLICAS: usize = 10;

impl<T: Clone + PartialEq + Debug + ToString> Ring<T> {
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_REPLICAS)
    }

    pub fn with_capacity(replicas: usize) -> Self {
        Ring {
            replicas,
            nodes: HashMap::new(),
        }
    }

    pub fn add_multiple(&mut self, keys: &[T]) {
        for key in keys {
            self.add(key);
        }
    }

    pub fn add(&mut self, node: &T) {
        for i in 0..self.replicas {
            let hash_key = hash(format!("{}-{}", node.to_string(), i));
            self.nodes.insert(hash_key, node.clone());
        }
    }

    pub fn remove_multiple(&mut self, keys: &[T]) {
        for key in keys {
            self.remove(key.clone());
        }
    }

    pub fn remove(&mut self, node: T) {
        for i in 0..self.replicas {
            let hash_key = hash(format!("{}-{}", node.to_string(), i));
            self.nodes.remove(&hash_key);
        }
    }

    pub fn get(&self, node: T) -> Option<&T> {
        for i in 0..self.replicas {
            let hash_key = hash(format!("{}-{}", node.to_string(), i));
            if let Some(n) = self.nodes.get(&hash_key) {
                return Some(n);
            }
        }
        None
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len() / self.replicas
    }
}
