// --- region: imports
use crate::{
    hot_potato::hot_potato, 
    queue::Queue
};
// --- enddregion: imports

// --- region: modules
mod queue;
mod hot_potato;
// --- endregion: modules

fn main() {
    
    // QUEUE DATA TYPE
    println!("\n\n***QUEUE DATA TYPE***");
    let mut queue = Queue::new(4);
    queue.enqueue(1).unwrap(); queue.enqueue(2).unwrap();
    queue.enqueue(3).unwrap(); queue.enqueue(4).unwrap();
    
    if let Err(error) = queue.enqueue(5) {
        println!("Got an error when enqueue 5 to the queue: {error}");
    }
    
    println!("Sum of queue = {}", queue.into_iter().sum::<i32>());


    // HOT POTATO
    println!("\n\n***HOT POTATO***");
    let players = vec!["John", "James", "Marry", "Anna"];
    let winner = hot_potato(players, 23);
    println!("Winner is {winner}");

}