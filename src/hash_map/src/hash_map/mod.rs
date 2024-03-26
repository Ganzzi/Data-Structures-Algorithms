use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct HashMap<K, V> {
    cap: usize,
    slot: Vec<K>,
    data: Vec<V>,
}

impl<K, V> HashMap<K, V>
where
    K: ToString + Default + PartialEq + Clone + Debug,
    V: Clone + PartialEq + Default,
{
    pub fn new(cap: usize) -> Self {
        let mut slot = Vec::with_capacity(cap);
        let mut data = Vec::with_capacity(cap);
        for _ in 0..cap {
            slot.push(Default::default());
            data.push(Default::default());
        }

        HashMap { cap, slot, data }
    }

    pub fn insert(&mut self, key: &K, value: &V) {
        let mut pos = self.hash(&key);

        while self.slot[pos] != Default::default() && self.slot[pos] != *key {
            pos = self.rehash(pos);

            if pos == self.hash(&key) {
                panic!("Hash table is full");
            }
        }

        self.slot[pos] = key.clone();
        self.data[pos] = value.clone();
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut pos = self.hash(key);

        while self.slot[pos] != Default::default() && self.slot[pos] != *key {
            pos = self.rehash(pos);

            if pos == self.hash(&key) {
                return None;
            }
        }

        let removed_value = self.data[pos].clone();
        self.slot[pos] = Default::default();
        self.data[pos] = Default::default();
        Some(removed_value)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut pos = self.hash(key);

        while self.slot[pos] != *key {
            pos = self.rehash(pos);

            if pos == self.hash(&key) {
                return None;
            }
        }

        if self.slot[pos] == *key {
            Some(&self.data[pos])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let mut pos = self.hash(key);

        while self.slot[pos] != *key {
            pos = self.rehash(pos);

            if pos == self.hash(&key) {
                return None;
            }
        }

        if self.slot[pos] == *key {
            Some(&mut self.data[pos])
        } else {
            None
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        let mut pos = self.hash(key);

        while self.slot[pos] != Default::default() && self.slot[pos] != *key {
            pos = self.rehash(pos);

            if pos == self.hash(&key) {
                return false;
            }
        }

        self.slot[pos] == *key
    }

    pub fn is_empty(&self) -> bool {
        self.slot.iter().all(|x| *x == Default::default())
    }

    pub fn len(&self) -> usize {
        self.slot.iter().filter(|&x| *x != Default::default()).count()
    }

    pub fn clear(&mut self) {
        for slot in &mut self.slot {
            *slot = Default::default();
        }
        for data in &mut self.data {
            *data = Default::default();
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.slot
            .iter()
            .zip(&self.data)
            .filter(|(key, _)| key != &&Default::default())
            .map(|(key, value)| (key, value))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.slot
            .iter()
            .zip(&mut self.data)
            .filter_map(|(key, value)| {
                if *key != Default::default() {
                    Some((key, value))
                } else {
                    None
                }
            })
    }

    pub fn index_of(&self, key: &K) -> usize {
        let mut pos = self.hash(key);

        while self.slot[pos] != Default::default() && self.slot[pos] != *key {
            pos = self.rehash(pos);

            if pos == self.hash(&key) {
                panic!("key not found");
            }
        }
        pos
    }

    fn hash(&self, key: &K) -> usize {
        let mut sum_bytes: u32 = 0;
        for byte in key.to_string().bytes() {
            sum_bytes += byte as u32;
        }
        sum_bytes as usize % self.cap
    }

    fn rehash(&self, pos: usize) -> usize {
        (pos + 1) % self.cap
    }
}
