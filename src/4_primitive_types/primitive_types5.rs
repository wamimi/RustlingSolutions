// In this exercise, I learned about:

// Tuple Destructuring: How to destructure a tuple in Rust to access its individual elements.

// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age)/* your pattern here */ = cat; // Destructure the tuple into `name` and `age`

    println!("{} is {} years old.", name, age);
//     Explanation:
// Tuple Destructuring: The code let (name, age) = cat; destructures the cat tuple, assigning "Furry McFurson" to name and 3.5 to age.
// Printing Values: The println! macro uses the destructured variables to print the message.
}

