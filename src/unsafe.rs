
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

//時刻
use std::time::{SystemTime, UNIX_EPOCH};
//グローバル変数
static mut SEED: u32 = 0;
//unsafe な関数
//u32 32ビット符号なし整数 0-4294967295
unsafe fn rand_global(start: u32, end: u32) -> u32 {
    if SEED == 0 {

        //現在の時刻からエポックを生x成する:
        //duration_since  時間差 uinxtimeが取得できたかチェック
        let epoc = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        //as_millis 実際の値を取得
        SEED = epoc.as_millis() as u32;
        println!("{}",SEED);
    }
    //乱数を生成
    //ビット演算
    //<<左シフト
    // ^=ビットxor後に代入
    SEED ^= SEED << 13;
    SEED ^= SEED << 17;
    SEED ^= SEED << 5;
    return SEED % (end - start + 1) + start;
}

fn main() {
    for _ in 0..100 {
        //安全でないことを明示
        unsafe {
            //乱数を生成
            let v = rand_global(1, 6);
           // println!("{}", v);
        }
    }
}
