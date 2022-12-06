
#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, stdin, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let random_num = rand::thread_rng().gen_range(1..101);

    println!("Random: {}", random_num);
    let age: i32 = 8;
    if (age >= 1) && (age <= 18) {
        println!("Birthday");
    } else if (age == 21) || (age == 50) {
        println!("Birthday");
    } else {
        println!("Not Birthday");
    }
}
