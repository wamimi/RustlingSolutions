//In this exercise, I learned about:

// Conditional Logic: Using if-else if-else statements to handle multiple conditions.
// String Comparisons: Comparing string slices (&str) in conditional statements.
// Function Return Types: Ensuring consistent return types across all branches.

// Key Takeaways:

// Handling Multiple Conditions: Using else if to add more conditions.
// Consistency: Ensuring all branches return the same type (&str).




// if2.rs
//
// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
//
// Execute `rustlings hint if2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "fuzz"{
        "bar"
    }
    else{
        "baz"
    }
}

// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }
}
