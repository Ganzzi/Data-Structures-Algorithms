#[derive(Debug, Clone, Default, PartialEq)]
pub struct Vertex<T, U> {
    key: T,
    pub neibors: Vec<(T, U)>,
}

impl<T, U> Vertex<T, U>
where
    T: Eq,
    U: Clone,
{
    pub fn new(key: T) -> Self {
        Vertex {
            key,
            neibors: Vec::new(),
        }
    }

    pub fn add_neibor(&mut self, key: T, weight: U) {
        self.neibors.push((key, weight));
    }

    pub fn get_neibors(&self) -> &Vec<(T, U)> {
        &self.neibors
    }

    pub fn get_neibor_keys(&self) -> Vec<&T> {
        self.neibors.iter().map(|(key, _)| key).collect()
    }

    pub fn get_key(&self) -> &T {
        &self.key
    }

    pub fn get_neibor_weight(&self, key: &T) -> Option<&U> {
        self.neibors.iter().find(|(k, _)| k.eq(key)).map(|(_, w)| w)
    }

    pub fn is_adjacent(&self, key: &T) -> bool {
        self.neibors.iter().any(|(k, _)| k.eq(key))
    }

    pub fn remove_neibor(&mut self, key: &T) -> Option<(T, U)> {
        if let Some(index) = self.neibors.iter().position(|(k, _)| k.eq(key)) {
            Some(self.neibors.remove(index))
        } else {
            None
        }
    }
}
