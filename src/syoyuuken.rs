
#![allow(unused)]

// use rand::Rng;
// use std::cmp::Ordering;
// use std::fs::File;
// use std::io::{self, stdin, Read};
// use std::io::{BufRead, BufReader, ErrorKind, Write};

fn print_str(x: String) {
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}
fn change_str(name: &mut String) {
    name.push_str("is happy");
    println!("Message : {}", name);
}
fn main() {
    //str1を複製しているのでエラーにならない
    let str1 = String::from("World");
    //str1を明示的に複製
    let str2 = str1.clone();
    println!("Hello {}", str1);

    //str1はstr2で使用されているのでエラー
    //let str2 = str1;
    //println!("Hello {}", str1);

    //str1を変更するケース
    // mutとして宣言し
    //let mut str1 = String::from("World");
    //str1を変更する
    //change_str(&mut str1);


    //関数を通してstr1を別の変数に代入
    let str3 = print_return_str(str1);
    println!("str3={}", str3);
}
