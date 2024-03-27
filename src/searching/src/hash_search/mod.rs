use hash_map::hash_map::HashMap;


pub fn hash_search(hash_map: &HashMap<i32, usize>, num: i32) -> Option<usize> {
    hash_map.get(&num).cloned()
}