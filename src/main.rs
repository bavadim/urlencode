use std::io;
use std::io::{BufRead, Write};
use urlencoding::encode;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let s = encode(&line.unwrap());
        println!("{}", s);
    }

    Ok(())
}
