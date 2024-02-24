// --- region: imports
use crate::vec::LinkedVec;
// --- endregion: imports

// --- region: modules
mod vec;
// --- endregion: modules

fn main() {
    // LINKED VEC DATA TYPE
    println!("***LINKED VEC DATA TYPE***");
    let mut lvec1 = LinkedVec::new();
    lvec1.push(10); lvec1.push(11);
    lvec1.push(12); lvec1.push(13);
    lvec1.insert(0,9);
    print!("vec 1: "); lvec1.print(); 
    print!("\nElement at 3: {:?}", lvec1.find(3));

    let mut lvec2 = LinkedVec::new();
    lvec2.insert(0, 8); lvec2.append(&mut lvec1);
    print!("\nvec 2: "); lvec2.print();
    print!("\nlen: {} - ", lvec2.len());
    print!("pop {:?} - ", lvec2.pop().unwrap());
    print!("remove {:?}\n", lvec2.remove(0).unwrap());
    print!("vec 2: "); lvec2.print();

    let sum1 = lvec2.iter().sum::<i32>();
    let mut addend = 0;
    for item in lvec2.iter_mut() {
        *item += 1;
        addend += 1;
    }
    let sum2 = lvec2.iter().sum::<i32>();
    println!("\nsum = {sum1} + {addend} = {sum2}");
}
