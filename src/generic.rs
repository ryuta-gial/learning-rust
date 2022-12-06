
#![allow(unused)]

// use rand::Rng;
// use std::cmp::Ordering;
// use std::fs::File;
// use std::io::{self, stdin, Read};
// use std::io::{BufRead, BufReader, ErrorKind, Write};

//加算演算子の機能を指定する
use std::ops::Add;

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}
fn main() {
    println!("5+4 = {}", get_sum_gen(5, 4));
    println!("5.2+4.7 = {}", get_sum_gen(5.2, 4.7));
}
