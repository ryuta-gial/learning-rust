#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, stdin, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    //ベクトル 配列
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    println!("1st:{}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        //値がマッチする場合
        Some(second) => println!("2nd:{}", second),
        //ない場合
        None => println!("Mo 2nd value"),
    }
    //配列すべてに2を掛ける
    for i in &mut vec2 {
        *i *= 2;
    }
    //それを表示する
    //&は参照する
    for i in &vec2 {
        println!("{}", i);
    }
    // 配列の長さ
    println!("Vec Length {}", vec2.len());
    //配列の最後の値を削除
    //{:?}vecをprintlnで表示したい時に使う
    println!("Pop: {:?}", vec2.pop());
}
