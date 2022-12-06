
#![allow(unused)]

// use rand::Rng;
// use std::cmp::Ordering;
// use std::fs::File;
// use std::io::{self, stdin, Read};
// use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }
    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("Bob Smith"),
        balance: 234.50,
    };
    bob.address = String::from("505 Main St");
    println!("aa {}",bob.address);
}
