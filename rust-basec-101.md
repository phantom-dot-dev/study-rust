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
    let age: &str = "47"; // immuable variable

    // mutable variable
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't a number");
    age = age + 1;
    println!("I'm {} and I want ${}, LOL", age, ONE_MIL);
}
```


### Data types:
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
