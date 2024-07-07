// enums1.rs
//
// No hints this time! ;)

// I AM NOT DONE
/*
Key Learning Objectives
Enum Basics:

Learned how to define an enum in Rust with multiple variants. This is crucial for representing a set of related values compactly and safely.
Debugging with Enums:

Explored the importance of the Debug trait for enums, which allows printing enum variants for debugging purposes, using the {:?} formatter.

Enum Declaration: Defined a Message enum with variants like Quit, Echo, Move, and ChangeColor to represent different actions or signals in an application.
Usage of Debug Trait: Implemented the Debug trait using #[derive(Debug)] to facilitate easy debugging and logging of the enum's state.
Learning Highlights
Understanding Debug vs. Display:

Debug Trait: Allows for quick debugging outputs of any form, making it simpler to inspect the state of enum values during development.
Display Trait: Unlike Debug, Display is meant for user-facing outputs and requires manual implementation to tailor the output format. It’s used when you need more control over how information is presented to the end user.
Practical Application:

Utilized println! with the {:?} formatter to demonstrate how enums can be logged or debugged in applications, reinforcing the practicality of enums in handling multiple predefined states effectively.
*/

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
}
fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
