// variables6.rs
//
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// const are not the same as immutable
// 1. constants are't just immutable by default, they're always muttables.
// 2. use const keyword, type must be annotated
// 3. May be set only to a constant expresion, but should not be a fucntion call.
// 4. namming convention

const NUMBER_FOR_memory = 3;
fn main() {
    println!("Number {}", NUMBER_FOR_memory);

    NUMBER = 4;
    println!("Number {}", NUMBER_FOR_memory);
}
