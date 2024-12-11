use practices::{
    base58::{decode, encode},
    blockchain::Blockchain,
    consistent_hashing::{Node, Ring},
    filters::BloomFilter,
    least_recently_used::LRUCache,
    levenshtein_distance::levenshtein_distance,
    trie::Trie,
};

fn main() {
    consistent_hashing();
    least_recently_used();
    trie();
    edit_distance();
    filters();
    base58();
    blockchain();
}

fn consistent_hashing() {
    println!("\n\n***CONSISTENT HASHING***");

    let mut ring = Ring::new();

    let nodes = vec![
        Node {
            data: String::from("Node1"),
        },
        Node {
            data: String::from("Node2"),
        },
        Node {
            data: String::from("Node3"),
        },
    ];

    ring.add_multiple(&nodes);

    let key = Node {
        data: String::from("Node2"),
    };
    if let Some(node) = ring.get(key) {
        println!("Found node: {:?}", node);
    } else {
        println!("Node not found");
    }

    println!("Node Total: {}", ring.node_count());
    ring.remove_multiple(&nodes);
    println!("nodes removed");

    println!("Node Total: {}", ring.node_count());
}

fn least_recently_used() {
    println!("\n\n***LEAST RECENTLY USED***");

    let mut cache = LRUCache::new(2);
    cache.insert(1, "one");
    println!("inserted key {}", 1);
    cache.insert(2, "two");
    println!("inserted key {}", 2);
    println!("key {}: {:?}", 1, cache.get(&1));
    println!("key {}: {:?}", 2, cache.get(&2));
    cache.insert(3, "three");
    println!("inserted key {}", 3);
    println!("key {}: {:?}", 1, cache.get(&1));
    println!("key {}: {:?}", 3, cache.get(&3));
    cache.remove(&2);
    println!("removed key {}", 2);
    println!("key {}: {:?}", 2, cache.get(&2));
}

fn trie() {
    println!("\n\n***TRIE***");

    let mut trie = Trie::new();

    trie.insert("hello");
    println!("Inserted 'hello'");
    trie.insert("world");
    println!("Inserted 'world'");

    println!("Contains 'hello': {}", trie.contains("hello"));
    println!("Contains 'world': {}", trie.contains("world"));
    println!("Contains 'hell': {}", trie.contains("hell"));
    println!("Starts with 'hell': {}", trie.start_with("hell"));
    println!("Starts with 'worl': {}", trie.start_with("worl"));
    println!("Starts with 'wor': {}", trie.start_with("wor"));
}

fn edit_distance() {
    println!("\n\n***EDIT DISTANCE***");

    let source = "kitten";
    let target = "sitting";
    let distance = levenshtein_distance(source, target);
    println!(
        "The Levenshtein distance between '{}' and '{}' is {}",
        source, target, distance
    );
}

fn filters() {
    println!("\n\n***FILTERS***");

    let mut bloom_filter = BloomFilter::new();

    let item1 = "hello";
    let item2 = "world";
    let item3 = "rust";

    bloom_filter.insert(&item1);
    println!("Inserted '{}'", item1);
    bloom_filter.insert(&item2);
    println!("Inserted '{}'", item2);

    println!("Contains '{}': {}", item1, bloom_filter.contains(&item1));
    println!("Contains '{}': {}", item2, bloom_filter.contains(&item2));
    println!("Contains '{}': {}", item3, bloom_filter.contains(&item3));
}

fn base58() {
    println!("\n\n***BASE 58***");

    let encoded = encode("hello");
    println!("Encoded: {}", encoded);
    let decoded = decode(&encoded);
    println!("Decoded: {}", decoded);
}

fn blockchain() {
    println!("\n\n***BASIC BLOCKCHAIN***");

    println!("-----------------mining information-----------------");
    let mut blockchain = Blockchain::new();
    blockchain.add_block(format!(
        "{} -> {}: {}btc",
        encode("Alice"),
        encode("Bob"),
        1
    ));
    blockchain.add_block(format!(
        "{} -> {}: {}btc",
        encode("Bob"),
        encode("Charlie"),
        2
    ));
    blockchain.add_block(format!(
        "{} -> {}: {}btc",
        encode("Charlie"),
        encode("Alice"),
        3
    ));

    println!("-----------------blockchain information-----------------");
    blockchain.print_blockchain_info();
}
