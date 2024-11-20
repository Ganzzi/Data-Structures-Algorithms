use hash_map::hash_map::HashMap;
use queue::queue::Queue;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::Add;

use crate::graph::{vertex::Vertex, Graph};

/// # Dijkstra's Shortest Path Algorithm
///
/// Dijkstra's shortest path algorithm to find the shortest distanced path between two vertices in a graph . The algorithm uses a priority queue to keep track of the vertices to visit next.
///
/// # Arguments
///
/// * `graph` - A mutable reference to a graph.
/// * `start` - The starting vertex.
/// * `end` - The ending vertex.
///
/// # Returns
///
/// A tuple containing the total distance between the two vertices and the path between them.
///
/// # Example
///
/// ```
/// use crate::graph::{graph::Graph, djkstra_shortest_path::djkstra_shortest_path};
///
/// let mut graph = Graph::new();
///
/// graph.add_edge(&"A", &"B", 1);
/// graph.add_edge(&"B", &"C", 1);
/// graph.add_edge(&"C", &"D", 1);
/// graph.add_edge(&"A", &"D", 5);
///
/// let start_vertex = graph.get_vertex(&"A").unwrap().clone();
/// let end_vertex = graph.get_vertex(&"D").unwrap().clone();
///
/// let result = djkstra_shortest_path(&mut graph, start_vertex.clone(), Some(end_vertex.clone()));
///
/// assert_eq!(result.0.get(&"D").unwrap(), &3);
/// assert_eq!(result.1.unwrap(), vec!["A", "B", "C", "D"]);
/// ```
pub fn djkstra_shortest_path<
    T: Clone + Debug + Display + Eq + Hash,
    U: Debug + Default + Clone + PartialEq + PartialOrd + Add<Output = U>,
>(
    graph: &mut Graph<T, U>,
    start: Vertex<T, U>,
    end: Option<Vertex<T, U>>,
) -> (HashMap<T, U>, Option<Vec<T>>) {
    if !graph.contains(&start.get_key()) {
        return (HashMap::new(), None);
    }
    if end.is_some() && !graph.contains(&end.as_ref().unwrap().get_key()) {
        return (HashMap::new(), None);
    }

    let mut predecessors: HashMap<T, Option<T>> = HashMap::with_capacity(graph.vertices_count());
    predecessors.insert(start.get_key().clone(), None);
    let mut distances: HashMap<T, U> = HashMap::with_capacity(graph.vertices_count());
    distances.insert(start.get_key().clone(), U::default());

    let mut vertex_queue: Queue<Vertex<T, U>> = Queue::new(graph.vertices_count());
    let _ = vertex_queue.enqueue(start.clone());

    while !vertex_queue.is_empty() {
        let vertex = vertex_queue.dequeue().unwrap();

        if distances.get(vertex.get_key()).is_none() {
            continue;
        }

        for neighbor in vertex.get_neighbors() {
            let vertex_neighbor = graph.get_vertex_mut(&neighbor.0).unwrap();

            let new_distance = distances
                .get(vertex.get_key())
                .unwrap()
                .clone()
                .add(neighbor.1.clone());

            let is_shorter = distances
                .get(vertex_neighbor.get_key())
                .map_or(true, |curr| new_distance < *curr);
            if is_shorter {
                distances.insert(vertex_neighbor.get_key().clone(), new_distance);
                predecessors.insert(
                    vertex_neighbor.get_key().clone(),
                    Some(vertex.get_key().clone()),
                );
                let _ = vertex_queue.enqueue(vertex_neighbor.clone());
            }
        }
    }

    match end {
        Some(end) => {
            let mut path = vec![];
            let mut curr = Some(end.get_key().clone());
            while let Some(key) = curr {
                path.push(key.clone());
                curr = predecessors.get(&key.clone()).unwrap().clone();
            }
            path.reverse();

            return (distances, Some(path));
        }
        None => (distances, None),
    }
}
