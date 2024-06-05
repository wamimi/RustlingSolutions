// In this exercise, I learned about:

// Array Declaration: How to declare an array with a specific number of elements.
// Array Initialization: Initializing all elements in the array with a specific value.
// Checking Array Length: Using the .len() method to check the length of an array.


// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    // learnt how to declare an array with a certain number of elements
    let a: [i32; 100] = [0; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
