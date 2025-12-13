#![allow(unused)] // to get rid of Unused warning

use std::io; // std::io:* to bring all packages
use rand::Rng; // packages/libraries are called crates in rust, search rust libraries and version in crates.io
use std::io::{Write, BufReader, BufRead, ErrorKind}; // nested path syntax to multiple import
use std::fs::File;
use std::cmp::Ordering;

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
    
}
