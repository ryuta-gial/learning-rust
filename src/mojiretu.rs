
#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, stdin, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let mut st1 = String::new();
    //「'」で囲むことでchar型として宣言
    //文字列を追加
    st1.push('A');
    st1.push_str("word");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }
    let st2 = st1.replace("A", "another");
    println!("{}", st2);
    //文字列をランダムに表示
    let st3 = String::from("x r f e w q d c a");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }
    let st4 = String::from("Just");
    let st5 = String::from("words");
    //st5に&を付ける意味は st4は代入した時点で消滅したが、st5はまだ存在しているため
    let st6 = st4 + &st5;
    //文字のユニコードを表示する
    for char in st6.bytes() {
        println!("{}", char);
    }
}
