// reference01.rs
//
// Execute `rustlings hint reference01` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let mut char_array = ['a', 'b'];

    let first = &char_array[0];
    let second = &char_array[1];

    *first = *second;

    assert_eq!(char_array[0], 'b');
}
