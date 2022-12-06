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

//構造体(タブル)
struct Item(String, i64);

fn main() {
    let g0 = String::from("所有者");
    let g01 = g0;
    //    println!("{}",g0); これはエラー 所有権
    // let g01 = g0.clone();にすれば移動しない
    println!("{}", g01);
    //基本型は所有権システムの例外となる
    let g1 = 30;
    let g2 = g1;
    println!("{}", g1);
    println!("{}", g2);
    let num: i32 = 10;
    //if
    if (num >= 11) {
        println!("show moji {}", num)
    } else {
        println!("no")
    }
    //array
    let point = [40, 50, 60, 70, 80];
    //配列の中身表示
    println!("{:?}", point);
    //配列の要素数
    println!("len = {}", point.len());

    let banana = Item("バナナ".to_string(), 300);
    let apple = Item("アップル".to_string(), 200);
    let mango = Item("マンゴー".to_string(), 500);

    //ベクターに追加
    let items = vec![banana, apple, mango];
    //&を使ってitemsの所有権を借用する 参照を渡す
    let total = print_and_items(&items);
    println!("合計{}円です", total);
}
// &Vec<Item>がタプルの参照を指定している
fn print_and_items(items: &Vec<Item>) -> i64 {
    let mut total = 0;
    //
    for it in items {
        //タプルの位置を指定
        total += it.1;
    }
    //合計金額を返す
    total
}
