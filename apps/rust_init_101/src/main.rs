#![allow(unused)] // to get rid of Unused warning

use rand::Rng; // packages/libraries are called crates in rust, search rust libraries and version in crates.io
use std::cmp::Ordering;
use std::fs::File;
use std::io; // std::io:* to bring all packages
use std::io::{BufRead, BufReader, ErrorKind, Write}; // nested path syntax to multiple import

fn say_hello() {
    println!("Hello World From Rust")
}

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    x + y // no semicolon at the end and no `return` keyword is specified, it is implicit return
    // return x + y; // or explicitly using return statement and ending with semicolon
}

// returning tuple
fn get_tuple(initial: i32) -> (i32, i32) {
    (initial + 1, initial + 2)
}

fn main() {
    // Function calling
    say_hello();
    get_sum(1, 2);
    println!("{}", get_sum_2(3, 4));

    let destructuring_tuple = get_tuple(1);
    println!("Printing tuple = ({}, {})", destructuring_tuple.0, destructuring_tuple.1);
}
