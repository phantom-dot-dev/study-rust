### Importing (`using`) standered and external lilbrary:
Libraries are called `crates` in rust. `https://crates.io/` is the official place for external libraries.

* externel libaries/crates needs to be included inside of the `Cargo.toml` file under `[Dependencies]` section, to be used/imported

```rust
#![allow(unused)] // to get rid of Unsed warning

use std::io; // std::io:* to bring all packages
use rand::Rng; // packages/libraries are called crates in rust, search rust libraries and version in crates.io
use std::io::{Write, BufReader, BufRead, ErrorKind}; // nested path syntax to multiple import
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("");
}
```

### `::` vs `.`:
Or: . is for value member access, :: is for namespace member access

```rust
// usages of `.`
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}


// Usages of `::`
impl Rectangle {
    // Associated Function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3);
}
```

### Basic and User input:
```rust
#![allow(unused)] // to get rid of Unsed warning

use std::io; // std::io:* to bring all packages

fn main() {
    // by default variables are immutable, specify `mut` to make it mutable
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";

    io::stdin().read_line(&mut name)
    .expect("Didn't Receive Input");
    
    println!("Hello {}! {}", name.trim_end(), greeting)
}
```

## Variable and Constant:
```rust
use std::io; // std::io:* to bring all packages

fn main() {
    // Defining Constant
    const ONE_MIL: u32 = 1_000_00;
    const PI: f32 = 3.1416;

    // Variables
    let age: &str = "47"; // immutable variable

    // mutable variable
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't a number"); // Error handling are built into the language
    age = age + 1;
    println!("I'm {} and I want ${}, LOL", age, ONE_MIL);
}
```


### Data types 101:
`unsigned intergers`: u8, u16, u32, u64, u128, usize
`signed intergers`: i8, i16, i32, i64, i128, isize

* usize and isize integers are dependent of the device architecture. If the device is 32 bit, usize/isize will be 32, for 64 bit, it will be 64

* A signed integer can represent both positive and negative values, while an unsigned integer can only represent non-negative (zero and positive) values.

```rust
fn main() {
    println!("Max u32 {}", u32::MAX); // 4294967295
    println!("Max i32 {}", i32::MAX); // 2147483647
    println!("Max u64 {}", u64::MAX); // 18446744073709551615
    println!("Max usize {}", usize::MAX); // 18446744073709551615
    println!("Min usize {}", usize::MIN); // 0
    println!("Min u32 {}", isize::MIN); // -9223372036854775808
}
```

### Boolean, Character and Floating Point Numbers:
Variables with same name but different data types are supported, they are called shadowing.
```rust
fn main() {
    let is_true = true;
    let a_char: char = 'G'; // define 


    // rust has f32 and f64(default), both are signed
    // precession is non-deterministic. This means it varies by platform, Rust version, etc
    let num_1: f32 = 1.111111111111111;
    println!("f32 : {}", num_1 + 0.111111111111111); // 1.2222223
    println!("f32 min: {}", f32::MIN);

    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.111111111111111); // 1.2222222222222219
    println!("f64 min: {}", f64::MIN);
}
```

### Math operators and Random number gen:
```rust
use rand::Rng; // packages/libraries are called crates in rust, search rust libraries and version in crates.io

fn main() {
    // Math and operators
    let num_5: u32 = 5;
    let num_4: u32 = 4;
    let mut num_3: u32 = 3;
    
    println!("5 + 4 = {}", num_5 + num_4);
    println!("5 % 4 = {}", num_5 % num_4);
    num_3 += 1;
    println!("num_3 + 1 = {}", num_3); 

    // Random number gen
    let random_number: i32 = rand::thread_rng().gen_range(1..101);
    println!("Random number between 0 and 100 = {}", random_number);
}
```

### Conditionals | `if/else if/else`, `ternary` and `Match`:
`match` should cover all possible cases, it's like a switch in other programming language. Inside match, each cases are separated by `,` comma. And `;` are places after the match's ending braces.

```rust 
fn main() {
    // conditional | if
    let age: i32 = 8;
    if (age >= 1) && (age <= 18) {
        println!("Important Birthday");
    } else if (age == 21) || (age == 50) {
        println!("Other's birthdays are important");
    } else if age >= 65 {
        println!("Birthday is important again after 65");
    } else {
        println!("Birthdays are not that important");
    }

    // ternary operator
    let can_vote: bool = if age >= 18 {
        true
    } else {
        false
    };
    println!("Can vote: {}", can_vote);

    // Match conditional
    match age {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Others birthdays are Important"),
        65..=i32::MAX => println!("Important Birthday Again"),
        _ => println!("Birthday celebration is not important"),
    };
}
```

### Match with Comparison:
When match statement is used with the `std::cmp::Ordering`, all possible cases are covered by comparing cases.

```rust
use std::cmp::Ordering;

fn main() {
    // Match with Comparison
    let my_age: i32 = 18;
    let voting_age: i32 = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!(""),
        Ordering::Greater => println!(""),
        Ordering::Equal => println!(""),
    };
}
```

### Array and Loop:
- Elements in an array are must be same type
- Arrays are in fixed size

```rust
fn main() {
    // Arrays
    let arr_1: [i32; 9] = [1,2,3,4,5,6,7,8,9];
    println!("1st : {}", arr_1[0]);
    println!("Length: {}", arr_1.len());

    // loop
    let mut loop_idx: usize = 0; // index must be of usize

    loop {
        if arr_1[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }

        if arr_1[loop_idx] == 9 {
            break;
        }

        println!("Val : {}", arr_1[loop_idx]);
        loop_idx +=1;
    }

    // while loop
    loop_idx = 0; // resetting loop index
    while loop_idx < arr_1.len() {
        println!("Arr : {}", arr_1[loop_idx]);
        loop_idx +=1;
    }

    // for loop
    for val in arr_1.iter() {
        println!("Val : {}", val);
    }

}
```

