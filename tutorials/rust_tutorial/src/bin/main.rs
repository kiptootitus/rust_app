#![allow(warnings)]

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use rand::Rng;
use std::cmp::Ordering;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("What is your name, sir?");

    // Use BufReader with stdin for AsyncBufReadExt::read_line
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut name = String::new();

    reader.read_line(&mut name).await?;

    println!("Nice to meet you, {}!", name.trim());

    Ok(())
}
