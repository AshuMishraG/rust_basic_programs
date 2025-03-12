// Numeric Operators
// fn sub(a: i32, b: i32) -> i32 {
//     a - b
// }
// fn main() {
//     let sum = 2 + 2;
//     let value = 10 - 5;
//     let division = 10 / 2;
//     let multi = 5 * 5; // Ensure the variable name matches
//     let five = sub(8, 3);
//     let rem1 = 6 % 3;
//     let rem2 = 6 % 4;
//
//     println!("Sum: {}", sum);
//     println!("Value: {}", value);
//     println!("Division: {}", division);
//     println!("Multiplication: {}", multi); // Use 'multi' instead of 'mult'
//     println!("Result of sub(8, 3): {}", five);
//     println!("rem 1: {}", rem1);
//     println!("rem 2: {}", rem2);
//}

// Demo.rs exceptions if statements
// fn main() {
//     let age = 15;
//     if age >= 21 {
//         println!("ok to purchase");
//     } else {
//         println!("cannot purchase");
//     }
// }

// match.rs
// fn main() {
//     let my_name = "Bob";
//     match my_name {
//         "Ashutosh" => println!("that is my name"),
//         "Bob" => println!("not my name"),
//         "Alice" => println!("hello Alice"),
//         _ => println!("nice to meet you!"),
//     }
// }

// Repetition using loops
// fn main() {
//     let mut i = 3;
//     loop {
//         println!("{:?}", i);
//         i = i - 1;
//         if i == 0 {
//             break;
//         }
//     }
//     println!("done!");
// }

// Repetition using while
fn main() {
    let mut i = 1;
    while i <= 3 {
        println!("{:?}", i);
        i = i + 1;
    }
}