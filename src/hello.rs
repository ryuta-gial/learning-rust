#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, stdin, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn hello() {
    let mut a = 10;
    println!("{}", a);
    //入力を格納する変数を作成
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    //標準neoclip入力を受け取って変数に入れる
    io::stdin().read_line(&mut name).expect("input error");
    println!("Hello {}! {}",name.trim_end(), greeting);
}
