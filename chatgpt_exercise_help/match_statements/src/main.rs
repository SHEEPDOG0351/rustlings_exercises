// ------------------------------------------------------------ Problem 1: Number Classifier -------------------------------------------------
// Write a function that takes an i32 and returns a &'static str describing it:

// fn classify_number(n: i32) -> &'static str {
//     match n {
//         // TODO: Fill in the match arms
//         n if n < 0 => "Negative",
//         n if n == 0 => "Zero",
//         n if n % 2 == 0 => "Even Positive",
//         _ => "Odd Positive" // the underscore here just deals with all other possible situations this function could be given. This has to be defined this way since match statements are "exhaustive" meaning they have to cover every possible scenario
//     }
// }

// fn main() {
//     println!("{}", classify_number(-10)); // Expected: "Negative"
//     println!("{}", classify_number(0));   // Expected: "Zero"
//     println!("{}", classify_number(4));   // Expected: "Positive Even"
//     println!("{}", classify_number(7));   // Expected: "Positive Odd"
// }

// ------------------------------------------------------ Problem 2: Shape Area Calculator -----------------------------------------------

// enum Shape {
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64, f64),
// }

// fn calculate_area(shape: Shape) -> f64 {
//     match shape {
//         Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
//         Shape::Square(side) => side * side,
//         Shape::Rectangle(width, height) => width * height,
//     } // since this match statement works off the Shape (enum) data type, it handles all the possible scenarios the match statement could be given
// }

// fn main() {
//     println!("Circle Area: {}", calculate_area(Shape::Circle(3.0))); // Expected: ~28.27
//     println!("Square Area: {}", calculate_area(Shape::Square(4.0))); // Expected: 16.0
//     println!("Rectangle Area: {}", calculate_area(Shape::Rectangle(3.0, 5.0))); // Expected: 15.0
// }

// ------------------------------------------------------------- Simple Vending Machine -----------------------------------------------------

enum Drink {
    Water,
    Soda,
    Juice,
}

fn get_price(drink: Drink) -> u32 {
    match drink {
        // TODO: Match the correct price for each drink
        Drink::Juice => 3,
        Drink::Soda => 2,
        Drink::Water => 1
    }
}

fn main() {
    println!("Water costs: ${}", get_price(Drink::Water)); // Expected: $1
    println!("Soda costs: ${}", get_price(Drink::Soda));   // Expected: $2
    println!("Juice costs: ${}", get_price(Drink::Juice)); // Expected: $3
}
