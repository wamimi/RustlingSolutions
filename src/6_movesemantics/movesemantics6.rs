fn main() {
    let data = "Rust is great!".to_string();  // Initialize `data` as a String

    get_char(&data);  // Pass a reference to `data`, not ownership

    string_uppercase(data);  // Pass ownership to `string_uppercase`
}

// Modified to take a reference instead of ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()  // Access the last character without consuming `data`
}

// Correctly takes ownership
fn string_uppercase(data: String) {
    let data = data.to_uppercase();  // Transform `data` to uppercase
    println!("{}", data);  // Print the uppercase `data`
}
/*
Key Concepts Learned:
Ownership and Borrowing:

Enhanced understanding of when to pass ownership and when to pass references. 
Ownership transfers the entire object, while a reference allows temporary, shared access.
Mutable vs. Immutable References:

The use of immutable references (&String) helps prevent data from being altered unintentionally and allows multiple parts of the code to read the data without risk of modification.
Function Signatures and Data Management:

Adjusting function signatures to either accept data by reference or take full ownership, depending on the requirements, teaches careful management of data flow and lifetimes within a Rust program.
Lifetime of Temporary Values:

Dealt with the common pitfall of referencing temporary values, emphasizing the importance of scope and lifetime in memory safety.
*/