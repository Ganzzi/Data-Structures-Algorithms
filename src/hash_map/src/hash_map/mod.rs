use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use vec::linked_vec;
use vec::vec::LinkedVec;

/// A simple hash map implementation using a linked vector for buckets.
///
/// # Examples
///
/// ```
/// use crate::hash_map::hash_map::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("key1", "value1");
/// map.insert("key2", "value2");
///
/// assert_eq!(map.get(&"key1"), Some(&"value1"));
/// assert_eq!(map.get(&"key2"), Some(&"value2"));
/// assert_eq!(map.get(&"key3"), None);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct HashMap<K, V> {
    buckets: LinkedVec<LinkedVec<(K, V)>>,
    size: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq + Clone + Debug,
    V: Clone + PartialEq + Debug,
{
    /// Creates a new hash map with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The initial capacity of the hash map.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::hash_map::hash_map::HashMap;
    ///
    /// let map: HashMap<&str, &str> = HashMap::with_capacity(10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        let buckets = linked_vec![LinkedVec::new(); capacity + 1];
        HashMap { buckets, size: 0 }
    }

    /// Creates a new hash map with an initial capacity of 16.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::hash_map::hash_map::HashMap;
    ///
    /// let map: HashMap<&str, &str> = HashMap::new();
    /// ```
    pub fn new() -> Self {
        let initial_capacity = 16;
        let buckets = linked_vec![LinkedVec::new(); initial_capacity];
        HashMap { buckets, size: 0 }
    }

    /// Inserts a key-value pair into the hash map.
    ///
    /// If the key already exists, the value is updated.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to insert.
    /// * `value` - The value to insert.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::hash_map::hash_map::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("key1", "value1");
    /// map.insert("key2", "value2");
    ///
    /// assert_eq!(map.get(&"key1"), Some(&"value1"));
    /// assert_eq!(map.get(&"key2"), Some(&"value2"));
    /// ```
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

    /// Removes a key-value pair from the hash map.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove.
    ///
    /// # Returns
    ///
    /// An option containing the removed value, or None if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::hash_map::hash_map::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("key1", "value1");
    /// assert_eq!(map.remove(&"key1"), Some("value1"));
    /// assert_eq!(map.remove(&"key1"), None);
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let bucket_index = self.hash(key);
        let bucket = &mut self.buckets[bucket_index];

        if let Some(pos) = bucket
            .iter()
            .position(|(existing_key, _)| existing_key == key)
        {
            let (_, value) = bucket.remove(pos).unwrap();
            self.size -= 1;
            return Some(value);
        }

        None
    }

    /// Returns a reference to the value corresponding to the key.
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
    /// use crate::hash_map::hash_map::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("key1", "value1");
    ///
    /// assert_eq!(map.get(&"key1"), Some(&"value1"));
    /// assert_eq!(map.get(&"key2"), None);
    /// ```
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
        for bucket in &mut self.buckets.iter_mut() {
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
        let mut new_buckets = linked_vec![LinkedVec::new(); new_capacity];

        for bucket in self.buckets.drain().into_iter() {
            for (key, value) in bucket.iter() {
                let new_bucket_index = {
                    let mut hasher: DefaultHasher = DefaultHasher::new();
                    key.hash(&mut hasher);
                    (hasher.finish() as usize) % new_capacity
                };
                new_buckets[new_bucket_index].push((key.clone(), value.clone()));
            }
        }

        self.buckets = new_buckets;
    }
}
