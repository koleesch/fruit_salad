/*
This programm creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
powerful data structures in Rust. A vector is represented by Vec<T> where T is the type of the elements.
*/

use rand::seq::SliceRandom; // rand is a random number generator library for Rust
use rand::thread_rng; // thread_rng is a random number generator that is local to the current thread of execution

fn main() {
    let mut fruits = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruits.iter().enumerate() {
        if i != fruits.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    // add fruit from user to fruit salad
    let mut user_fruit = String::new();
    println!("Enter a fruit to add to the fruit salad:");
    std::io::stdin().read_line(&mut user_fruit).unwrap();
    fruits.push(user_fruit.trim());

    // shuffle fruit from salad
    let fruit = fruits.choose(&mut rng).unwrap();
    println!("A shuffled fruit from salad: {}", fruit)
}
