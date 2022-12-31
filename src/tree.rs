
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
use std::{env, path};


fn main() {
    //コマンドライン引数を得る
    let args: Vec<String> = env::args().collect();
    //カレントディレクトリを指定
    let mut target_dir = "./src";
    if args.len() >= 2 {
        target_dir = &args[1];
    }
    let target = path::PathBuf::from(target_dir);
    println!("{}", target_dir);
    tree(&target, 0);
}

//再帰的にファイルを一覧表示
fn tree(target: &path::PathBuf, level: isize) {
    //ファイル一覧を取得
    let files = target.read_dir().expect("存在しないパス");

    for ent in files {
        let path = ent.unwrap().path();
        for _ in 1..=level {
            print!("|　");
        }
        //ファイル名を得る
        let fname = path.file_name().unwrap().to_string_lossy();
        //ディレクトリなら再帰的に表示
        if path.is_dir() {
            println!("|-- <{}>", fname);
            tree(&path, level + 1);
            continue;
        }
        //ファイル名を表示
        println!("|-- {}", fname);
    }
}
