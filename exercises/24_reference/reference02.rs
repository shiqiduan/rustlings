// reference01.rs
//
// Execute `rustlings hint reference01` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// lifetime
fn my_push_back(v: &mut Vec<&str>, s: &str) {
    v.push(s);
}

fn main() {
    let mut my_vector: Vec<&str> = Vec::new();

    {
        let my_string = String::from("abc");
        my_push_back(&mut my_vector, &my_string);
    }

    println!("{:?}", my_vector);
}
