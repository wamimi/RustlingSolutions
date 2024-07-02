// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;  // Take a mutable reference to `x`
    *y += 100;       // Immediately use `y` to add 100 to `x`
    let z = &mut x;  // Now that `y` is no longer used, we can re-borrow `x` as `z`
    *z += 1000;      // Use `z` to add 1000 to `x`
    assert_eq!(x, 1200);  // Check that `x` is now 1200

}
/*
Key Concepts Learned:
Mutable References: The primary lesson was the management of mutable references within a single scope.
Rust enforces that only one mutable reference to a data point can be active at a time to prevent data races.

Scope and Borrow Checker: By carefully ordering the lines, I learned how the scope of a mutable reference affects its lifespan and how Rust's borrow checker enforces rules regarding these scopes. 
This understanding is crucial for writing safe and efficient concurrent Rust programs.

Dereferencing Operators: The exercise also helped solidify my understanding of the dereferencing operator *, which is used to access or modify the value pointed to by a reference. 
This is a fundamental aspect of working with pointers and references in Rust.

Sequential Mutable Access: I learned to sequence code to respect Rust's borrowing rules, allowing one mutable reference to be dropped (or its scope to end) before another is created. 
This sequential access pattern is a common strategy in Rust to work within the language's safety guarantees.

*/