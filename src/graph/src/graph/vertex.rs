#[derive(Debug, Clone, Default, PartialEq)]
pub struct Vertex<T, U> {
    key: T,
    pub neighbors: Vec<(T, U)>,
}

impl<T, U> Vertex<T, U>
where
    T: Eq,
    U: Clone,
{
    pub fn new(key: T) -> Self {
        Vertex {
            key,
            neighbors: Vec::new(),
        }
    }

    pub fn add_neighbor(&mut self, key: T, weight: U) {
        self.neighbors.push((key, weight));
    }

    pub fn get_neighbors(&self) -> &Vec<(T, U)> {
        &self.neighbors
    }

    pub fn get_neighbor_keys(&self) -> Vec<&T> {
        self.neighbors.iter().map(|(key, _)| key).collect()
    }

    pub fn get_key(&self) -> &T {
        &self.key
    }

    pub fn get_neighbor_weight(&self, key: &T) -> Option<&U> {
        self.neighbors
            .iter()
            .find(|(k, _)| k.eq(key))
            .map(|(_, w)| w)
    }

    pub fn is_adjacent(&self, key: &T) -> bool {
        self.neighbors.iter().any(|(k, _)| k.eq(key))
    }

    pub fn remove_neighbor(&mut self, key: &T) -> Option<(T, U)> {
        if let Some(index) = self.neighbors.iter().position(|(k, _)| k.eq(key)) {
            Some(self.neighbors.remove(index))
        } else {
            None
        }
    }
}
