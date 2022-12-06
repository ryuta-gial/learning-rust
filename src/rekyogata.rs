
#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, stdin, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }
    let today: Day = Day::Monday;
    match today {
        Day::Monday => println!("Monday"),
        Day::Tuesday => println!("Tuesday"),
        Day::Wednesday => println!("Wednesday"),
        Day::Thursday => println!("Thursday"),
        Day::Friday => println!("Friday"),
        Day::Saturday => println!("Saturday"),
        Day::Sunday => println!("Sunday"),
    }
    //定義した関数を使っている
    println!("Is today the weekend {}",today.is_weekend());
}
