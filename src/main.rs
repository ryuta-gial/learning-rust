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

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

//メソッドを定義
//構造体がジェネリクス T を使用しており、実装するメソッド内でもTを扱う必要がある場合は、 impl <T> から宣言を始める必要がある
//impl Point<f32>は特定の型のみを指定する
impl<T> Point<T>
where
    T: std::ops::AddAssign,
{
    //コンストラクター
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    fn add(&mut self, pt: Point<T>) {
        self.x += pt.x;
        self.y += pt.y;
    }
}

fn main() {
    //Point型を生成
    let mut pt = Point::new(10, 10);
    println!("{:?}", pt);
    //値を加算
    pt.add(Point { x: 20, y: 30 });
    println!("{:?}", pt);
}
