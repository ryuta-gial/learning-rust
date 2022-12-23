
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
    //配列に文字列を代入
    let array = [
        "Apple".to_string(),
        "Banana".to_string(),
        "Mango".to_string(),
        "Tomato".to_string(),
    ];
    for a in array.iter() {
        println!("{}",a);
    }
    println!("len = {}",array.len());
}
