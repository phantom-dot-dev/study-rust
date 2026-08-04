### Test in rust:
Tests are Rust functions that verify that the non-test code is functioning in the expected manner. 
Test bodies usually performs these 3 actions

1. Setup any needed data or state

2. Run the code you want to test

3. Assert that the results are what you expect

A test in rust is a function annotated with text attribute `#[text]` placed before the function definition line. When `cargo test` is run, rust build the test binary that runs the annotated functions & report about each function's success or failure.


Test module can be functions without the `#[test]` attribute, to help set up common scenarios or perform common operations for all test functions.......

```rust
// commands
// cargo new adder --lib
// cd adder, inside src/lib.rs

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)] // tells the compiler to compile the attached code only when running tests (such as executing cargo test). For `cargo build` and `cargo run`, the Rust compiler completely ignores and excludes any code marked with this attribute
mod tests {
    use super::*; // make all globally defined functions available by only their name
    // without this we've to call function with the crate namespace, ie, `crate::add(x,y)`

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// command to run the test
// cargo test
```

There are different kinds of test features
- Ignoring a test function
- Only running specified test functions by `cargo test <name>`, these are `filtered tests`
- Benchmark tests (only available for nightly release)
- Documentation test to compile and test code example in API documentation
- Controlling test to run differently


### Macros for test suites:
`assert!` macro from std lib, can evaluates to true (Test Ok) and false (Test Failed). When failed, the `assert!` macro calls `panic!` behind the scene

```rust
#[cfg(test)]
mod tests {
    use crate::Rectangle;

    #[test]
    fn larger_can_hold_smaller() {
        let larger_rec = Rectangle { width: 7, height: 7 };
        let smaller_rec = Rectangle { width: 4, height: 3 };

        assert!(larger_rec.can_hold(&smaller_rec));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger_rec = Rectangle { width: 7, height: 7 };
        let smaller_rec = Rectangle { width: 4, height: 3 };

        assert!(!smaller_rec.can_hold(&larger_rec)); // we're negating the output of can_hold function before injecting in assert! macro
    }
}
```


* `assert_eq!` for equity & `assert_ne!` and inequity. They’ll also print the two values if the assertion fails, which makes it easier to see why the test failed. Both can be achieved using `assert!` macro combining with `==` check. But the assert! macro only indicates that it got a false value for the == expression, without printing the values that led to the false value.


```rust

pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_fail() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}

/* 
// output from compiler
.......
---- tests::add_two_fail stdout ----
thread 'tests::add_two_fail' (5994) panicked at src/lib.rs:49:9:
assertion `left == right` failed
  left: 6
  right: 7
*/
```

* When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the PartialEq and Debug traits. All primitive types and most of the standard library types implement these traits. For structs and enums that you define yourself, you’ll need to implement PartialEq to assert equality of those types. Which is usually done by adding the #[derive(PartialEq, Debug)] annotation to your struct or enum definition.

### Custom test failure message:
All these functions (`assert!`, `assert_eq!`, 'assert_nq!`) support custom failure message as optional argument.

```rust
pub fn greeting(s: &str) -> String {
    format!("Hello {s}")
} 

#[cfg(test)]
mod tests {
    use super::*;
    // --------------- Custom error message when test fails
    #[test]
    fn greeting_success() {
        let result = greeting("Dickinson");
        assert!(result.contains("Dickinson"));
    }

     #[test]
    fn greeting_fail() {
        let name = "Dickinson";
        let result = greeting(name);
        assert!(result.contains("Eddie"), "Greeting doesn't contain the word {name}, and the full sentence is \n\"{result}\"");
    }
}
```

### Checking for panic (using should_panic):
The `#[should_panic]` attribute in-between the `#[test]` attribute and function name works opposite. The test passes if the code inside panics, the test fails if the code inside the function doesn't panic

```rust
// testing with should_panic
struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        Guess { value }
    }
}


// #[cfg(test)]
mod another_test {

    use super::*;

    #[test]
    #[should_panic]
    fn greeter_than_100_not_allowed_should_pass() {
        Guess::new(127); // this will cause to call panic, making this as ok because of #[should-panic] attribution
    }

    #[test]
    #[should_panic]
    fn smaller_than_100_allowed_should_fail() {
        Guess::new(7); // this won't call panic, making this as ok because #[should_panic] attribute 
    }
}
```



### Separating test files:
If `src/lib.rs` is the main file that holds the testing module. We can place another_test_file.rs using 
- `include!` macro from inside the main module. All test code in that file can be wrapped in a `mod` or can be separated.

- declare the other file as module from inside of the main file, under `#[cfg(test)]` attribute. And all function attributed with `#[test]` will be called only when `cargo test` is called.


```rust
// use scattered test function by declaring as module from the main file or lib.rs
/* file structure
src/
|-- lib.rs
|-- another
*/

// ./src/lib.rs, Declare the module only when compiling for tests
#[cfg(test)]
mod another_test;

// ./src/another_test.rs, contains all the test function
use crate::*; // bring your library's functions into the scope
#[test]
fn test_something() {
    assert_eq!(4+3, 7);
}

```

```rust
// include! macro to add test code form another file to main library module

// ./src/lib.rs
#[cfg(test)]
mod tests {
    // Verbatim includes the contents of another_test.rs here using include! macro
    include!("another_test.rs");
}

// another_test.rs, contains all testing code inside
// #[cfg(test)]
mod another_test {
    #[test]
    fn greeting_check_success() {
        panic!("Checking from another module");
    }
}
```



### Test can also return `Result<T, E>` (instead of Panic!):
Writing tests so that they return a Result<T, E> enables you to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.

* Note: `#[should_panic]` attribute won't work when function returning `Result<T, E>`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```
