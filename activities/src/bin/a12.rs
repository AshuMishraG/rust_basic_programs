// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}
// * Use an enum for the box color
enum Color {
    Brown,
    Red
}
impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("brown"),
            Color::Red => println!("red"),
        }
    }
}
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let small_dimensions = Dimensions {
        width: 10.0,
        height: 20.0,
        depth: 30.0,
    };
    let small_box = ShippingBox::new(5.0, Color::Red, small_dimensions);
    small_box.print();
}