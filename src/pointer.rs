
#![allow(unused)]

// use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, stdin, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    //RustにはBoxというオブジェクトがあります。これを使うとオブジェクトのメモリをヒープに確保することができます。
    //値を新たに生成する関数のうち、自明かつ最もシンプルなものがある場合に、 new() という名前をつけるという慣例が存在
    #[derive(Debug)]
    struct TreeNode<T> {
        //Rust の Option<T> 型は、値が 存在しない 可能性を暗示する列挙型
        //あればSome
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key,
            }
        }
        //self : 所有権の移動
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }
    let node1 = TreeNode::new(1)
        .left(TreeNode::new(2))
        .right(TreeNode::new(3));
    println!("aa {:?}", node1);
}
