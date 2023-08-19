// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Lemon,
    Mango,
}

struct Drink {
    flavor: Flavor,
    oz: f64,
}

fn show_drink_info(drink: &Drink) {
    match drink.flavor {
        Flavor::Lemon => println!("It is a Lemon flavored drink"),
        Flavor::Mango => println!("It is a Mango flavored drink"),
    };
    println!("And the drink weights {} oz", drink.oz);
}

fn main() {
    let drink = Drink {
        flavor: Flavor::Lemon,
        oz: 100.0,
    };

    show_drink_info(&drink);
    show_drink_info(&drink);
}
