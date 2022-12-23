
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
struct PrimeIterator {
    n: u8,
}

//メソッドを定義
//impl 構造体にメソッドを加える
//イテレータを作成しただけでは何もしません。イテレータを加工して新たなイテレータを作成するか、先ほどのnextやfor文などで消費する
impl PrimeIterator {
    //1
    //デフォルト
    fn new() -> Self {
        PrimeIterator { n: 1 }
    }

    //self.nが素数かどうか調べる
    fn is_prime(&self) -> bool {
        for i in 2..self.n {
            if self.n % i == 0 {
                return false;
            }
        }
        return true;
    }
}
//イテレータを実装 構造体名
impl Iterator for PrimeIterator {
    type Item = u8;
    //next()さえ実装すれば，あとはデフォルト定義が存在
    //Option と呼ばれるジェネリックな列挙型が組み込まれており、null を使わずに null 許容な値を表現できます。
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    //next()はイテレータの内部状態を1つ進めるメソッドです。
    // next()を呼び出すとイテレータが進みます。
    //
    // next()はOption<Self::Item>を返してきます。
    fn next(&mut self) -> Option<Self::Item> {
        //素数を探す
        loop {
            self.n += 1;
            //8ビットを超える素数を調べない
            //8ビット（1バイト）の符号なし整数型の値の範囲は0～255
            //u8::MAX この整数型で表現できる最大値
            if std::u8::MAX == self.n {
                return None;
            }
            //素数かどうか判定
            if self.is_prime() {
                return Some(self.n);
            }
        }
    }
}

fn main() {
    //イテレーターを生成
    let prime_iter = PrimeIterator::new();
    for n in prime_iter {
        print!("{},", n);
    }
}
