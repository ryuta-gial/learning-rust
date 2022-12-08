
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


//構造体の名前はキャメルケース
#[derive(Debug)]
struct BmiRange {
    min: f64,
    max: f64,
    //この文字列リテラルは「&’static str」という型になります。
    //これは、メモリのどこかに文字分のメモリを確保し、そのメモリの位置を参照するということです
    label: &'static str, //判定ラベル
}

fn main() {
    //変数名はスネークケース
    let height_cm = input("身長(cm)は?");
    let weight = input("体重は(kg)は?");
    let height = height_cm / 100.0;
    
    println!("割った:{}",height);
    //浮動小数点数どうしのべき乗
    let bmi = weight / height.powf(2.0);
    let bmi_list = vec![
        BmiRange {
            min: 0.0,
            max: 18.5,
            label: "低体重",
        },
        BmiRange {
            min: 18.5,
            max: 25.0,
            label: "普通体重",
        },
        BmiRange {
            min: 25.0,
            max: 30.0,
            label: "肥満1度",
        },
        BmiRange {
            min: 30.0,
            max: 35.0,
            label: "肥満2度",
        },
        BmiRange {
            min: 35.0,
            max: 40.0,
            label: "肥満3度",
        },
        BmiRange {
            min: 40.0,
            max: 99.0,
            label: "肥満4度",
        },
    ];
    let mut result = "不明";
    for range in bmi_list {
        if range.min <= bmi && bmi < range.max {
            result = range.label;
            break;
        }
    }
    println!("BMI={:.1},判定＝{}",bmi,result);
}

fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut s = String::new();
    //入力値を求める
    std::io::stdin().read_line(&mut s).expect("入力エラー");
    //エラー
    s.trim().parse().expect("数値変換エラー")
}
