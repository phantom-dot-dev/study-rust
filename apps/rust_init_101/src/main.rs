#![allow(unused)] // to get rid of Unused warning

use std::io; // std::io:* to bring all packages
use rand::Rng; // packages/libraries are called crates in rust, search rust libraries and version in crates.io
use std::io::{Write, BufReader, BufRead, ErrorKind}; // nested path syntax to multiple import
use std::fs::File;
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
