pub struct Bucket {
    pub hasher: i32,
    pub values: Vec<i32>,
}

impl Bucket {
    /// Creates a new `Bucket` with the specified hasher.
    pub fn new(hasher: i32) -> Self {
        Bucket {
            hasher,
            values: Vec::new(),
        }
    }

    /// Creates a new `Bucket` with the specified hasher and value.
    pub fn with_value(hasher: i32, value: i32) -> Self {
        Bucket {
            hasher,
            values: vec![value],
        }
    }

    /// Adds a value to the bucket.
    pub fn add(&mut self, value: i32) {
        self.values.push(value);
    }
}
