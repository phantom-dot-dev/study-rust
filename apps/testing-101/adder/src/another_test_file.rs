pub fn adding(a: u64, b: u64) -> u64 {
    a + b
}


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
    fn greeting_check_success() {
        panic!("Checking from another module");
    }

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

    // test function returning Result<T, E>, instead of Panic!
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2); // when we call include! macro, it also makes all functions defined in the parent module available for child module

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
