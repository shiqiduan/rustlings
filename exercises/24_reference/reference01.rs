// reference01.rs
//
// Execute `rustlings hint reference01` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::f32;

fn main() {
    let mut my_vector: Vec<&str> = Vec::new();

    {
        let my_string = String::from("abc");
        my_vector.push(&my_string);
    }

    println!("{:?}", my_vector);
}
