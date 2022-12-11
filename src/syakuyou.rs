
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
    let mut g1 = String::from("過ちを見過ごす人は美しい");
    let mut g2 = String::from("美しいものは美しい");
    let mut g3 = String::from("参照がだめなら実体で");
    //所有権が移動する
    //show_messasge(g1);
    //所有権は↑に移っているのでエラーになる
    //println!("{}",g1);

    //所有権をg1に戻すとエラーではない
    g1 = show_messasge(g1);
    println!("{}", g1);

    //関数の引数に参照を指定する「借用」
    show_messasge2(&g2);
    println!("{}", g2);

    let m = show_messasge3(&mut g3);
}

fn show_messasge(mesaage: String) -> String {
    println!("{}", mesaage);
    return mesaage;
}
fn show_messasge2(mesaage: &String) {
    println!("{}", mesaage);
}
fn show_messasge3(mesaage: &mut String) {
    let msg = String::from("これは") + mesaage;
        println!("{}", msg);

}
