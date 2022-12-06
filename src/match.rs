#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, stdin, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let age = 8;
    let voting_age = 18;
    match age {
        1..=18 => println!("Birthday"),
        19..=21 => println!("Birthday"),
        65..=i32::MAX => println!("Birthday"),
        _ => println!("Not"),
    };
    //&は参照という意味
    match age.cmp(&voting_age) {
        Ordering::Less => println!("Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("to vote"),
    }
}
