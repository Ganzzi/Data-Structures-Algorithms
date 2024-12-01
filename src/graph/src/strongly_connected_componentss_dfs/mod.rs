use hash_map::hash_map::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::Add;
use vec::vec::LinkedVec;

use crate::graph::{vertex::Vertex, Graph};

/// # DFS Strongly Connected Components Algorithm
///
/// The dfs strongly connected components algorithm finds all the connected vertexs in a directed graph.
/// It uses depth-first seach approach to traverse the graph
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
/// use crate::graph::{graph::Graph, strongly_connected_componentss_dfs::strongly_connected_components_dfs};
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
/// let states = strongly_connected_components_dfs(&graph);
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
pub fn strongly_connected_components_dfs<
    T: Clone + Debug + Display + Eq + Hash,
    U: Debug + Default + Clone + PartialEq + Add<Output = U>,
>(
    graph: &Graph<T, U>,
) -> LinkedVec<LinkedVec<Vertex<T, U>>> {
    let mut visited: HashMap<T, bool> = HashMap::new();
    let mut result: LinkedVec<LinkedVec<Vertex<T, U>>> = LinkedVec::new();

    for key in graph.get_vertex_keys().iter() {
        visited.insert(key.clone(), false);
    }

    for key in graph.get_vertex_keys().iter() {
        if *visited.get(&key).unwrap() {
            continue;
        }

        let vertex = graph.get_vertex(&key).unwrap().clone();
        let component = get_component(graph, &mut visited, vertex);
        result.push(component);
    }

    return result;

    fn get_component<
        T: Clone + Debug + Display + Eq + Hash,
        U: Debug + Default + Clone + PartialEq + Add<Output = U>,
    >(
        graph: &Graph<T, U>,
        visited: &mut HashMap<T, bool>,
        vertex: Vertex<T, U>,
    ) -> LinkedVec<Vertex<T, U>> {
        visited.insert(vertex.get_key().clone(), true);
        let mut component: LinkedVec<Vertex<T, U>> = LinkedVec::new();
        component.push(vertex.clone());

        for (neighbor, _) in vertex.get_neighbors() {
            if !visited.get(&neighbor.clone()).unwrap() {
                let neighbor = graph.get_vertex(&neighbor).unwrap().clone();
                let sub_component = get_component(graph, visited, neighbor);
                component.extend(sub_component);
            }
        }

        component
    }
}
