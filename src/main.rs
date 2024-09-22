/*
This programm creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
powerful data structures in Rust. A vector is represented by Vec<T> where T is the type of the elements.
*/

use rand::seq::SliceRandom; // rand is a random number generator library for Rust
use rand::thread_rng; // thread_rng is a random number generator that is local to the current thread of execution

fn main() {
    let mut fruit = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];

    let mut rng = thread_rng(); // create a random number generator
    fruit.shuffle(&mut rng); // shuffle the fruit vector
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
