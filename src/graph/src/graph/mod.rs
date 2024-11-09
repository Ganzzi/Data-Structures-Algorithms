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
    T: Default + PartialEq + Clone + Debug + Display + Eq + Hash,
    Vertex<T, U>: Default + PartialEq,
    U: Clone,
{
    vertices_count: usize,
    edges_count: usize,
    vertices: HashMap<T, Vertex<T, U>>,
}

impl<T, U> Graph<T, U>
where
    T: Default + PartialEq + Clone + Debug + Display + Eq + Hash,
    Vertex<T, U>: Default + PartialEq,
    U: Clone,
{
    pub fn new() -> Self {
        Graph {
            vertices_count: 0,
            edges_count: 0,
            vertices: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.vertices_count == 0
    }

    pub fn vertices_count(&self) -> usize {
        self.vertices_count
    }

    pub fn edges_count(&self) -> usize {
        self.edges_count
    }

    pub fn contains(&self, key: &T) -> bool {
        self.vertices.contains(key)
    }

    pub fn add_vertex(&mut self, key: &T) {
        if !self.contains(key) {
            self.vertices.insert(key.clone(), Vertex::new(key.clone()));
            self.vertices_count += 1;
        }
    }

    pub fn get_vertex(&self, key: &T) -> Option<Vertex<T, U>> {
        self.vertices.get(key).cloned()
    }

    pub fn get_vertex_keys(&self) -> Vec<T> {
        self.vertices.iter().map(|(k, _)| k.clone()).collect()
    }

    pub fn remove_vertex(&mut self, key: &T) -> Option<Vertex<T, U>> {
        if let Some(vertex) = self.vertices.remove(key) {
            self.vertices_count -= 1;

            self.edges_count -= vertex.get_neibors().len();

            for vtx_key in self.get_vertex_keys() {
                if let Some(vtx) = self.vertices.get_mut(&vtx_key) {
                    if vtx.is_adjacent(key) {
                        if let Some(_) = vtx.remove_neibor(key) {
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
            .add_neibor(to.clone(), u);
    }

    pub fn is_adjacent(&self, from: &T, to: &T) -> bool {
        if let Some(vertex) = self.vertices.get(from) {
            vertex.is_adjacent(to)
        } else {
            false
        }
    }
}
