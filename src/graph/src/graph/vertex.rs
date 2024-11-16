use std::fmt::Debug;

/// Represents a vertex in a graph.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Vertex<T, U> {
    key: T,
    pub neighbors: Vec<(T, U)>,
}

impl<T, U> Vertex<T, U>
where
    T: Eq,
    U: Clone + Debug,
{
    /// Creates a new vertex with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the vertex.
    ///
    /// # Returns
    ///
    /// A new vertex with the specified key.
    pub fn new(key: T) -> Self {
        Vertex {
            key,
            neighbors: Vec::new(),
        }
    }

    /// Adds a neighbor to the vertex with the given key and weight.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the neighbor vertex.
    /// * `weight` - The weight of the edge to the neighbor vertex.
    pub fn add_neighbor(&mut self, key: T, weight: U) {
        if self.neighbors.iter().any(|(k, _)| k == &key) {
            return;
        }
        self.neighbors.push((key, weight));
    }

    /// Returns a reference to the neighbors of the vertex.
    ///
    /// # Returns
    ///
    /// A reference to a vector of tuples containing the keys and weights of the neighbors.
    pub fn get_neighbors(&self) -> &Vec<(T, U)> {
        &self.neighbors
    }

    /// Returns a vector of references to the keys of the neighbors.
    ///
    /// # Returns
    ///
    /// A vector of references to the keys of the neighbors.
    pub fn get_neighbor_keys(&self) -> Vec<&T> {
        self.neighbors.iter().map(|(key, _)| key).collect()
    }

    /// Returns a reference to the key of the vertex.
    ///
    /// # Returns
    ///
    /// A reference to the key of the vertex.
    pub fn get_key(&self) -> &T {
        &self.key
    }

    /// Returns a reference to the weight of the edge to the specified neighbor, if it exists.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the neighbor vertex.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the weight of the edge to the neighbor, or None if the neighbor does not exist.
    pub fn get_neighbor_weight(&self, key: &T) -> Option<&U> {
        self.neighbors
            .iter()
            .find(|(k, _)| k.eq(key))
            .map(|(_, w)| w)
    }

    /// Checks if the vertex is adjacent to the specified vertex.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the vertex to check adjacency with.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the vertex is adjacent to the specified vertex.
    pub fn is_adjacent(&self, key: &T) -> bool {
        self.neighbors.iter().any(|(k, _)| k.eq(key))
    }

    /// Removes the neighbor with the specified key from the vertex.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the neighbor to be removed.
    ///
    /// # Returns
    ///
    /// An option containing a tuple of the removed neighbor's key and weight, or None if the neighbor does not exist.
    pub fn remove_neighbor(&mut self, key: &T) -> Option<(T, U)> {
        if let Some(index) = self.neighbors.iter().position(|(k, _)| k.eq(key)) {
            Some(self.neighbors.remove(index))
        } else {
            None
        }
    }
}
