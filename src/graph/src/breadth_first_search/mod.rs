use hash_map::hash_map::HashMap;
use queue::queue::Queue;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::Add;
use vec::linked_vec;
use vec::vec::LinkedVec;

use crate::graph::{vertex::Vertex, Graph};

/// # Breadth-first search
///
/// Breadth-first search algorithm to find the shortest path between two vertices in a graph. The search is performed by traversing the graph level by level, starting from the initial vertex. The algorithm uses a queue to keep track of the vertices to visit next.
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
/// use crate::graph::{graph::Graph, breadth_first_search::breadth_first_search};
///
/// let mut graph = Graph::new();
///
/// graph.add_edge(&"A", &"B", 1);
/// graph.add_edge(&"B", &"C", 1);
///
/// let start_vertex = graph.get_vertex(&"A").unwrap().clone();
/// let end_vertex = graph.get_vertex(&"C").unwrap().clone();
///
/// let result = breadth_first_search(&mut graph, start_vertex, end_vertex);
///
/// println!("Path: {:?}", result.1);
/// println!("Total distance: {}", result.0);
///
/// assert_eq!(result.0, 2);
/// ```
pub fn breadth_first_search<
    T: Clone + Debug + Display + Eq + Hash,
    U: Debug + Default + Clone + PartialEq + Add<Output = U>,
>(
    graph: &mut Graph<T, U>,
    start: Vertex<T, U>,
    end: Vertex<T, U>,
) -> (U, LinkedVec<T>) {
    if !graph.contains(start.get_key()) {
        return (U::default(), linked_vec![]);
    }
    if !graph.contains(end.get_key()) {
        return (U::default(), linked_vec![]);
    }

    let mut predecessors: HashMap<T, Option<T>> = HashMap::with_capacity(graph.vertices_count());
    let mut distances: HashMap<T, U> = HashMap::with_capacity(graph.vertices_count());
    distances.insert(start.get_key().clone(), U::default());
    predecessors.insert(start.get_key().clone(), None);

    let mut vertex_queue: Queue<Vertex<T, U>> = Queue::new(graph.vertices_count());
    let _ = vertex_queue.enqueue(start.clone());

    while !vertex_queue.is_empty() {
        let vertex = vertex_queue.dequeue().unwrap();

        for neighbor in vertex.get_neighbors() {
            let vertex_neighbor = graph.get_vertex_mut(&neighbor.0).unwrap();

            if vertex_neighbor.get_key() == start.get_key() {
                continue;
            } else if vertex_neighbor.get_key() == end.get_key() {
                let mut path = linked_vec![vertex_neighbor.get_key().clone()];
                let mut current_vertex = Some(vertex.get_key().clone());
                while let Some(key) = current_vertex {
                    path.push(key.clone());
                    current_vertex = predecessors.get(&key.clone()).unwrap().clone();
                }
                path.reverse();

                return (
                    distances
                        .get(vertex.get_key())
                        .unwrap()
                        .clone()
                        .add(neighbor.1.clone()),
                    path,
                );
            } else if distances.get(vertex_neighbor.get_key()).is_none() {
                let neighbor_distance = distances
                    .get(vertex.get_key())
                    .unwrap()
                    .clone()
                    .add(neighbor.1.clone());
                distances.insert(vertex_neighbor.get_key().clone(), neighbor_distance.clone());
                predecessors.insert(
                    vertex_neighbor.get_key().clone(),
                    Some(vertex.get_key().clone()),
                );

                let _ = vertex_queue.enqueue(vertex_neighbor.clone());
            }
        }
    }

    (U::default(), linked_vec![])
}
