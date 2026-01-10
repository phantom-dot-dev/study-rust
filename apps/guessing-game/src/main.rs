// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io}; // importing libraries in a single line

use rand::Rng;
// importing a library is called `prelude` in rust. prelude = `an action or event serving as an introduction to something more important`
// std stands for `standard` library. `std::io` makes the input/output standard library available to the scope
// here, we're using `rand` library/caret's `Rng` trait
// The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.

fn main() {
    println!("Guess the number!");

    // creating the secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");

    println!("Please input your guess.");

    // The equal sign (=) tells Rust we want to bind something to the variable
    let mut guess = String::new();

    /*
    * The `::` syntax in the `::new` line indicates that new is an associated function of the `String` type. 
    * An associated function is a function that’s implemented on a type, in this case String. 
    * You’ll find a new function on many types because it’s a common name for a function that makes a new value of some kind.
    * If we hadn’t imported the io module with use std::io; at the beginning of the program, we could still use the function by writing this function call as std::io::stdin. 
    * The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
    */

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data 
    // without needing to copy that data into memory multiple times. 
    // like variables, references are immutable, so we need to specify `&mut` to mutate

    /*
     * read_line function returns a Result type. Result is an enumeration (Enum), often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.
     * Result’s variants are Ok and Err. The Ok variant indicates the operation was successful, and it contains the successfully generated value. The Err variant means the operation failed, and it contains information about how or why the operation failed.
     * Values of the Result type, like values of any type, have methods defined on them. An instance of Result has an expect method that you can call. 
     * If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect. 
     * If the read_line method returns an Err, it would likely be the result of an error coming from the underlying operating system. 
     * If this instance of Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so that you can use it. 
     * Without expect, the program will compile, but issue warning showing the `error case is not handled`
     * In simple terms, expect is used to handle the error case
    */

    /*
    Convert the user input into number type u32
    - (Shadowing) creating a new variable of same name but different type (u32 here)
    - Shadowing is feature is often used when you want to convert a value from one type to another type
    - `trim` will remove any white-space or newline `\n` as user must press `return` to satisfy read_line function 
    - The parse method on strings converts a string to another type
    - u32 for `positive number`
     */
    let guess: u32 = guess.trim().parse().expect("Please type a number");


    /*
    Comparing the Guess to the Secret Number
     */
   loop {
     match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("You win");
            break;
        },
    }
   }

}
