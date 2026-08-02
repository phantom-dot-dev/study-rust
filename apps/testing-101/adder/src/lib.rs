use std::derive;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Rectangle;

    #[test]
    fn it_works() {
        let result = crate::add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another_test() {
        // panic!("Failing Test") // if panic is not called, the test will OK for this``
    }

    #[test]
    fn add_two_success() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_two_fail() {
        let result = add_two(4);
        assert_eq!(result, 7);
    }

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
