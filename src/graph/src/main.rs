use graph::{
    breadth_first_search::breadth_first_search, depth_first_search::depth_first_search,
    graph::Graph, knight_tour_puzzle::knight_tour_puzzle, topological_sort::topological_sort,
    word_ladder_puzzle::word_ladder_puzzle,
};

fn main() {
    test_graph();
    test_breadth_first_search();
    test_word_ladder_puzzle_find_shortest_distance_with_bfs();
    test_knight_tour_puzzle();
    test_depth_first_search();
    test_topological_sort();
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

fn test_breadth_first_search() {
    println!("\n\n***BREADTH-FIRST SEARCH***");

    let mut graph = Graph::new();

    graph.add_vertex(&"A");
    graph.add_vertex(&"B");
    graph.add_vertex(&"C");
    graph.add_vertex(&"D");

    graph.add_edge(&"A", &"B", 1);
    graph.add_edge(&"A", &"C", 2);
    graph.add_edge(&"B", &"C", 3);
    graph.add_edge(&"C", &"D", 4);

    let start_vertex = graph.get_vertex(&"A").unwrap().clone();
    let end_vertex = graph.get_vertex(&"D").unwrap().clone();

    let result = breadth_first_search(&mut graph, start_vertex, end_vertex);

    println!("Path: {:?}", result.1);
    println!("Total distance: {}", result.0);
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

fn test_knight_tour_puzzle() {
    println!("\n\n***KNIGHT'S TOUR***");

    let board_size = 5;
    let start_position = 10;

    if let Some(path) = knight_tour_puzzle(board_size, start_position) {
        println!("Knight's tour found: {:?}", path);
        println!("The path: {}", path.len());
    } else {
        println!(
            "No knight's tour found for the {}x{} board starting at position {}",
            board_size, board_size, start_position
        );
    }
}

fn test_depth_first_search() {
    println!("\n\n***DEPTH-FIRST SEARCH***");

    let mut graph = Graph::new();

    graph.add_vertex(&"A");
    graph.add_vertex(&"B");
    graph.add_vertex(&"C");
    graph.add_vertex(&"D");

    graph.add_edge(&"A", &"B", 1);
    graph.add_edge(&"A", &"C", 2);
    graph.add_edge(&"B", &"C", 3);
    graph.add_edge(&"C", &"D", 4);

    let start_vertex = graph.get_vertex(&"A").unwrap().clone();
    let end_vertex = graph.get_vertex(&"D").unwrap().clone();

    let result = depth_first_search(&mut graph, start_vertex, end_vertex);

    println!("Path: {:?}", result.1);
    println!("Total distance: {}", result.0);
}

fn test_topological_sort() {
    println!("\n\n***TOPOLOGICAL SORT***");

    let edges = vec![
        ("Mix", "3/4 cup of milk"),
        ("Mix", "1 egg"),
        ("Mix", "1 tablespoon olive oil"),
        ("Pour in 1/4 cup", "Mix"),
        ("Pour in 1/4 cup", "Heat the pan"),
        ("Flip the bottom until golden", "Pour in 1/4 cup"),
        ("Enjoy", "Flip the bottom until golden"),
        ("Enjoy", "Heat the sauce"),
    ];
    let order = topological_sort(edges.clone());
    if order.is_none() {
        println!("The graph has a cycle");
    } else {
        println!("The graph has no cycle. Order:");
        order.unwrap().print();
    }
}
