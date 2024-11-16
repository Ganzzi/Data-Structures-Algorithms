use hash_map::hash_map::HashMap;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use vertex::Vertex;

pub mod vertex;

#[derive(Debug)]
pub struct Graph<T, U>
where
    T: PartialEq + Clone + Debug + Display + Eq + Hash,
    Vertex<T, U>: PartialEq,
    U: Debug + Clone,
{
    vertices_count: usize,
    edges_count: usize,
    vertices: HashMap<T, Vertex<T, U>>,
}

impl<T, U> Graph<T, U>
where
    T: PartialEq + Clone + Debug + Display + Eq + Hash,
    Vertex<T, U>: PartialEq,
    U: Debug + Clone,
{
    /// Creates a new empty graph.
    ///
    /// # Returns
    ///
    /// A new empty graph.
    pub fn new() -> Self {
        Graph {
            vertices_count: 0,
            edges_count: 0,
            vertices: HashMap::new(),
        }
    }

    /// Checks if the graph is empty.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the graph is empty.
    pub fn is_empty(&self) -> bool {
        self.vertices_count == 0
    }

    /// Returns the number of vertices in the graph.
    ///
    /// # Returns
    ///
    /// The number of vertices in the graph.
    pub fn vertices_count(&self) -> usize {
        self.vertices_count
    }

    /// Returns the number of edges in the graph.
    ///
    /// # Returns
    ///
    /// The number of edges in the graph.
    pub fn edges_count(&self) -> usize {
        self.edges_count
    }

    /// Checks if the graph contains a vertex with the specified key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the vertex to check.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the graph contains the vertex.
    pub fn contains(&self, key: &T) -> bool {
        self.vertices.contains(key)
    }

    /// Adds a vertex with the specified key to the graph.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the vertex to be added.
    pub fn add_vertex(&mut self, key: &T) {
        if !self.contains(key) {
            self.vertices.insert(key.clone(), Vertex::new(key.clone()));
            self.vertices_count += 1;
        }
    }

    /// Returns a reference to the vertex with the specified key, if it exists.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the vertex to retrieve.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the vertex, or None if the vertex does not exist.
    pub fn get_vertex(&self, key: &T) -> Option<Vertex<T, U>> {
        self.vertices.get(key).cloned()
    }

    /// Returns a mutable reference to the vertex with the specified key, if it exists.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the vertex to retrieve.
    ///
    /// # Returns
    ///
    /// An option containing a mutable reference to the vertex, or None if the vertex does not exist.
    pub fn get_vertex_mut(&mut self, key: &T) -> Option<&mut Vertex<T, U>> {
        self.vertices.get_mut(key)
    }

    /// Returns a vector of keys of all vertices in the graph.
    ///
    /// # Returns
    ///
    /// A vector of keys of all vertices in the graph.
    pub fn get_vertex_keys(&self) -> Vec<T> {
        self.vertices.iter().map(|(k, _)| k.clone()).collect()
    }

    /// Removes the vertex with the specified key from the graph.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the vertex to be removed.
    ///
    /// # Returns
    ///
    /// An option containing the removed vertex, or None if the vertex does not exist.
    pub fn remove_vertex(&mut self, key: &T) -> Option<Vertex<T, U>> {
        if let Some(vertex) = self.vertices.remove(key) {
            self.vertices_count -= 1;

            self.edges_count -= vertex.get_neighbors().len();

            for vtx_key in self.get_vertex_keys() {
                if let Some(vtx) = self.vertices.get_mut(&vtx_key) {
                    if vtx.is_adjacent(key) {
                        if let Some(_) = vtx.remove_neighbor(key) {
                            self.edges_count -= 1;
                        }
                    }
                }
            }

            Some(vertex)
        } else {
            None
        }
    }

    /// Adds an edge between two vertices with the specified weight.
    ///
    /// # Arguments
    ///
    /// * `from` - The key of the starting vertex.
    /// * `to` - The key of the ending vertex.
    /// * `weight` - The weight of the edge.
    pub fn add_edge(&mut self, from: &T, to: &T, u: U) {
        if !self.vertices.contains(from) {
            self.add_vertex(from);
        }

        if !self.vertices.contains(to) {
            self.add_vertex(to);
        }

        self.edges_count += 1;

        self.vertices
            .get_mut(from)
            .unwrap()
            .add_neighbor(to.clone(), u);
    }

    /// Checks if there is an edge between two vertices.
    ///
    /// # Arguments
    ///
    /// * `from` - The key of the starting vertex.
    /// * `to` - The key of the ending vertex.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether there is an edge between the two vertices.
    pub fn is_adjacent(&self, from: &T, to: &T) -> bool {
        if let Some(vertex) = self.vertices.get(from) {
            vertex.is_adjacent(to)
        } else {
            false
        }
    }
}
