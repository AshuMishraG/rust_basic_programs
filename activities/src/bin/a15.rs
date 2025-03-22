// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    // * Create one of each ticket and place into a vector
    let tickets = vec![
        Ticket::Backstage(100.0, "Billy".to_owned()),
        Ticket::Standard(50.0),
        Ticket::Vip(200.0, "John".to_owned()),
    ];

    // * Use a match expression while iterating the vector to print the ticket info
    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Backstage ticket for {}: ${:.2}", holder, price);
                println!("Backstage ticket {}: ${:.2}", holder, price)
            }
            Ticket::Standard(price) => println!("Standard ticket for {}", price),
            Ticket::Vip(price, holder) => {
                println!("Vip ticket for {}: ${:.2}", holder, price)
            }
        }
    }
}