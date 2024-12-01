use hash_map::hash_map::HashMap;
use queue::queue::Queue;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::Add;
use vec::vec::LinkedVec;

use crate::graph::{vertex::Vertex, Graph};

/// # BFS Strongly Connected Components Algorithm
///
/// The bfs strongly connected components algorithm finds all the connected vertices in a directed graph.
/// It uses breadth-first search approach to traverse the graph.
///
/// # Agruments
///
/// * `graph` - A directed graph.
///
/// # Returns
///
/// A Linked Vector of strongly connected components.
///
/// # Example
///
/// ```
/// use crate::graph::{graph::Graph, strongly_connected_componentss_bfs::strongly_connected_components_bfs};
/// use vec::linked_vec;
///
/// let mut graph = Graph::new();
///
/// let states_and_cities = linked_vec![
///     ("NY", linked_vec!["Albany", "Buffalo", "Rochester"], 36),
///     ("CA", linked_vec!["Sacramento", "Los Angeles", "San Francisco"], 39),
///     ("TX", linked_vec!["Austin", "Houston", "Dallas"], 48),
///     ("FL", linked_vec!["Tallahassee", "Miami", "Orlando"], 12),
///     ("IL", linked_vec!["Springfield", "Chicago", "Naperville"], 17),
/// ];
///
/// states_and_cities
///     .clone()
///     .iter()
///     .for_each(|(_, cities, code)| {
///         cities.iter().for_each(|&city| {
///             cities
///                 .iter()
///                 .filter(|&&c| c != city)
///                 .for_each(|&c| graph.add_edge(&city.to_string(), &c.to_string(), *code))
///         })
///     });
///
/// let states = strongly_connected_components_bfs(&graph);
///
/// assert_eq!(states.len(), 5);
/// states.iter().for_each(|state| {
///     let state_code = state[0].get_neighbors().first().unwrap().1;
///     let state_name = states_and_cities
///         .iter()
///         .find(|(_, _, code)| *code == state_code)
///         .unwrap()
///         .0;
///     println!("{:?} - {}:", state_name, state_code);
///     state.iter().for_each(|city| {
///         println!("- {:?}", city.get_key());
///     });
///     println!();
/// });
/// ```
pub fn strongly_connected_components_bfs<
    T: Clone + Debug + Display + Eq + Hash,
    U: Debug + Default + Clone + PartialEq + Add<Output = U>,
>(
    graph: &Graph<T, U>,
) -> LinkedVec<LinkedVec<Vertex<T, U>>> {
    let mut visited: HashMap<T, bool> = HashMap::new();
    let mut queue: Queue<Vertex<T, U>> = Queue::new(graph.vertices_count());
    let mut result: LinkedVec<LinkedVec<Vertex<T, U>>> = LinkedVec::new();

    for key in graph.get_vertex_keys().iter() {
        visited.insert(key.clone(), false);
    }

    for key in graph.get_vertex_keys().iter() {
        if *visited.get(&key).unwrap() {
            continue;
        }

        let mut component: LinkedVec<Vertex<T, U>> = LinkedVec::new();
        let vertex = graph.get_vertex(&key).unwrap().clone();
        let _ = queue.enqueue(vertex.clone());
        visited.insert(vertex.get_key().clone(), true);

        while !queue.is_empty() {
            let vertex = queue.dequeue().unwrap();
            component.push(vertex.clone());

            for (neighbour_key, _) in vertex.get_neighbors() {
                if !*visited.get(&neighbour_key).unwrap() {
                    let neighbour = graph.get_vertex(&neighbour_key).unwrap().clone();
                    let _ = queue.enqueue(neighbour.clone());
                    visited.insert(neighbour.get_key().clone(), true);
                }
            }
        }

        result.push(component);
    }

    result
}
