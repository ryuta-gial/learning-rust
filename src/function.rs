
#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, stdin, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn say_hello() {
    println!("Hello");
}
fn get_sum(x: i32, y: i32) -> i32 {
    println!("{}+{}={}", x, y, x + y);
    x + y + y
}
fn get_2(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}
fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    sum
}
fn main() {
    //文字列の表示
    say_hello();
    println!("{}", get_sum(4, 5));
    //変数分割代入
    let (val_1, val_2) = get_2(3);
    println!("Nums:{} {}", val_1, val_2);
    //配列のすべての値を合算して返す関数
    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum of list = {}", sum_list(&num_list))
}
