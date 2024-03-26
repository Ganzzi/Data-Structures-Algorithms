use hash_map::hash_map::HashMap;

fn main() {
    // HASHMAP DATA TYPE
    println!("\n\n***HASHMAP DATA TYPE***");
        
    // Create a new HashMap with capacity 6
    let mut hash_map = HashMap::<String, String>::new(6);

    // Insert key-value pairs
    hash_map.insert(&"key1".to_string(), &"value1".to_string());
    hash_map.insert(&"key2".to_string(), &"value2".to_string());
    hash_map.insert(&"key3".to_string(), &"value3".to_string());
    hash_map.insert(&"key4".to_string(), &"value4".to_string());

    // Retrieve values by key
    println!("HashMap after initial insertions: \n{:?}\n", hash_map);

    // Update a value
    hash_map.insert(&"key2".to_string(), &"updated_value2".to_string());
    println!("HashMap after updating 'key2': \n{:?}\n", hash_map);

    // Remove a key-value pair
    hash_map.remove(&"key3".to_string());
    println!("HashMap after removing 'key3': \n{:?}\n", hash_map);

    // Get value by key
    match hash_map.get(&"key2".to_string()) {
        Some(value) => {
            println!("Index of 'key2': {}", hash_map.index_of(&"key2".to_string()));
            println!("Value of 'key2': {}", value);
        } 
        None => println!("'key2' not found in HashMap"),
    }

    // Get mutable value by key
    if let Some(value) = hash_map.get_mut(&"key1".to_string()) {
        *value = "updated_value1".to_string();
        println!("\nHashMap after updating 'key1' through mutable reference: \n{:?}", hash_map);
    } else {
        println!("'key1' not found in HashMap");
    }

    // Check length and emptiness of the HashMap
    println!("\nLength of HashMap: {}", hash_map.len());
    println!("\nIs HashMap empty? {}", hash_map.is_empty());

    // Iterate over key-value pairs
    println!("\nIterating over key-value pairs:");
    for (key, value) in hash_map.iter() {
        println!("Key: {}, Value: {}", key, value);
    }

    // Iterate over mutable key-value pairs
    println!("\nIterating over mutable key-value pairs:");
    for (key, value) in hash_map.iter_mut() {
        *value += &"_mutated".to_string();
        println!("Key: {}, Value: {}", key, value);
    }
}
