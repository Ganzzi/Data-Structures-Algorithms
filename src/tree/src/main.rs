use tree::{binary_heap::BinaryHeap, binary_tree::BinaryTree};

fn main() {
    test_binary_tree();

    test_binary_heap();
}

fn test_binary_tree() {
    println!("***BINARY TREE DATA TYPE***");
    let mut tree = BinaryTree::new(10);
    tree.push(&5);
    tree.push(&15);
    tree.push(&3);
    tree.push(&7);
    tree.push(&12);
    tree.push(&18);

    println!("Tree size: {}", tree.size());
    println!("Leaf size: {}", tree.leaf_size());
    println!("Non-leaf size: {}", tree.none_leaf_size());
    println!("Tree depth: {}", tree.depth());
    println!("Min key: {:?}", tree.min_key());
    println!("Max key: {:?}", tree.maxkey());
    println!("Contains 7: {}", tree.contains(&7));
    println!("Contains 20: {}", tree.contains(&20));

    println!("Pre-order traversal:");
    tree.pre_order();

    println!("\nIn-order traversal:");
    tree.in_order();

    println!("\nPost-order traversal:");
    tree.post_order();

    println!("\nLevel-order traversal:");
    tree.level_order();
}

fn test_binary_heap() {
    println!("\n\n***BINARY TREE DATA TYPE***");

    let mut heap = BinaryHeap::new();

    heap.push(10);
    heap.push(4);
    heap.push(15);
    heap.push(20);
    heap.push(0);

    println!("Heap size: {}", heap.size());
    println!("Min element: {:?}", heap.min());

    while !heap.is_empty() {
        println!("Popped element: {:?}", heap.pop());
    }

    let data = vec![3, 1, 6, 5, 2, 4];
    heap.build_new(&data);
    println!("Heap after build_new: {:?}", heap);

    let additional_data = vec![7, 8, 9];
    heap.build_add(additional_data);
    println!("Heap after build_add: {:?}", heap);
}