### Tuple:
```rust
fn main() {
    // Tuple
    let my_tuple: (u8, String, f64) = (47, "Something".to_string(), 1234.00);
    println!("my_tuple values are {} {} {}", my_tuple.0, my_tuple.1, my_tuple.2);

    // tuple destructuring
    let (a, b, c) = my_tuple;
}
```

### String:
2 types (mostly): String and &str.

A `String` is stored as a vector of bytes (Vec<u8>), but guaranteed to always be a valid UTF-8 sequence. String is heap allocated, growable and not null terminated.

`&str` (String pointer) is a slice (&[u8]) that always points to a valid UTF-8 sequence, and can be used to view into a String, just like &[T] is a view into Vec<T>.

```rust
fn main() {
    // String
    let mut st1: String = String::new();
    st1.push('A'); // pushing a character
    st1.push_str(" Word");
    for w in st1.split_whitespace() {
        println!("{}", w);
    }

    // replacing
    let st2: String = st1.replace(&st1, "Hello");
    println!("{st2}");
    let st3: String = st1.replace("A", "Hello");
    println!("{st3}");

    // more string functions
    let st4: String = String::from("z y x w v u o o d");
    let mut v1: Vec<char> = st4.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{char}");
    } 

    let st5: &str = "Random string";
    let mut st6: String = st5.to_string();
    println!("{}", st6);

    // string to byte
    let byte_arr1 = st6.as_bytes();
    let st7: &str = &st6[0..6]; // will panic if the range is out of bound
    println!("st7 string length = {} and the string is {st7}", st7.len());
    

    // clearing is only possible with mutable string
    st6.clear();
    println!("After clearing st6's length is st6.len() = {}", st6.len()); // 0

    // string concatenation
    let st8: String = String::from("Hello");
    let st9: String = String::from(" world");
    let st10: String = st8 + &st9;
    // converting byte to char again to demo
    for char in st10.bytes() {
        println!("char {} and byte {}", char::from(char) , char);
    }
    
}
```

### Type casting:

```rust
fn main() {
    // type casting
    let int1_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int1_u8 as u32) + (int2_u8 as u32);
}
```

### Enum:

```rust
fn main() {
    // Emum
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    // defining functions for enum
    impl Day {
        fn is_weekend(&self) -> bool {
            return match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            };
        }
    }

    // usages
    let today: Day = Day::Monday;
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!(""),
        Day::Wednesday => println!(""),
        Day::Thursday => println!(""),
        Day::Friday => println!(""),
        Day::Saturday => println!(""),
        Day::Sunday => println!(""),
    }

    println!("Is today a weekend = {}", today.is_weekend());
}
```


### Vectors:
Vector: Vectors are like array. Mutable vectors can grow. It store value of same type

```rust
fn main() {
    // Vectors
    let vec1: Vec<i32> = Vec::new();
    let mut vec2:Vec<i32> = vec![1,2,3,4];
    vec2.push(5); 
    println!("1st : {}", vec2[0]);

    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("(Matching)  2nd : {}", second),
        None => println!("Not a match!"),
    };

    // note : if Some and None arms are provide inside of the match block, the match will expect `Option<T>`
    match vec2[1] {
        // Some(second) => println!("(Matching)  2nd : {}", second),
        // None => println!("Not a match!"),
        _ => println!("Just a default output") // default arm must be provide for non exhaustive matching
    };

    for i in &mut vec2 {
        // `&mut vec2` will make the vec2 borrow-able, so we can modify that inside of the for loop
        *i *= 2; // multiplying the element (pointer) with 2 
    }

    // lets check if the vec2 is modified

    // checking with index, using vec2.inter().enumerate()
    for (index, element) in vec2.iter().enumerate() {
        println!("vec2 index : {} and vec2 element : {}", index, element);
    }

    // popping a value and showing that as output
    println!("The last popped element value is {:?}", vec2.pop());
}
```

### `Option<T>` enum | containing `Some` and `None`:
None and Some are the variants of the enum, that is, a value with type Option<T> can either be a None, or it can be a Some containing a value of type T. 

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

### Functions:

```rust
fn say_hello() {
    println!("Hello World From Rust");
}

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    x + y // no semicolon at the end and no `return` keyword is specified, it is implicit return
    // return x + y; // or explicitly using return statement and ending with semicolon .
}

// returning tuple
fn get_tuple(initial: i32) -> (i32, i32) {
    (initial + 1, initial + 2)
}

// Vector as function's param
fn sum_list(list: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for &val in list.iter() {
        sum += &val;
    }
    sum // implicit return, when without the semicolon
}
fn main() {
    // Function calling
    say_hello();
    get_sum(1, 2);
    println!("{}", get_sum_2(3, 4));

    let destructuring_tuple = get_tuple(1);
    println!(
        "Printing tuple = ({}, {})",
        destructuring_tuple.0, destructuring_tuple.1
    );

    // Calling function with the vector parameter
    let a_vector: Vec<i32> = vec![1,2,3,4];
    let vec_fn_res: i32 = sum_list(&a_vector);
    println!("The combined result of the a_vector is {}", vec_fn_res);
}
```

### Generics :
```rust
use std::ops::Add; // operators Add to implement Generics computation

fn get_sum_generics<T: Add<Output = T>> (x: T, y: T) -> T {
    // to do any further computation in the Generics Type, we need to set bounds using Traits (Interfaces)
    return x + y;
}
fn main() {
    // Generics
    
    println!("5 + 4 = {}", get_sum_generics(4, 7));
    println!("5 + 4 = {}", get_sum_generics(4.12, 7.12));
}
```
