// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

struct ColorClassicStruct {
    red : u32,
    green : u32,
    blue : u32,
    // TODO: Something goes here
}

struct ColorTupleStruct( u32, u32, u32);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            red : 0,
            green : 255,
            blue : 0,


        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct(0,255,0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
/*
Classic C Structs (ColorClassicStruct):

Defined with named fields (red, green, blue), this struct is perfect for scenarios where clear identification of each component is necessary. 
It provides an intuitive way to represent and manipulate data attributes individually.
Practical Use: By instantiating the ColorClassicStruct for a specific color, I practiced how to organize data logically, making the code self-documenting and easier to understand. 
For example, modifying or accessing the green component directly by name enhances code clarity.
I learned how to perform unit tests that validate each component's value, ensuring that the struct behaves as expected. 
This is crucial for debugging and maintaining high code quality.

Tuple Structs (ColorTupleStruct):

Tuple structs are essentially named tuples. They are beneficial when the data structure needs a type name but does not require the fields to be named, thus simplifying the code.
This struct type is utilized for lighter and less verbose data handling situations.
For instance, creating a ColorTupleStruct(0, 255, 0) is straightforward and efficient for passing color data where the field names are implicit.
Testing with tuple structs demonstrated how to access fields by index, which is slightly less readable but very efficient. 
Understanding how to manipulate these indices is key for working with more compact data structures.

Unit-Like Structs (UnitLikeStruct):

Defined without any fields, unit-like structs are useful in scenarios where type itself carries significance, such as signaling or tagging elements.
Used in the exercise to show how these structs can be employed in logging or display messages, where their existence marks a condition or triggers a behavior without necessarily carrying data.
The use of format!("{:?}s are fun!", unit_like_struct); illustrated how to implement Rust’s Debug trait for unit-like structs, allowing them to be printable and more versatile in debugging scenarios.

*/
