// variables7.rs
//
// Execute `rustlings hint variables7` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// A dangling pointer

fn main() {
    let my_int_ptr;
    {
        let my_int = 10;
        my_int_ptr = &my_int;
        println!("{}", my_int_ptr);
    }
    println!("{}", my_int_ptr);
}
