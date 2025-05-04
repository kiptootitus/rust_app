#![allow(warnings)]

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use rand::Rng;
use std::cmp::Ordering;

#[tokio::main]

async fn main() {
    println!("MAX u32: {}", u32::MAX);
    println!("MAX u64: {}", u64::MAX);
    println!("MAX i32: {}", i32::MAX);
    println!("MAX f32: {}", f32::MAX);



    // doubles quotes create a string while single create a character
    let _my_char = 'A';
    

}
