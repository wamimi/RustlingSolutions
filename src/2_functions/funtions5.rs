// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
    // or return num*num;
}
// In this exercise, I learned about:

// Expressions and Statements: Understanding the difference between an expression and a statement in Rust.
// Returning Values: How to return values from a function without using the return keyword by making sure the last line is an expression.
// Key Takeaways:

// Expression vs. Statement: An expression returns a value, while a statement does not.
// Returning Values: In Rust, the last expression in a function is returned as the value of the function without needing the return keyword.