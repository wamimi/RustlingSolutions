// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    call_me(5);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
// In this exercise, I learned about:

// Function Parameters and Arguments: How to define functions with parameters and call them with arguments.
// Function Invocation: Calling a function with an argument to pass data to it.
// For Loops and Ranges: Using a range (0..num) in a for loop to iterate through a series of values.
// This exercise reinforced my understanding of function parameters, how to pass arguments to functions, 
// and iterating with for loops using ranges in Rust.