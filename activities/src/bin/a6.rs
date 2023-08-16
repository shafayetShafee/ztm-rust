// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

fn main() {
    let mut n: i32 = 5;

    while n >= 0 {
        if n == 0 {
            println!("done!");
        } else {
            println!("{}", n);
        }
        n -= 1;
    }
}
