
#![allow(unused)]
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, stdin, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};
//ライブラリ
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    //strは変更できない文字列、Stringは変更できる文字列
    //strはプリミティブ型の一種です。一般的な型
    let s1: &str = "this is str";
    let s2: String = String::from("this is String");

    println!("{}", s1);
    println!("{}", s2);

    let mut s3: String = String::from("hello");
    let mut s4: String = String::from(" world");

    //文字列合成

    let mut  s5 = format!("{}{}", s3, s4);

    println!("{}", s5);
}
