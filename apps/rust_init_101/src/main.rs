#![allow(unused)] // to get rid of Unsed warning

use std::io; // std::io:* to bring all packages
use rand::Rng; // packages/libraries are called crates in rust, search rust libraries and version in crates.io
use std::io::{Write, BufReader, BufRead, ErrorKind}; // nested path syntax to multiple import
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("Max u32 {}", u32::MAX); // 4294967295
    println!("Max i32 {}", i32::MAX); // 2147483647
    println!("Max u64 {}", u64::MAX); // 18446744073709551615
    println!("Max usize {}", usize::MAX); // 18446744073709551615
    println!("Min usize {}", usize::MIN); // 0
    println!("Min u32 {}", isize::MIN); // -9223372036854775808
}
