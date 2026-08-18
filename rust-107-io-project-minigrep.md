### Project initialization:
`cargo init` creates project in the current directory with no name supplied
`cargo init --name project_name` creates in current directory with the supplied name as project name
`cargo new <project_name>` creates project in a new directory as the project_name

* cargo init vs new: The primary difference between cargo init and cargo new is where the project files are created.


### Safely access vector element:
When you access a vector index that doesn't exist, the program will panic if not dealt.

- `v[index]` need to be absolute certain about the existence
- `v.get(index)` will return `Option<&T>`, so need to handle Some and None cases through match or use `unwrap_or(&0)`

- using `v.get_mut(index)` to mutably borrowing and changing an element

- using `v.get(index).unwrap_or(&default_value)` getting a default value if the index is missing

```rust
fn main() {
    let mut v = vec![10, 20, 30];

    // Immutably borrowing an element
    match v.get(1) {
        Some(value) => println!("Found: {value}"),
        None => println!("Index out of bounds"),
    }

    // Mutably borrowing and changing an element
    if let Some(value) = v.get_mut(1) {
        *value = 25; 
    }

    // Getting a default value if the index is missing
    let value = v.get(5).unwrap_or(&0);
    println!("Value or default: {value}");
}

```


### Single Reference vs Double Reference (&String vs &&String):
A &String is a borrowed reference to that text. A &&String is a reference to a reference. This double reference are usually seen (`&&String`) when a method or iterator yields references automatically, but you rarely need to write it yourself.


- &String (Single Reference)
    - Borrows the String without taking ownership.
    - Read-only by default.
    - Lets multiple parts of your code look at the text without copying it.

- &&String (Double Reference)
    - A reference that points to a &String box.
    - Happens during loops or iterator methods like .iter() over a collection of references.
    - Rust usually uses auto-dereferencing to let you call normal methods on it without trouble.


### Struct::new vs Struct::build:
Many programmers expect the `new` function should never fail. But the `build` function can fail, so all error handling can reside there.


### Ok(()) type, Box<dyn Error> and `?` and `if let`:
When returning a `Ok(())`, the `()` means we're returning nothing but doing something as side effect. The `()` is a empty tuple signature.

The `Box<dyn Error>` is a trait object, a function returning this means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value will be. This gives us flexibility to return error values that may be of different types in different error cases. The dyn keyword is short for dynamic.

And the `?` operator is used with functions that can panic!. Rather than panic! on an error, ? will return the error value from the current function for the caller to handle.


The `if let Variable(error)` pattern works kinda same as unwrap_or_else to check is a function returns an `Error` value ignoring any `Ok` value, when the `Ok(())` is returning nothing but doing side effect.

```rust
use std::error::Error;

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
```

### `unimplemented!()` vs `todo!()` for unfinished code and some other options:
Both of these macros are same (will panic) but differ slightly for intentional purpose and both satisfy the compiler's type checking while prototyping.

- `unimplemented!("Optional Message")` is for Permanent/deliberate omission and ignored by IDE todo tracker. Emit message 'not implemented'. Support optional message.

- `todo!("Optional Message")` is for temporary/work-in-progress placeholder and are highlighted by IDE todo tracker, Emit message 'not yet implemented'. Support optional message.

There are some other helpful macros as well to deal with IDE and coding progression

- `unreachable!` : Panics instantly with "internal error: entered unreachable code". Used for default match arms where you have already handled all valid enums

```rust
match light {
    TrafficLight::Red => stop(),
    TrafficLight::Green => go(),
    _ => unreachable!("We only have Red and Green lights!"),
}
```

- `#[warn(clippy:todo)]` compiler attribute to flag incomplete work. This avoid runtime panics by forcing compile-time warnings or error through `cargo check` or `cargo build`. Best for keeping track of text-based reminders without breaking your program's execution.


```rust
#[warn(clippy::todo)]
fn finish_this_later() {
    // Clippy will trigger a compiler warning right here
}
```

