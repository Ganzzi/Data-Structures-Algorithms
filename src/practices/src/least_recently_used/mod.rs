use hash_map::hash_map::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use vec::vec::LinkedVec;

/// Represents an entry in the LRU cache.
///
/// # Generic Parameters
///
/// * `K` - Type of keys stored in the cache.
/// * `V` - Type of values stored in the cache.
#[derive(Debug, Clone, PartialEq)]
pub struct Entry<K, V> {
    key: K,
    value: Option<V>,
}

/// Represents a Least Recently Used (LRU) cache.
///
/// # Generic Parameters
///
/// * `K` - Type of keys stored in the cache.
/// * `V` - Type of values stored in the cache.
pub struct LRUCache<K, V> {
    cap: usize,
    map: HashMap<K, usize>,
    entries: LinkedVec<Entry<K, V>>,
}

impl<K: Hash + Eq + Clone + Debug, V: Clone + PartialEq + Debug> LRUCache<K, V> {
    /// Creates a new LRU cache with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The maximum number of elements the cache can hold.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::lru_cache::LRUCache;
    ///
    /// let cache: LRUCache<i32, &str> = LRUCache::new(2);
    /// ```
    pub fn new(cap: usize) -> Self {
        LRUCache {
            cap,
            map: HashMap::new(),
            entries: LinkedVec::new(),
        }
    }

    /// Inserts a key-value pair into the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to be inserted.
    /// * `value` - The value to be inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::lru_cache::LRUCache;
    ///
    /// let mut cache = LRUCache::new(2);
    /// cache.insert(1, "one");
    /// cache.insert(2, "two");
    /// ```
    pub fn insert(&mut self, key: K, value: V) {
        if self.contains(&key.clone()) {
            let index = self.map.get(&key).unwrap();
            let entry = self.entries[*index].clone();
            let mut entry = entry;
            entry.value = Some(value);
            self.entries.remove(*index);
            self.entries.push(entry);
            self.map.insert(key, self.entries.len() - 1);
        } else {
            if self.entries.len() == self.cap {
                let entry = self.entries.remove(0);
                self.map.remove(&entry.unwrap().key);
            }
            let entry = Entry {
                key: key.clone(),
                value: Some(value),
            };
            self.entries.push(entry);
            self.map.insert(key, self.entries.len() - 1);
        }
    }

    /// Removes a key-value pair from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to be removed.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::lru_cache::LRUCache;
    ///
    /// let mut cache = LRUCache::new(2);
    /// cache.insert(1, "one");
    /// cache.remove(&1);
    /// assert_eq!(cache.get(&1), None);
    /// ```
    pub fn remove(&mut self, key: &K) {
        if self.contains(key) {
            let index = self.map.get(key).unwrap();
            self.entries.remove(*index);
            self.map.remove(key);
        }
    }

    /// Checks if the cache contains a key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check for existence.
    ///
    /// # Returns
    ///
    /// `true` if the key exists in the cache, `false` otherwise.
    pub fn contains(&self, key: &K) -> bool {
        self.map.contains(key)
    }

    /// Retrieves a value from the cache by key.
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
    /// use crate::lru_cache::LRUCache;
    ///
    /// let mut cache = LRUCache::new(2);
    /// cache.insert(1, "one");
    /// assert_eq!(cache.get(&1), Some(&"one"));
    /// assert_eq!(cache.get(&2), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<V> {
        if self.contains(key) {
            let index = self.map.get(key).unwrap();
            let entry = self.entries[*index].clone();
            return entry.value;
        }
        None
    }

    /// Retrieves a mutable reference to a value from the cache by key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up.
    ///
    /// # Returns
    ///
    /// An option containing a mutable reference to the value, or None if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::lru_cache::LRUCache;
    ///
    /// let mut cache = LRUCache::new(2);
    /// cache.insert(1, "one");
    /// let value = cache.get_mut(&1);
    /// assert_eq!(value, Some(&mut "one"));
    /// value.map(|v| *v = "two");
    /// assert_eq!(cache.get(&1), Some(&"two"));
    /// ```
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.contains(key) {
            let index = self.map.get(key).unwrap();
            let entry = &mut self.entries[*index];
            return entry.value.as_mut();
        }
        None
    }

    /// Get the number of elements in the cache.
    ///
    /// # Returns
    ///
    /// The number of elements in the cache.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::lru_cache::LRUCache;
    ///
    /// let mut cache = LRUCache::new(2);
    /// cache.insert(1, "one");
    /// cache.insert(2, "two");
    /// assert_eq!(cache.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}
