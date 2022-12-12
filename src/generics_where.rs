
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

fn add<T>(a: T, b: T) -> T
where
    //トレイト
    T: std::ops::Add<Output = T>,
{
    a + b
}
//フィボナッチ数列
fn main() {
    println!("{}", add(10, 25));
    println!("{}", add(10.0, 25.0));
}
