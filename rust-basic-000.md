### Overviews:
* Rust is a statically typed language, which means that it must know the types of all variables at compile time

### Cargo:
Creating project : `cargo init` or `cargo new <project name>`. also run `cargo --help` or `cargo help init` for more option.

Build and run at the same time : `cargo run`
Build only: `cargo build` and then navigate to `target/debug` dir and run `./<package_name>`

`cargo check` only to quickly check if there are any compiler errors, 

Building for release: `cargo build --release`


Ongoing : Guessing game https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html


### String interpolation:
Rust use macro like `std::fmt` for format string.

```rust
let name = "Alice";
let age = 30;
let s = format!("{} is {} years old.", name, age);
// Result: "Alice is 30 years old."

let s2 = format!("{n} is {a} years old.", n = name, a = age); // variables can be injected with `named-argument`
let s3 = format!("{name} is {age} years old."); // variable can be put inside of the braces

// Expression should not be inside of the {}, ie, method call or complex computation
let width = 10;
let height = 5;
println!("Area: {}", width * height); // result of evaluating an expression should be put outside of the braces

// Raw String 
let world = "lovely world";
println!(r#"hello "{world}""#); // Output: hello "lovely world"
```

### Using Libraries (Carets) | version management with cargo | Cargo.toml or `cargo add <crate>`:

