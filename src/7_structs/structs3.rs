// structs3.rs
//
// Structs contain data, but can also have logic. In this exercise we have
// defined the Package struct and we want to test some logic attached to it.
// Make the code compile and the tests pass!
//
// Execute `rustlings hint structs3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
// This exercise, structs3.rs, focuses on understanding and implementing methods within the Package struct in Rust. It serves as an excellent practice for integrating business logic directly within data structures through methods that assess conditions and compute values based on the struct’s properties.

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Package {
        if weight_in_grams < 10 {
            // This is not how you should handle errors in Rust,
            // but we will learn about error handling later.
            panic!("Can not ship a package with weight below 10 grams.")
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    fn is_international(&self) -> bool {
        
        self.sender_country != self.recipient_country // Something goes here...
    }

    fn get_fees(&self, cents_per_gram: u32) -> u32{
        // Something goes here
        self.weight_in_grams * cents_per_gram
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
/*
Key Learning Objectives
Struct Implementation:

Defined and instantiated a Package struct with attributes such as sender_country, recipient_country, and weight_in_grams.
Conditional Logic in Methods:

Implemented methods like is_international() to check if a package needs to be sent internationally, based on comparing the sender and recipient countries.
Calculation Methods:

Created a get_fees() method to calculate the shipping fees based on the package’s weight and a rate in cents per gram.
Key Concepts Learned
Using panic! for Error Handling:

Learned to use the panic! macro effectively to handle error conditions that are unrecoverable within the context of business logic, such as ensuring a package meets a minimum weight requirement before processing.
Understood that panic! halts program execution, making it suitable for situations where continuing could lead to incorrect operations or outcomes.
Difference between panic! and println!:

panic!: Utilized for abrupt termination of the program when a non-recoverable error is encountered. It ensures that the error state is clearly signaled and that no further code execution can compound the issue.
println!: Recognized as a tool for outputting information to the console, useful for debugging or user notifications during normal program operations. It does not influence the program's flow or error handling.
Error Messaging:

Discussed the importance of clear error messages within panic! calls to aid in debugging and maintaining the code.

*/