use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq)]
pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq + Clone + Debug,
    V: Clone + PartialEq,
{
    pub fn with_capacity(capacity: usize) -> Self {
        let buckets = vec![Vec::new(); capacity + 1];
        HashMap { buckets, size: 0 }
    }

    pub fn new() -> Self {
        let initial_capacity = 16;
        let buckets = vec![Vec::new(); initial_capacity];
        HashMap { buckets, size: 0 }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.size >= self.buckets.len() {
            self.resize();
        }

        let bucket_index = self.hash(&key);
        let bucket = &mut self.buckets[bucket_index];

        for &mut (ref existing_key, ref mut existing_value) in bucket.iter_mut() {
            if existing_key == &key {
                *existing_value = value;
                return;
            }
        }

        bucket.push((key, value));
        self.size += 1;
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let bucket_index = self.hash(key);
        let bucket = &mut self.buckets[bucket_index];

        if let Some(pos) = bucket
            .iter()
            .position(|(existing_key, _)| existing_key == key)
        {
            let (_, value) = bucket.remove(pos);
            self.size -= 1;
            return Some(value);
        }

        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket_index = self.hash(key);
        let bucket = &self.buckets[bucket_index];

        for (existing_key, value) in bucket.iter() {
            if existing_key == key {
                return Some(value);
            }
        }

        None
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let bucket_index = self.hash(key);
        let bucket = &mut self.buckets[bucket_index];

        for (existing_key, value) in bucket.iter_mut() {
            if existing_key == key {
                return Some(value);
            }
        }

        None
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        for bucket in &mut self.buckets {
            bucket.clear();
        }
        self.size = 0;
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.buckets
            .iter()
            .flat_map(|bucket| bucket.iter())
            .map(|(key, value)| (key, value))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut K, &mut V)> {
        self.buckets
            .iter_mut()
            .flat_map(|bucket| bucket.iter_mut())
            .map(|(key, value)| (key, value))
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets.len()
    }

    fn resize(&mut self) {
        let new_capacity = self.buckets.len() * 2;
        let mut new_buckets = vec![Vec::new(); new_capacity];

        for bucket in self.buckets.drain(..) {
            for (key, value) in bucket {
                let new_bucket_index = {
                    let mut hasher: DefaultHasher = DefaultHasher::new();
                    key.hash(&mut hasher);
                    (hasher.finish() as usize) % new_capacity
                };
                new_buckets[new_bucket_index].push((key, value));
            }
        }

        self.buckets = new_buckets;
    }
}
