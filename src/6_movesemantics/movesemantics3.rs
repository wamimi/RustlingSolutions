// move_semantics3.rs
//
// Make me compile without adding new lines -- just changing existing lines! (no
// lines with multiple semicolons necessary!)
//
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let  vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec( mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}
/*
Exercise: move_semantics3.rs
Key Concepts Learned:
Mutable Ownership: This exercise deepened my understanding of mutable ownership in Rust. 
By changing the function parameter from vec: Vec<i32> to mut vec: Vec<i32>, I learned how Rust requires explicit permissions to modify data structures. 
This change signifies that the vec inside the function is not just any vector, but a mutable vector that can be altered.

Function Parameter Mutability: I realized the importance of declaring mutability in function parameters when you need to modify the argument within the function.
Rust’s safety and concurrency guarantees come from these strict, but clear rules about mutability and ownership.

In-place Modifications: By making the vector mutable within the function, I was able to modify vec directly by pushing an element to it. 
This approach is memory efficient as it avoids creating unnecessary copies of the data structure, unlike cloning.

Ownership Transfer: The exercise also illustrated the concept of ownership transfer. When vec0 is passed to fill_vec, ownership is transferred to the function parameter vec. Inside the function, since vec is mutable, it can be changed and then returned, effectively transferring the ownership back with the modifications.
*/