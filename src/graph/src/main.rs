use graph::{
    breadth_first_search::breadth_first_search, depth_first_search::depth_first_search,
    djkstra_shortest_path::djkstra_shortest_path, graph::Graph,
    knight_tour_puzzle::knight_tour_puzzle,
    strongly_connected_componentss_bfs::strongly_connected_components_bfs,
    strongly_connected_componentss_dfs::strongly_connected_components_dfs,
    topological_sort::topological_sort, word_ladder_puzzle::word_ladder_puzzle,
};

fn main() {
    test_graph();
    test_breadth_first_search();
    test_word_ladder_puzzle_find_shortest_distance_with_bfs();
    test_knight_tour_puzzle();
    test_depth_first_search();
    test_topological_sort();
    test_strongly_connected_components_bfs();
    test_strongly_connected_components_dfs();
    test_djkstra_shortest_path();
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

fn test_strongly_connected_components_bfs() {
    println!("\n\n***STRONGLY CONNECTED COMPONENTS WITH BFS APPROACH***");

    let mut graph = Graph::new();

    let courses_and_subjects = vec![
        ("Math", vec!["Algebra", "Calculus", "Geometry"], 101),
        ("Science", vec!["Physics", "Chemistry", "Biology"], 102),
        ("Literature", vec!["Poetry", "Prose", "Drama"], 103),
        ("History", vec!["Ancient", "Medieval", "Modern"], 104),
        ("Art", vec!["Painting", "Sculpture", "Music"], 105),
    ];

    courses_and_subjects
        .clone()
        .iter()
        .for_each(|(_, subjects, code)| {
            subjects.iter().for_each(|&subject| {
                subjects
                    .iter()
                    .filter(|&&s| s != subject)
                    .for_each(|&s| graph.add_edge(&subject.to_string(), &s.to_string(), *code))
            })
        });

    let courses = strongly_connected_components_bfs(&graph);

    assert_eq!(courses.len(), 5);
    courses.iter().for_each(|course| {
        let course_code = course[0].get_neighbors().first().unwrap().1;
        let course_name = courses_and_subjects
            .iter()
            .find(|(_, _, code)| *code == course_code)
            .unwrap()
            .0;
        print!("\n{:?} - {}: ", course_name, course_code);
        course.iter().for_each(|subject| {
            print!(" {:?} ", subject.get_key());
        });
        println!("");
    });
}

fn test_strongly_connected_components_dfs() {
    println!("\n\n***STRONGLY CONNECTED COMPONENTS WITH DFS APPROACH***");

    let mut graph = Graph::new();

    let states_and_cities = vec![
        ("NY", vec!["Albany", "Buffalo", "Rochester"], 36),
        ("CA", vec!["Sacramento", "Los Angeles", "San Francisco"], 39),
        ("TX", vec!["Austin", "Houston", "Dallas"], 48),
        ("FL", vec!["Tallahassee", "Miami", "Orlando"], 12),
        ("IL", vec!["Springfield", "Chicago", "Naperville"], 17),
    ];

    states_and_cities
        .clone()
        .iter()
        .for_each(|(_, cities, code)| {
            cities.iter().for_each(|&city| {
                cities
                    .iter()
                    .filter(|&&c| c != city)
                    .for_each(|&c| graph.add_edge(&city.to_string(), &c.to_string(), *code))
            })
        });

    let states = strongly_connected_components_dfs(&graph);

    assert_eq!(states.len(), 5);
    states.iter().for_each(|state| {
        let state_code = state[0].get_neighbors().first().unwrap().1;
        let state_name = states_and_cities
            .iter()
            .find(|(_, _, code)| *code == state_code)
            .unwrap()
            .0;
        print!("\n{:?} - {}: ", state_name, state_code);
        state.iter().for_each(|city| {
            print!(" {:?} ", city.get_key());
        });
        println!("");
    });
}

fn test_djkstra_shortest_path() {
    println!("\n\n***DIJKSTRA'S SHORTEST PATH***");

    let mut graph = Graph::new();

    graph.add_edge(&"A", &"B", 1);
    graph.add_edge(&"B", &"C", 1);
    graph.add_edge(&"C", &"D", 1);
    graph.add_edge(&"A", &"D", 5);

    let start_vertex = graph.get_vertex(&"A").unwrap().clone();
    let end_vertex = graph.get_vertex(&"D").unwrap().clone();

    let result = djkstra_shortest_path(&mut graph, start_vertex.clone(), Some(end_vertex.clone()));

    result
        .0
        .iter()
        .filter(|(&key, _)| key != *start_vertex.get_key())
        .for_each(|(key, distance)| {
            println!(
                "Shortest distance from {} to {key}: {distance}",
                start_vertex.get_key()
            )
        });

    println!(
        "Shortest path from {} to {}: {:?}",
        start_vertex.get_key(),
        end_vertex.get_key(),
        result.1
    );
}
