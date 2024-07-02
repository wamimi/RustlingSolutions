// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

#[test]
fn main() {
    // Remove vec0 initialization
    // let vec0 = vec![22, 44, 66];

    // Call fill_vec without any parameters
    let vec1 = fill_vec();

    // Verify that vec1 contains the expected elements
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument - don't change this!
fn fill_vec() -> Vec<i32> {
    // Create and fill the vector inside this function
    let mut vec = vec![22, 44, 66]; // Initialize with the required elements

    vec.push(88); // Add the additional element

    vec
}
/*
Key Concepts Learned:
Function Responsibility: This exercise emphasized moving the responsibility of creating and initializing data structures into the function itself, rather than passing them as parameters.
This can simplify the calling code and encapsulate the logic within the function.

Function Signatures: Understanding how to modify function signatures and adapt the calling code accordingly is essential. 
Here, fill_vec was changed to take no parameters and return a newly created vector.

Initialization within Functions: I learned how to initialize and manipulate data structures directly within functions. 
This included:

Creating a new vector.
Initializing it with specific values.
Modifying it before returning.
Ownership and Returning Values: The exercise also reinforced the concept of ownership transfer in Rust. 
By creating and modifying the vector within fill_vec and returning it, we transfer the ownership of the vector to the caller (main function in this case).

Avoiding Unnecessary Parameters: By creating the vector inside the function, we avoided passing unnecessary parameters, making the function more self-contained and reducing dependencies.
*/