- `compile_error!` to intentionally stop compilation before a binary is even created. Used mostly in conditional compilation (#[cfg]) to prevent unsupported platforms or feature flags from compiling.


```rust
#[cfg(not(target_os = "linux"))]
compile_error!("This crate only supports Linux operating systems.");
```

- `Option` and `Result` types: For production-safe prototyping, these can be used to avoid panicking macros entirely. Return an empty variant that caller functions can gracefully handle. These bubbles up a standard, safe error or non-value. Best used for public APIs where crashing the entire program is unacceptable.

```rust
fn get_user_v2() -> Option<User> {
    None // Placeholder until the database is wired up
}
```

### Splitting Code into Library Crate (alongside Binary Crate):
The `./src/main.rs` should do less work  and  should rely on library crate (`.src/lib.rs`) for business logics. That way, we can test the code (for both unit and integration test) and have the `.src/main.rs` file with fewer responsibilities. 

Defining all business logics inside lib crate will open more context for usages and possibilities for other people use the code.

```rust
// ./src/main.rs
use minigrep::search;
 fn main() {
    let content = fs::read_to_string(&file_name)?;

    for line in search(&config.query, &content) {
        println!("{line}");
    }
 }

// ./src/lib.rs
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    unimplemented!();
}
```

### TDD in rust (Test driven development):
Its the same workflow (consisting a loop of 4 steeps) as for other programming language.

1. Write a test that fails and run it to make sure it fails for the reason you expect.
2. Write or modify just enough code to make the new test pass.
3. Refactor the code you just added or changed and make sure the tests continue to pass.
4. Repeat from step 1!

```rust
use std::result;

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // unimplemented!();
    // first we'll return an empty vector to make the function to fail
    // vec![]
    // then we're gonna implement the function to pass the test. Do just as much required, no pre mature optimization
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
```


### Environment variables:
In rust environment variable can be passed through compiler arguments or through `Cargo.toml` or directly inside code. They can be key-value pair or just a key (to check if a key exists or not).

The standard library provides `env::var` to look up a variable is something exists while the programme running. This function returns a Result enum (Ok(String) or Err(VarError)) because the requested variable might be missing or contain non-Unicode characters.

* reading for a environment variable can be done on both runtime or compile time

```rust
// sometime, we only need to be sure if some key was injected (maybe with `Key cargo run` command), not caring about the value (`KEY=VALUE cargo run)

fn main() {
    // maybe called by `IGNORE_CASE cargo run`
    let ignore_case_result: Result<String, VarError> = env::var("IGNORE_CASE").is_ok();
    let ignore_case: bool = ignore_case_result.is_ok();
    /*
    The env::var function returns a Result that will be the successful Ok variant that contains the value of the environment variable if the environment variable is set to any value. It will return the Err variant if the environment variable is not set.

    we're using is_ok rather than `unwrap` or `except`, as `is_ok()` will only return true or false if `IGNORE_CASE` was injected or not (to check a environment variable existence)
    */
}
```

* Setting environment variable for shell session and removing: `IGNORE_CASE=1 cargo run` will persist the environment for the shell session and `Remove-Item IGNORE_CASE` will remove that variable from the current shell session. For window this will be `$Env:IGNORE_CASE=1; cargo run` and `Remove-Item Env:IGNORE_CASE`

```rust
// using match and expect to get environment variable and handle the error case

use std::env;

fn main() {
    // Looks up the "DATABASE_URL" environment variable
    match env::var("DATABASE_URL") {
        Ok(url) => println!("Connecting to: {}", url),
        Err(e) => println!("Error or variable not set: {}", e),
    }

    // Shorthand if you just want to crash/panic if it is missing
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
}

```

* Using `Cargo.toml` to host environment variables

```toml
[env]
# 1. Simple text variable
DATABASE_URL = "postgres://localhost/mydb"

# 2. Force override if the variable already exists on the system
API_KEY = { value = "secret_key", force = true }

