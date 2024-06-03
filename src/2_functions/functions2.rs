// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    call_me(3);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
// In this exercise, I learned about:

// Function Parameters: Functions can take parameters, and each parameter must have a type annotation.
// Type Annotations: Specifying the type of parameters is mandatory in Rust. This ensures type safety and clarity.
// For Loops: Using ranges in for loops to iterate over a set of values.
// Range Syntax (0..num): The .. operator creates a range from 0 to num (exclusive),
// meaning it includes values from 0 up to num - 1.
