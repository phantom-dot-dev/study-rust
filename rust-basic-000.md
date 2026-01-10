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
