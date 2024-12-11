use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct BloomFilter<T: ?Sized + Hash> {
    bitset: Vec<bool>,
    hashers: [DefaultHasher; 3],
    _phantom: std::marker::PhantomData<T>,
}

impl<T: ?Sized + Hash> BloomFilter<T> {
    pub fn new() -> Self {
        Self {
            bitset: vec![false; 1024],
            hashers: [
                DefaultHasher::new(),
                DefaultHasher::new(),
                DefaultHasher::new(),
            ],
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn insert(&mut self, item: &T) {
        let hash = self.make_hash(item);
        for i in 0..self.hashers.len() {
            let index = self.get_index(hash, i);
            self.bitset[index] = true;
        }
    }

    pub fn contains(&self, item: &T) -> bool {
        let hash = self.make_hash(item);
        for i in 0..self.hashers.len() {
            let index = self.get_index(hash, i);
            if !self.bitset[index] {
                return false;
            }
        }
        true
    }

    fn make_hash(&self, item: &T) -> [u64; 3] {
        let hasher1 = &mut self.hashers[0].clone();
        let hasher2 = &mut self.hashers[1].clone();
        let hasher3 = &mut self.hashers[2].clone();

        item.hash(hasher1);
        item.hash(hasher2);
        item.hash(hasher3);

        [hasher1.finish(), hasher2.finish(), hasher3.finish()]
    }

    fn get_index(&self, hash: [u64; 3], i: usize) -> usize {
        i.wrapping_mul(
            (hash[0] as usize).wrapping_add(
                i.wrapping_mul(hash[1] as usize)
                    .wrapping_add(i.wrapping_mul(hash[2] as usize)),
            ),
        ) % self.bitset.len()
    }
}
