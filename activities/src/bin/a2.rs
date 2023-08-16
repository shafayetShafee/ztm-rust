// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn show_res(r: i32) {
    println!("{:?}", r);
}

fn main() {
    let x: i32 = 5;
    let y: i32 = 11;
    let res = add(x, y);
    show_res(res);
}
