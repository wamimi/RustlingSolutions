// primitive_types2.rs
//
// Fill in the rest of the line that has code missing! No hints, there's no
// tricks, just get used to typing these :)

// I AM NOT DONE

fn main() {
    // Characters (`char`)

    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character = '😚' ; // Finish this line like the example! What's your favorite character?
    // Try a letter, try a number, try a special character, try a character
    // from a different language than your own, try an emoji!
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
// In this exercise, I learned about:

// Character Type (char): How to use character literals in Rust.
// Character Methods: Using methods like .is_alphabetic() and .is_numeric() to check the type of characters.
// The methods .is_alphabetic() and .is_numeric() are built-in methods provided by the Rust standard library for the char type. 
// These methods are part of Rust's comprehensive standard library, which includes many utility functions for various data types.
// Learned about character type (`char`) in Rust
// - Used character methods to check if a character is alphabetic or numeric
// - Practiced working with different types of characters, including emojis