# 3. Path relative to this config file (turns into an absolute path automatically)
LOG_FILE_PATH = { value = "logs/output.log", relative = true }
```

### `stdout` vs `stderr`:
`println!` macro is used for standard output (general information). For showing error message `stderr` rust provide `eprintln!` macro.

The stderr is helpful, when we're saving the program output to a file but still want to print error message in the terminal (if any error occurs). 


```rust
fn main() {
    // -- snip --
    if (program_runs_well) {
        // do the task
    } else {
        // program fails to run
        eprintln!("Application error");
    }
}

// cargo run -- to poem.txt > output.txt
// if some runtime error happens, the terminal will still print the error (because of eprintln!), without this the errors will go directly to the output.txt file (println!)
```


### Functional programming in rust and Closures, Iterators:
Programming in a functional style often includes using functions as values by passing them in arguments, returning them from other functions, assigning them to variables for later execution, and so forth.

`Closures`: anonymous functions, can be saved in a variable or pass as other functions arguments.

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 } // function definition
let add_one_v2 = |x: u32| -> u32 { x + 1 }; // closure definition with optional type and return type
let add_one_v3 = |x|             { x + 1 }; // ,, without param & return type
let add_one_v4 = |x|               x + 1  ; // ,, without braces, and without param and return types


// compiler automatic type inference strategy for closure function
// - like any other variable type inference, closure's param and return types are inferred by compiler by first use/call

let example_closure = |x| x; // defining closure without any type information

let s = example_closure(String::from("hello")); // first-time calling with String type for both parameter and return will instruct compiler to set the type as `String`
let n = example_closure(5); // won't work, as the closure type is already set as `String`, the compiler will not accept `i32` as any of the closure type
```


### Closure's environment capturing (accessing variable from outer scope) vs Regular Fn:
A regular function is rust cannot access variable defined in outer scope, only parameter and local variable created inside of that function are accessible.

Ie, an inner function nested inside another function cannot read the outer function's local variable. 

But a closure can access out-of-scope defined variable.

```rust
fn main() {
    let outer_var = 47;

    // This works! The closure can access outer scope variable (capturing dynamic environment)
    let my_closure = || {
        println!("{}", outer_var); // closure will borrow the outer scope variable automatically 
    };

    my_closure();

    // This will cause a compilation error! while using regular function
    fn my_function() {
        println!("{}", outer_var); // Error: can't capture dynamic environment
    }
}
```

### Closure's capturing reference or moving ownership:
A closure can capture values from their environment (out-of-scope variable access) in three ways, like the 3 ways a function can take a parameter
- borrowing immutably: will borrow any out-of-scope variable automatically if not mutating the variable.
- borrowing mutably: if the closure is mutating the variable, the variable will follow rules for mutating. The old variable cannot be read until the closure had been called.
- moving/taking ownership: Closure doesn't move ownership unless specified the `move` keyword before the double pipe `||`, as `move || println!("moving ownership of the out-of-scope-variable")`

```rust
// immutable borrowing example ---------------------------------
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

// mutable borrowing example -------------------------------------
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);
    // println!("{list:?}"); // won't work here until the closure is called (and will be released afterwards), as mutable borrowing rule apply
    borrows_mutably();
    println!("After calling closure: {list:?}");
}

// moving ownership example -------------------------------------
// use `move` keyword specifically to move the out-of-scope variable ownership
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
```


### Closure types by captured value management and trait bounds (Closure traits):
A closure body can do any of the following
- Move a captured value out of the closure | bounds to `FnOnce` trait
- Mutate the captured value (but not moving ownership) | bounds to `FnMut` trait
- Neither move or mutate the value | bounds to `Fn` trait
- Capture nothing from the environment (variable defined outside) to begin with | bound to `Fn` trait

A closure's capturing & handling of environment values depends on the three kinds of underlying trait implementation.


- `FnOnce` applies to closures that can be called once. All closures implement at least this trait because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits because it can only be called once.

- `FnMut` applies to closures that don’t move captured values out of their body but might mutate the captured values. These closures can be called more than once.

- `Fn` applies to closures that don’t move captured values out of their body and don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.


