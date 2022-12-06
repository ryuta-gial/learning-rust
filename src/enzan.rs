
#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, stdin, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    println!("MAX u32: {}", u32::MAX);
    println!("MAX f32: {}", f32::MAX);
    println!("MAX u64: {}", u64::MAX);
    println!("MAX usize: {}", usize::MAX);

    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4={}", num_3 + num_4);
    println!("5 - 4={}", num_3 - num_4);
    println!("5 / 4={}", num_3 / num_4);
    println!("5 % 4={}", num_3 % num_4);
}
