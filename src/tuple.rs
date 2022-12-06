#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, stdin, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let my_tuple: (u8, String, f64) = (47, "Duk".to_string(), 50_000.00);
    let (v1, v2, v3) = my_tuple;
    println!("Name:{}", v2);
    let my2_tuple: (String, u32, f64) = ("sakuragi", 12, 50_000);
    println!("{}", my2_tuple);
}
