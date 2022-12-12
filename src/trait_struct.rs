
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

//異なる構造体の振る舞いを共通化することができるtrait
//宝石箱の振る舞いを定義するtrait
trait TreasureBox {
    //&self 所有権はいらず、構造体のデータを読み込みたいだけで、書き込む必要はない
    fn open(&self, key_no: i32) -> bool;
    fn check(&self);
}

//宝石箱を表現するstructを定義
#[derive(Debug)]
struct JewelryBox {
    price: i32,
    key_no: i32,
}

//impl トレイト名 for 構造体名
//3→
impl TreasureBox for JewelryBox {
    //4→渡ってくる構造体のkey_noは1
    //２つ目の引数には2
    fn open(&self, key_no: i32) -> bool {
        //指定の鍵のみで開く
        println!("ここが読まれる{}", self.key_no);
        self.key_no == key_no
    }
    fn check(&self) {
        //&self 所有権はいらず、構造体のデータを読み込みたいだけで、書き込む必要はない
        println!("宝石箱だった!金貨{}枚入手。", self.price);
    }
}

//罠の箱を表現するsttruct
#[derive(Debug)]
struct TrapBox {
    damage: i32,
}

//impl トレイト名 for 構造体名
impl TreasureBox for TrapBox {
    //_から始まると未使用でエラーでない
    //7→
    fn open(&self, _key: i32) -> bool {
        return true;//どんな鍵でも開く
    }
    //9→
    fn check(&self) {
        println!("罠だった。{}のダメージ", self.damage);
    }
}
//引数にTreasureBoxトレイトを実装したオブジェクトを指定できるようにする
//1→
fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    //2→
    //6→
    if !tbox.open(key_no) {
        println!("鍵が合わず宝箱が開きません");
        return;
    }
    //8→
    tbox.check();
}

fn main() {
    let box1 = JewelryBox {
        price: 30,
        key_no: 1,
    };
    let box2 = TrapBox { damage: 3 };
    let box3 = JewelryBox {
        price: 20,
        key_no: 2,
    };
    let my_key = 2;
    //0→
    open_box(&box1, my_key);
    //5→
    open_box(&box2, my_key);
    open_box(&box3, my_key);
}
