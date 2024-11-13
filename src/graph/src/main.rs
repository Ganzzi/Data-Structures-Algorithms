use graph::{graph::Graph, word_ladder_puzzle::word_ladder_puzzle};

fn main() {
    test_graph();

    test_word_ladder_puzzle_find_shortest_distance_with_bfs();
}

fn test_graph() {
    println!("\n\n***GRAPH DATA TYPE***");

    let mut graph = Graph::new();

    graph.add_vertex(&"A");
    graph.add_vertex(&"B");
    graph.add_vertex(&"C");
    graph.add_vertex(&"D");

    graph.add_edge(&"A", &"B", 1);
    graph.add_edge(&"A", &"C", 2);
    graph.add_edge(&"B", &"C", 3);
    graph.add_edge(&"C", &"D", 4);

    println!("Graph is empty: {}", graph.is_empty());
    println!("Vertices count: {}", graph.vertices_count());
    println!("Edges count: {}", graph.edges_count());

    println!("Contains vertex 'A': {}", graph.contains(&"A"));
    println!("Contains vertex 'E': {}", graph.contains(&"E"));

    println!(
        "Is {} adjacent to {}: {}",
        "A",
        "B",
        graph.is_adjacent(&"A", &"B")
    );
    println!(
        "Is {} adjacent to {}: {}",
        "B",
        "D",
        graph.is_adjacent(&"B", &"D")
    );
}

fn test_word_ladder_puzzle_find_shortest_distance_with_bfs() {
    println!("\n\n***WORD LADDER PUZZLE***");

    let start = "hit";
    let end = "cog";
    let words = vec!["hit", "hot", "dot", "dog", "lot", "log", "cog"];

    let distance = word_ladder_puzzle(words, start, end);
    println!("Path: {:?}", distance.1);
    println!(
        "Shortest distance from {} to {} is: {}",
        start, end, distance.0
    );
}
