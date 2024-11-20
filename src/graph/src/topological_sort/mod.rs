use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::graph::Graph;
use hash_map::hash_map::HashMap;
use vec::vec::LinkedVec;

/// # Topological Sort
///
/// Topological sorting is a variant of depth-first search. It produces a linear ordering of the vertices in a directed acyclic graph (DAG). For example, the vertices of the graph may represent tasks to be performed, and the edges may represent constraints that one task must be performed before another; in this application, a topological ordering is just a valid sequence for the tasks.
///
/// # Arguments
///
/// * `edges` - A vector of tuples representing the edges of the graph. Each tuple contains two elements: the first element is the source vertex and the second element is the destination vertex.
///
/// # Returns
///
/// An optional linked list of vertices in topological order. If the graph has a cycle, the function returns `None`.
///
/// # Example
///
/// ```
/// use graph::topological_sort::topological_sort;
/// use vec::vec::LinkedVec;
///
/// let edges = vec![
///     ("A", "B"),
///     ("A", "C"),
///     ("B", "C"),
/// ];
///
/// let schedule = topological_sort(edges).unwrap();
///
/// assert_eq!(schedule.to_vec(), vec!["C", "B", "A"]);
/// ```
///
pub fn topological_sort<T: Clone + Debug + Display + Eq + Hash>(
    edges: Vec<(T, T)>,
) -> Option<LinkedVec<T>> {
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum VisitStatus {
        Unvisited,
        Visited,
        Visiting,
    }

    let graph = build_graph(edges);

    let mut schedule: LinkedVec<T> = LinkedVec::new();
    let mut visited: HashMap<T, VisitStatus> = HashMap::with_capacity(graph.vertices_count());
    let mut has_cycle = false;

    for key in graph.get_vertex_keys() {
        visited.insert(key.clone(), VisitStatus::Unvisited);
    }

    for key in graph.get_vertex_keys() {
        if *visited.get(&key).unwrap() == VisitStatus::Unvisited {
            visited.insert(key.clone(), VisitStatus::Visiting);
            has_cycle = build_schedule(&graph, key.clone(), &mut schedule, &mut visited, has_cycle);
            if has_cycle {
                break;
            }
        }
    }

    if has_cycle {
        return None;
    } else {
        return Some(schedule);
    }

    fn build_graph<T: Clone + Debug + Display + Eq + Hash>(edges: Vec<(T, T)>) -> Graph<T, u8> {
        let mut graph = Graph::new();
        for (t1, t2) in edges {
            graph.add_edge(&t1, &t2, 1);
        }

        graph
    }

    fn build_schedule<T: Clone + Debug + Display + Eq + Hash>(
        graph: &Graph<T, u8>,
        vertex_key: T,
        schedule: &mut LinkedVec<T>,
        visited: &mut HashMap<T, VisitStatus>,
        has_cycle: bool,
    ) -> bool {
        let mut cycle_found = has_cycle;

        let neighbors = graph
            .get_vertex(&vertex_key)
            .unwrap()
            .get_neighbors()
            .clone();
        for (neighbor_key, _) in neighbors {
            if *visited.get(&neighbor_key).unwrap() == VisitStatus::Visiting {
                return true;
            } else if *visited.get(&neighbor_key).unwrap() == VisitStatus::Unvisited {
                visited.insert(neighbor_key.clone(), VisitStatus::Visiting);
                cycle_found = build_schedule(graph, neighbor_key, schedule, visited, has_cycle);
                if cycle_found {
                    return cycle_found;
                }
            }
        }

        visited.insert(vertex_key.clone(), VisitStatus::Visited);
        schedule.push(vertex_key.clone());

        cycle_found
    }
}