`Cargo.toml` (Tom's obvious minimal language) is the place for versioning. 
Inside, each section is divided by a header, like `[package]`, `[dependencies]` etc. 
`Cargo.lock` file is used internally by cargo to track dependencies versions, not for manual editing.

`cargo update` command will ignore the `Cargo.lock` file, and will update the latest minor version. IE, `0.7.1` will update to maximum of `0.7.9` if available, and also will notify for available latest major release. To update to major release, editing the `Cargo.toml` is the option to go.

* always run `cargo build` or `cargo run` to compile the packages

```toml
[package]
name = "guessing-game"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8.5"
```

### Error Description:
The compiler will throw error and error number as `error[<error_number>]`
- `rustc --explain <error_number>` command will show some more description about the error

### Guessing Game:
A simple terminal game, the program will generate a random number between 1 & 100, and will ask the user to guess the exact number until it's correct.

```rust
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

    // println!("Please input your guess.");

    // The equal sign (=) tells Rust we want to bind something to the variable
    // let mut guess = String::new();

    /*
     * The `::` syntax in the `::new` line indicates that new is an associated function of the `String` type.
     * An associated function is a function that’s implemented on a type, in this case String.
     * You’ll find a new function on many types because it’s a common name for a function that makes a new value of some kind.
     * If we hadn’t imported the io module with use std::io; at the beginning of the program, we could still use the function by writing this function call as std::io::stdin.
     * The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
     */

    // replace by same code inside of the loop block
    // io::stdin()
    // .read_line(&mut guess)
    // .expect("Failed to read line");

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
    // replaced by match and error handling inside the loop
    // let guess: u32 = guess.trim().parse().expect("Please type a number");

    /*
    Comparing the Guess to the Secret Number
     */
    loop {
        let mut guess = String::new();
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
```

### Variable `let` and Constant `const`:
Variables `let variable_name` are immutable by default, to make it mutable, use `mut` keyword as `let mut variable_name`. Naming convention is `snake_case`

Constants are always immutable, mut cannot be used with const and cannot be evaluated during runtime or based on another variable. `const CONSTANT_NAME` to define a constant. Naming Convention is `SCREAMING_SNAKE_CASE`

`shadowing` - new variable with the same name as a previous variable (both same or different scope), hence the first variable is shadowed by the second.

* `let` is required to create a shadowed variable.
* By creating shadowed variable using let (instead of `let mut`), we can perform a few transformations on a value but have the variable be immutable after those transformations have completed.

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

// The value of x in the inner scope is: 12
// The value of x is: 6
```
* Shadowing can also done with same name but different data types. Which makes it handy and powerful

```rust
let mut spaces = "   ";
spaces = spaces.len();
```

* But, mutable variable doesn't support this type of conversion

```rust
let mut spaces = "   ";
spaces = spaces.len(); // will throw compiler error, as mutable variable cannot change type
```

### Rust naming convention: 
- Local Variables & Function Parameters: Use snake_case (all lowercase, words separated by underscores), e.g., let user_id = 1;, let file_name = "data";.

- Constants (const) & Statics (static): Use SCREAMING_SNAKE_CASE (all uppercase, words separated by underscores), e.g., const MAX_TIMEOUT: u32 = 30;.

- Struct Fields: Use snake_case.

- Types, Structs, Enums, Traits: Use UpperCamelCase (PascalCase)

- Enum Variants: Use UpperCamelCase, e.g., MyOption::SomeValue.

- Functions & Methods: Use snake_case, e.g., fn calculate_total() {}.

- Modules: Use snake_case.

- Crates: Use snake_case

### Data types | scalar (single) and compound:
Primarily can be divided into 2 types
- Scalar types: A scalar type represents a single value. Rust has four primary scalar types: `integers` (signed `i` or unsigned `u`), `floating-point` numbers (always signed following the IEEE-754 standard, `f32 and f64`), `Booleans`, and `characters`.

- Compound types: Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

### Integers and Floating-Point number | Scalar:

* signed `i` vs unsigned `u` : when the number needs to have a sign (`+` or `-` sign) with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned)

* Each signed variant can store numbers from `−2^(n − 1)` to `2^(n − 1) − 1` inclusive, where n is the number of bits that variant uses. So, an i8 can store numbers from −(2^7) to 2^7 − 1, which equals −128 to 127. Unsigned variants can store numbers from 0 to `2^n − 1`, so a u8 can store numbers from 0 to `2^8 − 1`, which equals 0 to 255.

* Integer overflow: when an integer is out of the types bound. ie, type u8 that can hold values between 0 and 255, storing 256 will cause `integer overflow`. In Rust's debug mode, it will cause `panic` at runtime (panicking = when a program exits with an error). In `--release` mode, rust will not check for integer overflow. instead, if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on.

* To explicitly handle the possibility of overflow, use `wrapping_*` methods, such as `wrapping_add`. Some more are `checked_*`, `overflowing_*`, `saturating_*` etc

- Numeric operations

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
```

### Boolean `bool` and Character `char` | Scalar:
- Booleans `bool` are one byte in size. It can be either `true` or `false`. 

- Rust’s `char` type is the language’s most primitive alphabetic type. Defined with `single-quote` (Strings are defined by `double-quote`). `char` type is 4 bytes in size and represents a Unicode scalar value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emojis; and zero-width spaces are all valid char values in Rust.

```rust
fn main() {
    // Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    // Character
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

### Compound Types | `tuple` and `array` | array vs Vec<T> (Stack Performance vs Heap Flexibility):
Both tuples and arrays are fixed in size. Hence, stored in the stack. Once declared, they cannot grow or shrink in size. Vec<T> can grow or shrink, hence stored in the heap.

* Both tuple and array can be declared using `let` and `let mut`, when declared with `mut`, we can change its individual element's value.

- Tuple : written by a comma-separated list of values inside parentheses `(1, 2, 1.2)`. It can store different types.

- Array : written by a comma-separated list of values inside square brackets. Unlike tuple, all elements must have the same type. Explicit type is done by: Elements data type, a semicolon and then the number of elements all within a square bracket as `[type; number_of_elements]`.

* `unit` type is an empty tuple `()`

```rust
fn main() {
    let tup: (i32, f64, u8) = (400, 6.4, 1);
    let (x, y, z) = tup; // pattern matching for destructuring
    println!("The value of y is: {y}");

    // using dot syntax to access tuple
    let four_hundred = tup.0;
    let six_point_four = tup.1;

    // Array
    let a = [1, 2, 3, 4, 5, 6, 7];
    let b: [u32; 4] = [1, 2, 3, 4];

    // declaration of array's type: type, a semicolon and then the number of elements all within a square bracket as `[type; number_of_elements]`
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    println!("This is the month = {}", months[11]);
}
```

* Index out of range | Panic: While in runtime, if out of range (If the index is greater than or equal to the length) array elements are accessed, rust program will panic and exit. 


### Functions `fn` :
Functions can be defined before or after the calling block. Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.

* argument vs parameter: Technically, the concrete values are called arguments (callers's use this, when calling the function), parameter is the variable defined in the functions declaration. Bun in casual conversation, they are same

```rust
fn main() {
    print_labeled_measurement(7, 'G');
}

print_labeled_measurement(value: i32, unit_label: char) {
    println! ("The measurement is : {value}{unit_label});
}
```

* Statement vs Expression: Rust is an expression based language. Where 
    - Statements are instructions that perform some action and do not return a value (no returns, perform tasks, binding or assignment). Function declaration is a Statement (calling that is an expression)
    - Expressions evaluate to a resultant value (evaluates to a value or returns). `6+6` or `6` are both expression as they evaluate to a value. Expression can be part of a statement ( as a whole )


* why `let x = y = 7` doesn't work in rust but works in C and Ruby:
In Rust any assignment or binding doesn't retune anything, but in C or Ruby, assignment or binding returns the value.

* Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression.

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

/**
* This part `a new scope block` is an expression
* Note: Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. You've to explicitly state the `return` statement to make it return.
* A.K.A : ending line with no semicolon will be considered as return 
{
    let x = 3;
    x + 1
}
*/
```

* Rust nothing returning type is `Unit` as `()` empty tuple
* Swift's nothing returning type is `Void` as `()` empty tuple
* Java's nothing returning type is `Void` only, value of Void is null
