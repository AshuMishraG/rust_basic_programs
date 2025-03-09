fn sub(a: i32, b: i32) -> i32 {
    a - b
}
fn main() {
    let sum = 2 + 2;
    let value = 10 - 5;
    let division = 10 / 2;
    let multi = 5 * 5; // Ensure the variable name matches
    let five = sub(8, 3);

    println!("Sum: {}", sum);
    println!("Value: {}", value);
    println!("Division: {}", division);
    println!("Multiplication: {}", multi); // Use 'multi' instead of 'mult'
    println!("Result of sub(8, 3): {}", five);
}