```rust
// signature of `unwrap_or_else` from the std library
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T 
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None = f(),
        }
    }
}

// unwrap_or_else function returns a generic type T, either `Some(x) => x` or `None => f()` defined in match statement inside of the body
// As the trait bound in `where` clause `F: FnOnce() -> T`, `FnOnce()` trait impose that the generic type `F` must not be called more than once, inside of the unwrap_or_else function
// Because, unwrap_or_else implements the base `FnOnce()` trait, all closure types (+ FnMut, Fn) are supported here 
```


### The closure trait Hierarchy:
In Rust, closure traits are arranged in a strict hierarchy where each trait builds on top of the other. 

The base trait is `FnOnce()`
- `FnMut` extends the `FnOnce()`
- `Fn` extends the `FnMute`

For this reason when the bound is set to `FnOnce()`, it will work with all `FnOnce`, `FnMut` and `Fn` trait. But if the bound is set to `Fn` trait only, it will only accept `Fn` implemented closures.


```rust
pub trait FnOnce<Args> {
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}

pub trait FnMut<Args>: FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait Fn<Args>: FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

/*
* extern "rust-call" is a special internal calling convention (ABI) in Rust used by the compiler to implement the core function-calling traits like Fn, FnMut, and FnOnce. It tells the compiler to treat the arguments of a tuple as individual, flattened arguments at the machine code level rather than as a single packaged tuple structure

* See the ABI section (included here) for mini ABI guides
*/
```

### The `()` syntax as both unit type (empty tuple) and no-arg closure type:
In Rust, `()` plays two completely different roles depending on where it's been used.
it is fundamentally an empty tuple syntax, but it is also used as a type or value to represent zero arguments in function signatures. 

When `()` used inside the angle brackets `<()>` of a closure trait, the special syntactic sugar is just `()`, which accept not arguments. To support multiple arguments, we can use `FnOnce(i32, i32)` kind of syntax.

```rust
// closure type implementing `FnOnce(i32, i32)`, multiple arguments
fn consume_and_add<F>(closure: F) 
where
    F: FnOnce(i32, i32) -> i32 // Trait bound expecting two i32 arguments
{
    // Call the closure exactly once with two arguments
    let result = closure(10, 20); 
    println!("The result is: {}", result);
}

fn main() {
    // This heavy object will be consumed by the closure
    let unique_resource = String::from("Config data");

    // The closure takes two arguments: x and y
    let my_closure = move |x: i32, y: i32| -> i32 {
        // Accessing unique_resource moves it into the closure,
        // and because it drops here, this closure can only run once.
        println!("Using resource: {}", unique_resource); 
        
        x + y
    };

    // Pass the closure to the function
    consume_and_add(my_closure);
    
    // my_closure cannot be used again here because it was consumed
}

// The simplified internal definition in the standard library
pub trait FnOnce<Args> {
    type Output;
    fn call_once(self, args: Args) -> Self::Output;
}

```

