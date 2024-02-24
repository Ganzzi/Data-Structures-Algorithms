/// A generic stack data structure implemented using a vector.
///
/// # Generic Parameters
///
/// * `T` - Type of elements stored in the stack.
///
/// # Fields
///
/// * `data` - Internal vector storing the elements of the stack.
///
/// # Examples
///
/// ```
/// use stack::Stack;
///
/// let mut stack: Stack<i32> = Stack::new();
///
/// // Push elements onto the stack.
/// stack.push(1);
/// stack.push(2);
///
/// // Pop an element from the stack.
/// let popped = stack.pop();
///
/// assert_eq!(popped, Ok(2));
/// ```
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates a new empty stack.
    ///
    /// # Returns
    ///
    /// A new empty stack.
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    /// Pushes an element onto the stack.
    ///
    /// # Arguments
    ///
    /// * `item` - The element to push onto the stack.
    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    /// Pops an element from the stack.
    ///
    /// # Returns
    ///
    /// The popped element, or an error if the stack is empty.
    pub fn pop(&mut self) -> Result<T, &'static str> {
        self.data.pop().ok_or("Stack is empty")
    }

    /// Returns a reference to the top element of the stack without removing it.
    ///
    /// # Returns
    ///
    /// A reference to the top element of the stack, or an error if the stack is empty.
    pub fn peek(&self) -> Result<&T, &'static str> {
        self.data.last().ok_or("Stack is empty")
    }

    /// Returns a mutable reference to the top element of the stack without removing it.
    ///
    /// # Returns
    ///
    /// A mutable reference to the top element of the stack, or an error if the stack is empty.
    pub fn peek_mut(&mut self) -> Result<&mut T, &'static str> {
        self.data.last_mut().ok_or("Stack is empty")
    }

    /// Clears the stack, removing all elements.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Checks if the stack is empty.
    ///
    /// # Returns
    ///
    /// `true` if the stack is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the current size of the stack.
    ///
    /// # Returns
    ///
    /// The number of elements currently in the stack.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Returns an iterator over the elements of the stack.
    ///
    /// # Returns
    ///
    /// An iterator yielding references to the elements of the stack in reverse order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().rev()
    }

    /// Returns a mutable iterator over the elements of the stack.
    ///
    /// # Returns
    ///
    /// A mutable iterator yielding references to the elements of the stack in reverse order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().rev()
    }

    /// Consumes the stack and returns an iterator over its elements.
    ///
    /// # Returns
    ///
    /// An iterator consuming the stack and yielding its elements in reverse order.
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.data.into_iter().rev()
    }
}
