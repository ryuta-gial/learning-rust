
#![allow(unused)]
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, stdin, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};
//ライブラリ
use std::thread;
use std::time::Duration;

//並列処理
fn main() {
    let thread1 =  thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread : {}", i);
            //処理をある時間だけ停止させる
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("Main thread : {}", i);
        //処理をある時間だけ停止させる
        thread::sleep(Duration::from_millis(1));
    }
    //値を返す
     thread1.join().unwrap();

}
