// quiz1.rs
//
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given the
// quantity bought.
//
// No hints this time ;)

// I AM NOT DONE

// Put your function here!
fn main() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    println!("Price1: {}", price1);
    println!("Price2: {}", price2);
    println!("Price3: {}", price3);
    println!("Price4: {}", price4);
}


fn calculate_price_of_apples(quantity: i32) -> i32 {
    if quantity > 40 {
        quantity * 1
    } else {
        quantity * 2
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
// In this exercise, I learned about:

// Conditional Logic: Using if-else statements to apply different pricing based on the quantity of apples.
// Function Implementation: Writing a function that calculates the price of apples based on given rules.
// Edge Cases: Handling conditions when the quantity is exactly 40 or more than 40.
// Testing: Understanding the importance of not modifying test functions to ensure consistency in evaluation.
// What the Test Was Evaluating
// The test was evaluating my ability to:

// Implement conditional logic in a function.
// Calculate the price correctly based on different conditions.
// Ensure the function returns the correct results for various input quantities.
// Write code that passes predefined tests without modifying the test functions.