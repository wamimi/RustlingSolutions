// structs2.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();
        // TODO: Create your own order using the update syntax and template above!
        let your_order = Order {
            name: String::from("Hacker in Rust"),
            count: 1,
            ..order_template
        };
        // Another way to solve though inneficient is this one below
        /*
        let your_order = Order {
        name: String::from("Hacker in Rust"),
        year: create_order_template().year,
        made_by_phone: create_order_template().made_by_phone,
        made_by_mobile: create_order_template().made_by_mobile,
        made_by_email: create_order_template().made_by_email,
        item_number: create_order_template().item_number,
        count: 1,
    };
         */
        
        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}
/*
Key Learnings
1. Struct Update Syntax Mastery
Through this exercise, I gained a deeper understanding of Rust’s struct update syntax. 
This feature is a powerful tool that allows for the creation of new struct instances by leveraging existing ones. 
By using this syntax, I learned how to efficiently initialize new structs without manually specifying each field, especially when many fields remain unchanged from one instance to another.

2. Efficient Code Practices
Initially, I instantiated several fields by repeatedly calling create_order_template(). 
Through trial and error, I learned that this approach was inefficient. 
It was a practical demonstration of how repeated function calls can lead to redundant code and unnecessary computation. 
The exercise guided me to utilize a single template instance (order_template) for all fields not explicitly set, teaching me a more efficient coding practice.

*/
