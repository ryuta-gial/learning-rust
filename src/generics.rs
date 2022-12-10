
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


//std::ops::Add<Output=T> 標準ライブラリ
//足し算を表すトレイト
//足し算ができる型ならOK
fn add<T:std::ops::Add<Output=T>>(a:T,b:T)->T{
    a+b
}

fn main() {
    println!("{}",add(10,25));
    println!("{}",add(10.0,25.0));
    println!("{}",add::<i32>(10,25));//型を明示
    //エラーになる
    //println!("{}",add('a','a'));
}
