use hash_map::hash_map::HashMap;

/// Searches for a key-value pair in the given `HashMap` based on the provided key.
///
/// # Arguments
///
/// * `hash_map` - A reference to the `HashMap` to be searched.
/// * `num` - The key whose associated value is to be searched.
///
/// # Returns
///
/// An `Option` containing the index associated with the provided key, if found. If the key
/// is not found, `None` is returned.
///
/// # Examples
///
/// ```
/// use hash_map::hash_map::HashMap;
/// let nums = [1, 2, 29, 34, 12];
/// let mut hash_map = HashMap::new(nums.len());
/// for (index, &num) in nums.iter().enumerate() {
///     hash_map.insert(&num, &index);
/// }
/// assert_eq!(hash_search(&hash_map, 2), Some(29));
/// assert_eq!(hash_search(&hash_map, 5), None);
/// ```
pub fn hash_search(hash_map: &HashMap<i32, usize>, num: i32) -> Option<usize> {
    hash_map.get(&num).cloned()
}
