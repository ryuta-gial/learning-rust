
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
    //構造体を定義
    pub struct Bank {
        balance: f32,
    }
    //関数
    //Arc<Mutex<  スレッドの中で使用可能になるまでロックする
    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!(
                "Current Balance : {} withdrawal a smaller amount",
                bank_ref.balance
            );
        } else {
            bank_ref.balance -= amt;

            //引いた数と残高
            println!(
                "Customer withdrew {} Current Balance {}",
                amt, bank_ref.balance
            );
        }
    }
    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00);
    }
    //初期値20.00
    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 20.00 }));
    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| customer(bank_ref))
    });
    for handle in handles {
        handle.join().unwrap();
    }
    //複数スレッド
    println!("Total {}", bank.lock().unwrap().balance);
}
