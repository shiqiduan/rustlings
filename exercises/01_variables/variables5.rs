// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);

    // shadow
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of y is: {}", y);

    // shadow and change type
    let abc = "ABC";
    let abc = abc.len();
    println!("The value of abc is: {}", abc);
}
