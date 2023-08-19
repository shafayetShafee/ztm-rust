// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Green,
    Blue,
}

fn print_color(c: Color) -> String {
    match c {
        Color::Red => "red color".to_string(),
        Color::Green => "green color".to_string(),
        _ => "Other color".to_string(),
    }
}

fn main() {
    println!("{}", print_color(Color::Green));
}