### `FnMut` for calling multiple times by reference (sort_by_key fn case):
The function  `sort_by_key` is defined to take an FnMut closure, it can call the closure multiple times, once for each item using a loop. The closure |r| r.width doesn’t capture, mutate, or move anything out from its environment, so it meets the trait bound requirements of `FnMut`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|r| {
        // sort_operations.push(value); // compile error, as the ownership of `value` changed, and we're calling that multiple time, after the first iteration, the variable `value` will be non-existence. All because the `FnMut` trait implemented by the `sort_by_key` 's accepted closure does not support moving ownership as this can be called multiple times (opposite of FnOnce)
        r.width
    });
    println!("{list:#?}");


    // But, the code below is valid, as it doesn't move the ownership for the `num_sort_operations` variable, it will use the reference
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}
```

### ABI and Interaction with the OS:
Rust supports a wide variety of calling conventions (ABIs) to interact with the underlying operating system, compile assembly, and link with foreign programming languages like C, C++, and WebAssembly.


ABI: Stands for Application binary interface, For communicating with another program and/or library file or the operating system (installing app, networking, etc), ABI dictates exactly how data structures are laid out in memory, how functions pass arguments to CPU registers, and how the program makes system calls to the operating system kernel.


* Usages of ABI
    - ABIs are not used for everything. Most of the time a program communicates with itself, the compiler laid out most of the instruction set, so all of its variables, loops and internal functions can be calculated using the compiler provided blueprint.
    - Storing local variables, running runtime loops, and calling internal functions do not use an ABI. When code runs entirely inside itself, the compiler has absolute freedom to arrange memory, manage loops, and handle variables however it wants.
    - ABIs are only used when the program need to cross its internal boundary, like communicating with the OS (not cpu), write/read a file, print text to screen, spawning a new thread, etc. 
    - ABIs are used to talk to shared `.dll` (windows) or `.so` (linux) files compiled by other program
    - ABIs are used when code written in C++/rust/swift needs to call a function written in other programming languages. Usually they must agree on a shared `C ABI` to understand each other's binary data layout
    - ABIs are used as a program cannot talk directly to computer's screen, hard drive, or Wi-Fi card. It must ask the Operating System (OS) kernel to do it via a System Call.
    - ABIs are used When Operating System Linkers and Loaders Launch Your Program (executables). The OS uses the Executable Format ABI (like ELF on Linux, PE on Windows, or Mach-O on macOS) to understand how the binary data is structured on disk, where to map it into RAM, and where the CPU should look to find the entry point (the main function).


    * ABI vs API Use cases
        - In standard software engineering, two separate program will communicate through exposed APIs contract. Only relying on ABIs are incredibly fragile.
        - If program A communicate with program B purely by guessing its memory offsets and CPU register usage (ABI-only), the slightest change will break it. If Program B is recompiled using a newer version of the compiler, the compiler might decide to optimize the code and move a variable from Register RDI to Register RSI.
        - ABI's are used When a security researcher or reverse engineer modifies a compiled binary program (like a video game or a closed-source application), they do not have access to the source code or the developer's API. They use a debugger to analyze the compiled binary file. They find the exact memory address where a function starts and look at how the CPU handles it. They write a separate binary injector. This injector directly targets the ABI rules of the target program. It forces the CPU to place a value into Register RAX and hijacks the execution pointer.


The list of supported ABIs is divided into stable options you can use today, platform-specific options for hardware architectures, and internal unstable options.

- Stable ABIs (Cross-Platform)
    These are the most common ABIs used for standard foreign function interfaces (FFI) and cross-language communication.

    - "Rust": The default ABI used for standard Rust functions. It is unstable, meaning its exact layout can change between compiler versions.
    
    - "C" (or "extern" without a string): Matches the standard C ABI of the target platform. It is the default choice for interoperability with almost all other languages.
    
    - "system": A helper that automatically resolves to the standard system ABI. On most platforms, this is identical to "C", but on 32-bit Windows, it maps to "stdcall"


- Platform-Specific ABIsThese ABIs
    Target specific operating systems or processor architectures, often used in embedded development, low-level OS kernels, or legacy systems.
    - "stdcall": Used primarily for the Win32 API on 32-bit Windows.
    - "fastcall": Passes as many arguments as possible in CPU registers rather than the stack (common in 32-bit x86 Windows/Linux).
    - "thiscall": Used for invoking C++ non-static member functions on 32-bit Windows.
    - "vectorcall": Passes vector registers for graphics and SIMD math (32-bit and 64-bit Windows).
    - "aapcs" / "aapcs-vfp": Standard calling conventions for ARM architecture devices.
    - "sysv64": The standard ABI for 64-bit non-Windows operating systems (Linux, macOS, BSD).
    - "win64": The standard ABI for 64-bit Windows applications.

- Unstable and Internal ABIs
    These require explicit feature flags and a nightly Rust compiler, as they are meant for core language development or specific compiler optimizations.
    
    - "rust-call": Used internally to implement the Fn, FnMut, and FnOnce closure traits by flattening tuple arguments.
    
    - "rust-intrinsic": Used by the compiler to expose direct CPU instructions or low-level compiler intrinsics (like transmute or bitreverse).
    
    - "ptx-kernel": Used to write GPU kernels for NVIDIA CUDA devices.
    
    - "efiapi": Used for building Extensible Firmware Interface (UEFI) applications and drivers."wasm": Used for specific execution environments in WebAssembly.
