pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Result<T, &'static str> {
        self.data.pop().ok_or("Stack is empty")
    }

    pub fn peek(&self) -> Result<&T, &'static str> {
        self.data.last().ok_or("Stack is empty")
    }

    pub fn peek_mut(&mut self) -> Result<&mut T, &'static str> {
        self.data.last_mut().ok_or("Stack is empty")
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().rev()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().rev()
    }

    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.data.into_iter().rev()
    }
}
