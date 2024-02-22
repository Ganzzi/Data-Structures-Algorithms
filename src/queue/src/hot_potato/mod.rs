use queue::queue::Queue;

/// Hot Potato game
///
/// Simulates the "Hot Potato" game where a potato is passed around in a circle
/// and whoever is holding the potato when the game ends is out.
///
/// # Arguments
///
/// * `names` - A vector containing the names of players participating in the game.
/// * `num` - The number of passes after which the player holding the potato is removed.
///
/// # Returns
///
/// * A reference to the name of the player left in the game after all eliminations.
///
/// # Example
///
/// ```
/// use hot_potato_game::hot_potato;
///
/// let names = vec!["Alice", "Bob", "Charlie", "David", "Emma"];
/// let winner = hot_potato(names, 3);
/// assert_eq!(winner, "Charlie");
/// ```
pub fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    let mut queue = Queue::new(names.len());

    for name in names.iter().rev() {
        let _ = queue.enqueue(*name);
    }

    while queue.size() > 1 {
        for _ in 0..num {
            let front_name = queue.dequeue().unwrap();
            let _ = queue.enqueue(front_name);
        }

        queue.dequeue();
    }

    queue.dequeue().unwrap()
}
