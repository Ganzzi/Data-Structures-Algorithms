 
/// A generic queue data structure implemented using a vector.
///
/// # Generic Parameters
///
/// * `T` - Type of elements stored in the queue.
///
/// # Fields
///
/// * `cap` - Maximum capacity of the queue.
/// * `data` - Internal vector storing the elements of the queue.
///
/// # Examples
///
/// ```
/// use queue::Queue;
///
/// let mut queue: Queue<i32> = Queue::new(5);
///
/// // Enqueue elements into the queue.
/// queue.enqueue(1).unwrap();
/// queue.enqueue(2).unwrap();
///
/// // Dequeue an element from the queue.
/// let dequeued = queue.dequeue();
///
/// assert_eq!(dequeued, Some(1));
/// ```
 #[derive(Debug)]
pub struct Queue<T> {
    cap: usize,
    data: Vec<T>
}

impl<T> Queue<T>  {
    /// Creates a new empty queue with the given maximum capacity.
    ///
    /// # Arguments
    ///
    /// * `size` - Maximum capacity of the queue.
    ///
    /// # Returns
    ///
    /// A new empty queue with the specified capacity.
    pub fn new(size: usize) -> Self {
        Queue { cap: size, data: Vec::with_capacity(size) }
    }

    /// Enqueues an element into the queue.
    ///
    /// # Arguments
    ///
    /// * `item` - The element to enqueue.
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, otherwise returns an error if the queue is full.
    pub fn enqueue(&mut self, item: T) -> Result<(), String>{
        if self.cap == self.data.len() {
            return  Err("Max size!".into());
        }

        self.data.insert(0, item);

        Ok(())
    }

    /// Dequeues an element from the queue.
    ///
    /// # Returns
    ///
    /// An optional value containing the dequeued element, or `None` if the queue is empty.
    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Checks if the queue is empty.
    ///
    /// # Returns
    ///
    /// `true` if the queue is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    /// Returns the current size of the queue.
    ///
    /// # Returns
    ///
    /// The number of elements currently in the queue.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Clears the queue, removing all elements.
    pub fn clear(&mut self){
        self.data = Vec::with_capacity(self.cap);
    }

    /// Returns an iterator over the elements of the queue.
    ///
    /// # Returns
    ///
    /// An iterator yielding references to the elements of the queue.
    pub fn iter (&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Returns a mutable iterator over the elements of the queue.
    ///
    /// # Returns
    ///
    /// A mutable iterator yielding references to the elements of the queue.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    /// Consumes the queue and returns an iterator over its elements.
    ///
    /// # Returns
    ///
    /// An iterator consuming the queue and yielding its elements.
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.data.into_iter()
    }
}
