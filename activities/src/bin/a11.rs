// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
struct GroceryItem{
    // * Use two i32 fields for the quantity and id number
    quantity: i32,
    id: i32,
}
// * Create a function to display the quantity, with the struct as a parameter
fn display_quantity(item: &GroceryItem) {
    println!("quantity: {:?}", item.quantity);
}
// * Create a function to display the id number, with the struct as a parameter
fn display_id(item: &GroceryItem) {
    println!("id: {:?}", item.id);
}

fn main() {
    let my_item = &GroceryItem {
        quantity: 3,
        id: 99,
    };
    display_id(my_item);
    display_quantity(my_item);
    // although better to use it here & for borrowing and keeping the my_item as owner
}