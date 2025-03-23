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
// fn main() {
//     let mut i = 1;
//     while i <= 3 {
//         println!("{:?}", i);
//         i = i + 1;
//     }
// }

// Enum with Match
// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }
// fn main() {
//     let go = Direction::Up;
//     match go {
//         Direction::Up => println!("Go up"),
//         Direction::Down => println!("Go down"),
//         Direction::Left => println!("Go left"),
//         Direction::Right => println!("Go right"),
//     }
// }

// Struct
// struct GroceryItem {
//     stock: i32,
//     price: f64,
// }
// fn main() {
//     let cereal = GroceryItem {
//         stock: 10,
//         price: 2.99,
//     };
//     println!("Stock: {:?}", cereal.stock);
//     println!("Price: {:?}", cereal.price);
// }

// Tuples
// fn main() {
//     let coord = (2, 3);
//     println!("{:?}, {:?}", coord.0, coord.1);
//
//     let (x, y) = (2, 3);
//     println!("{:?}, {:?}", x, y);
//
//     let favorites = ("red", 14, "TX", "Pizza");
//
//     let state = favorites.2;
//     let place = favorites.3;
//
//     println!("{:?}, {:?}", state, place);
// }

// Expressions
// enum Access {
//     Admin,
//     Manager,
//     User,
//     Guest,
// }
// fn main() {
//     let access_level = Access::Guest;
//     let can_access_file = match access_level {
//         Access::Admin => true,
//         _ => false,
//     };
//     println!("can access: {:?}", can_access_file);
// }

// Ownership
// struct Book {
//     pages: i32,
//     rating: i32,
// }
// fn display_page_count(book: &Book) {
//     println!("pages = {:?}", book.pages);
// }
// fn display_rating(book: &Book) {
//     println!("rating = {:?}", book.rating);
// }
// fn main() {
//     let book = Book {
//         pages: 5,
//         rating: 9,
//     };
//     display_page_count(&book);
//     //no longer exits display_rating(book);
//     display_rating(&book);
// }

// Impl
// struct Temperature {
//     degrees_f: f64,
// }
// impl Temperature {
//     fn freezing() -> Self {
//         Self { degrees_f: 32.0 }
//     }
//     fn boiling() -> Self {
//         Self { degrees_f: 212.0 }
//     }
//     fn show_temp(&self) {
//         println!("{:?} degree F", self.degrees_f);
//     }
// }
// fn main() {
//     let hot = Temperature { degrees_f: 99.9};
//     hot.show_temp();
//
//     let cold = Temperature::freezing();
//     cold.show_temp(); // can use this multiple times
//
//     let boiling = Temperature::boiling();
//     boiling.show_temp();
// }

// Vectors
// struct Test {
//     score: i32,
// }
// fn main() {
//     let my_scores = vec![
//         Test { score: 90 },
//         Test { score: 100 },
//         Test { score: 200 },
//         Test { score: 300 },
//     ];
//     for test in my_scores {
//         println!("score = {:?}", test.score);
//     }
// }

// Strings
// struct LineItem {
//     name: String,
//     count: i32,
// }
// fn print_name(name: &str) {
//     println!("name: {:?}", name);
// }
// fn main() {
//     let receipt = vec![
//         LineItem {
//             name: "cereal".to_owned(),
//             count: 1,
//         },
//         LineItem {
//             name: String::from("fruit"),
//             count: 3,
//         },
//     ];
//     for item in receipt {
//         print_name(&item.name);
//         println!("count: {:?}", item.count);
//     }
// }

// Derive
// #[derive(Debug, Clone, Copy)]
// enum Position {
//     Manager,
//     Supervisor,
//     Worker
// }
// #[derive(Debug, Clone, Copy)]
// struct Employee {
//     aaaa: Position,
//     work_hours: i64
// }
// // creating copies hence not changing the ownership to the print statement
// fn print_employee(emp: Employee) {
//     println!("{:?}", emp);
// }
// fn main() {
//     let me = Employee {
//         position: Position::Worker,
//         work_hours: 40
//     };
//     // using Clone and Copy now
//     print_employee(me);
//     print_employee(me);
//     // using debug on struct as well
//     // println!("{:?}", me);
//     // using debug on enum
//     // println!("{:?}", me.position);
//     // without using debug
//     // match me.position {
//     //     Position::Manager => println!("Manger"),
//     //     Position::Supervisor => println!("Supervisor"),
//     //     Position::Worker => println!("Worker")
//     // }
// }

// Advanced Match
// enum Discount {
//     Percent(i32),
//     Flat(i32),
// }
// struct Ticket {
//     event: String,
//     price: i32,
// }
// fn main() {
//     let n = 3;
//     match n {
//         3 => println!("three"),
//         // _ => println!("number: {:?}", n),
//         other => println!("number: {:?}", other),
//     }
//     let flat = Discount::Flat(2);
//     match flat {
//         Discount::Flat(2) => println!("flat 2"),
//         Discount::Flat(amount) => println!("flat discount of {:?}", amount),
//         _ => (),
//     }
//     let concert = Ticket {
//         event: "Concert".to_owned(),
//         price: 10,
//     };
//     match concert {
//         Ticket { price: 10, event} => println!("event @ 10 = {:?}", event),
//         Ticket { price, .. } => println!("price = {:?}", price),
//     }
// }

// Options
// struct Survey {
//     q1: Option<i32>,
//     q2: Option<bool>,
//     q3: Option<String>,
// }
// fn main() {
//     let response = Survey {
//         q1: Some(12),
//         q2: Some(true),
//         q3: Some("A".to_owned()),
//     };
//
//     match response.q1 {
//         Some(ans) => println!("q1: {:?}", ans),
//         None => println!("q1: no response"),
//     }
//     match response.q2 {
//         Some(ans) => println!("q2: {:?}", ans),
//         None => println!("q1: no response"),
//     }
//     match response.q3 {
//         Some(ans) => println!("q3: {:?}", ans),
//         None => println!("q1: no response"),
//     }
// }

// Documentation
/// A favourite color.
enum Color {
    Red,
    Blue,
}
/// A piece of mail.
struct Mail {
    address: String,
}
/// Adds two numbers together.
fn add (a: i32, b: i32) -> i32 {
    a + b
}
fn main() { }