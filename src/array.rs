
#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, stdin, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx = 0;
    //ループ
    loop {
        if arr_1[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_1[loop_idx] == 9 {
            break;
        }
        println!("val:{}", arr_1[loop_idx]);
        loop_idx += 1;
    }
    loop_idx = 0;
    while loop_idx < arr_1.len() {
        println!("Arr :{}", arr_1[loop_idx]);
        loop_idx += 1;
    }
    loop_idx = 0;
    //ただループするだけのイテレータ
    for val in arr_1.iter() {
        println!("suuji:{}", val);
    }
}
