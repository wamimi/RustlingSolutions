// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0.clone());

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66,88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}
// Exercise: move_semantics2.rs
// Key Concepts Learned:
// 1. Ownership and Borrowing: This exercise reinforced Rust's ownership rules, 
// where a value (like a Vec<i32>) has a single owner at any point in time. 
// Once the value is moved, the original owner can no longer use it, which is a core part of ensuring memory safety in Rust without a garbage collector.

// 2. Cloning to Retain Ownership: I learned how to use the .clone() method to create a full copy of a data structure. 
//This is useful when you need to retain the original value after passing it to a function.
//  Cloning can be computationally expensive depending on the size of the data, 
//so it should be used judiciously.

// 3. Function Modifications to Handle Ownership: I explored how function signatures can be modified to accept a reference (&Vec<i32>) instead of taking ownership of the argument (Vec<i32>). 
//  This allows the original data to remain unchanged while still enabling the function to read from it.
// I also learned about the trade-offs between cloning data and modifying a function to use references.

//4 Mutable vs Immutable References: In the variant of the solution where I used a reference,
// I encountered Rust's borrowing rules, specifically the distinction between mutable and immutable references. 
// This is foundational for writing safe concurrent code, 
// where Rust ensures that data can either have multiple immutable references or one mutable reference, but not both at the same time.

5. 