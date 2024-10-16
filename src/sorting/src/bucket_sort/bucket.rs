pub struct Bucket {
    pub hasher: i32,
    pub values: Vec<i32>,
}

impl Bucket {
    pub fn new(hasher: i32) -> Self {
        Bucket {
            hasher,
            values: Vec::new(),
        }
    }

    pub fn with_value(hasher: i32, value: i32) -> Self {
        Bucket {
            hasher,
            values: vec![value],
        }
    }

    pub fn add(&mut self, value: i32) {
        self.values.push(value);
    }
}
