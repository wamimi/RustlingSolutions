// functions4.rs
//
// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off. (Don't worry
// about the function bodies themselves, we're only interested in the signatures
// for now. If anything, this is a good way to peek ahead to future exercises!)
//
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
    println!("the number is {}",is_even(original_price));

}

fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
// What I Learned from functions4.rs
// In this exercise, I learned about:

// Function Signatures: How to correctly define the return type of functions in Rust.
// Conditional Logic in Functions: Implementing logic to return different values based on conditions.
// Calling Functions: How to call one function from within another.
// Rust's Type System: Ensuring functions return the correct type as specified in their signatures.
// Key Takeaways:

// Function Return Types: Specifying the return type using ->.
// Conditional Statements: Using if-else to implement logic based on conditions.
// Function Calls: Calling is_even from within sale_price and from main.
// Type Checking: Ensuring function signatures match their implementation.