use tree::{
    avl_tree::AVLTree, binary_heap::BinaryHeap, binary_search_tree::BinarySearchTree,
    binary_tree::BinaryTree,
};
use vec::linked_vec;

fn main() {
    test_binary_tree();

    test_binary_heap();

    test_binary_search_tree();

    test_avl_tree();
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
    println!("Max key: {:?}", tree.max_key());
    println!("Contains 7: {}", tree.contains(&7));
    println!("Contains 20: {}", tree.contains(&20));

    println!("Pre-order traversal:");
    tree.pre_order(&mut |key| println!("Key: {}", key));

    println!("\nIn-order traversal:");
    tree.in_order(&mut |key| println!("Key: {}", key));

    println!("\nPost-order traversal:");
    tree.post_order(&mut |key| println!("Key: {}", key));

    println!("\nLevel-order traversal:");
    tree.level_order(&mut |key| println!("Key: {}", key));
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

    let data = linked_vec![3, 1, 6, 5, 2, 4];
    heap.build_new(&data);
    println!("Heap after build_new: {:?}", heap);

    let additional_data = linked_vec![7, 8, 9];
    heap.build_add(additional_data);
    println!("Heap after build_add: {:?}", heap);
}

fn test_binary_search_tree() {
    println!("\n\n***BINARY SEARCH TREE DATA TYPE***");

    let mut bst = BinarySearchTree::new();

    bst.insert(10, "ten");
    bst.insert(20, "twenty");
    bst.insert(5, "five");
    bst.insert(6, "six");
    bst.insert(15, "fifteen");

    println!("Tree is empty: {}", bst.is_empty());
    println!("Tree size: {}", bst.size());
    println!("Leaf size: {}", bst.leaf_size());
    println!("Non-leaf size: {}", bst.none_leaf_size());
    println!("Tree depth: {}", bst.depth());

    println!("Contains 10: {}", bst.contains(&10));
    println!("Contains 15: {}", bst.contains(&15));
    println!("Contains 100: {}", bst.contains(&100));

    if let Some(value) = bst.get(&10) {
        println!("Value for key 10: {}", value);
    }

    let (min_key, min_value) = bst.min();
    println!("Min key: {:?}, Min value: {:?}", min_key, min_value);

    let (max_key, max_value) = bst.max();
    println!("Max key: {:?}, Max value: {:?}", max_key, max_value);

    println!("Pre-order traversal:");
    bst.pre_order(&mut |key, value| println!("Key: {}, Value: {}", key, value));

    println!("In-order traversal:");
    bst.in_order(&mut |key, value| println!("Key: {}, Value: {}", key, value));

    println!("Post-order traversal:");
    bst.post_order(&mut |key, value| println!("Key: {}, Value: {}", key, value));

    println!("Level-order traversal:");
    bst.level_order(&mut |key, value| println!("Key: {}, Value: {}", key, value));
}

fn test_avl_tree() {
    println!("\n\n***AVL TREE DATA TYPE***");

    let mut avl_tree = AVLTree::new();

    avl_tree.insert(10);
    avl_tree.insert(20);
    avl_tree.insert(5);
    avl_tree.insert(6);
    avl_tree.insert(15);

    println!("Tree is empty: {}", avl_tree.is_empty());
    println!("Tree size: {}", avl_tree.size());
    println!("Leaf size: {}", avl_tree.leaf_size());
    println!("Non-leaf size: {}", avl_tree.non_leaf_size());
    println!("Tree depth: {}", avl_tree.depth());
    println!("Tree is balanced: {}", avl_tree.is_balanced());

    println!("Contains 10: {}", avl_tree.contains(&10));
    println!("Contains 15: {}", avl_tree.contains(&15));
    println!("Contains 100: {}", avl_tree.contains(&100));

    if let Some(min) = avl_tree.min() {
        println!("Min key: {}", min);
    }

    if let Some(max) = avl_tree.max() {
        println!("Max key: {}", max);
    }

    println!("Pre-order traversal:");
    avl_tree.pre_order(&mut |key| println!("Key: {}", key));

    println!("In-order traversal:");
    avl_tree.in_order(&mut |key| println!("Key: {}", key));

    println!("Post-order traversal:");
    avl_tree.post_order(&mut |key| println!("Key: {}", key));

    println!("Level-order traversal:");
    avl_tree.level_order(&mut |key| println!("Key: {}", key));
}
