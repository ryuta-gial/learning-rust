
#![allow(unused)]

// use rand::Rng;
// use std::cmp::Ordering;
// use std::fs::File;
// use std::io::{self, stdin, Read};
// use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    const PI: f32 = 3.141592;
    //トレイトを定義
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
    //構造体を定義
    struct Rectangle {
        length: f32,
        width: f32,
    };
    struct Circle {
        length: f32,
        width: f32,
    };
    //implを使ってトレイトを実装
    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle { length, width };
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }
    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }
    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);
    println!("Rec Area :{}", rec.area());
    println!("Circ Area :{}", circ.area());
}
