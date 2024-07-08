// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

/*
Key Learning Objectives
Complex Enum Variants:

Enhanced knowledge on how to define enums with complex data types such as structs and tuples within variants.
Learned to manage different forms of data (single values, tuples, or named fields) directly in enum variants.

Method Implementation on Enums:

Implemented a method on the enum to utilize the Debug trait for printing purposes, reinforcing how enums interact with Rust's ownership and borrowing principles.





*/

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(i32,i32,i32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
