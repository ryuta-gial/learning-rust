
#![allow(unused)]

// use rand::Rng;
// use std::cmp::Ordering;
// use std::fs::File;
// use std::io::{self, stdin, Read};
// use std::io::{BufRead, BufReader, ErrorKind, Write};

//標準
use std::collections::HashMap;

fn main() {
    let mut heroes = HashMap::new();
    //値を代入
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    //ベクター（Vec）のメソッドiter()
    //各要素を取り出す
    // The Flash ......から取り出される
    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }
    //特定のkeyを探す
    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Hey Batman is a hero"),
            None => println!("Hey Batman is not hero"),
        }
    }
}
