// variables8.rs
//
// Execute `rustlings hint variables7` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// A borrowing view

fn main() {
    let my_string = "asdfasdfasdfasdsa".to_string();

    let my_string_view = (my_string + "z").as_str();

    println!("{}", my_string_view);
}
