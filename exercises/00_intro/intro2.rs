// intro2.rs
//
// Make the code print a greeting to the world.
// Try different println!
//
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    printline!("Hello there!");

    let x = 7;
    let y = String::from("abc");

    // Try different println!.
    println!("x is {}, y is {}", x, y);
    println!("x is {valx}, y is {fred}", valx = x, fred = y);
    println!("debug {:?}", (3, 4));
    println!("y is {1}, x is {0}", x, y);
}
