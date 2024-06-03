// What I Learned from if1.rs
// In this exercise, I learned about:

// Conditional Statements: Using if and else to implement logic based on conditions.
// Returning Values: Returning values from different branches of a conditional statement.

// Key Takeaways:

// Conditional Statements: Using if-else to compare two values and return the appropriate one.
// Returning Values: How to return values from conditional branches.



// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.



pub fn bigger(a: i32, b: i32) -> i32 {
    if a > b{
        return a;
    }
    else{
        return b;
    }
    
    // Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}

