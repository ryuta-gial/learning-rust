#![allow(unused)]

// use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, stdin, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let mut sample1 = 5;
    let print_var = || println!("sample1 = {}", sample1);
    print_var();
    sample1 = 10;
    let mut change_var = || sample1 += 1;
    change_var();
    println!("sample1 = {}", sample1);
    sample1 = 10;
    println!("sample1 = {}", sample1);

    //where は　ジェネリクスをわかりやすくするために記述
    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where
        //ジェネリクスに対して定義している
        T: Fn(i32, i32) -> i32,
    {
        func(a, b)
    }
    let sum = |a, b| a + b;
    let prod = |a, b| a * b;
    println!("5+4={}", use_func(5, 4, sum));
    println!("5*4={}", use_func(5, 4, prod));
}
