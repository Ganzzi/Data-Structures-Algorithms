use hash_map::hash_map::HashMap;

/// Searches for a given key in a hash map and returns the associated value if found.
///
/// # Arguments
///
/// * `hash_map` - A reference to a `HashMap` where the search will be performed.
/// * `num` - The key to search for in the hash map.
///
/// # Returns
///
/// * `Option<usize>` - Returns `Some(usize)` containing the value associated with the key if found,
///   otherwise returns `None`.
///
/// # Examples
///
/// ```
/// use hash_map::hash_map::HashMap;
/// use crate::searching::hash_search::hash_search;
///
/// let mut map = HashMap::new();
/// map.insert(1, 10);
/// map.insert(2, 20);
///
/// assert_eq!(hash_search(&map, 1), Some(10));
/// assert_eq!(hash_search(&map, 3), None);
/// ```
pub fn hash_search(hash_map: &HashMap<i32, usize>, num: i32) -> Option<usize> {
    hash_map.get(&num).cloned()
}
