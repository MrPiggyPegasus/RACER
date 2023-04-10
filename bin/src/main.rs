use std::time::SystemTime;

use racer::bitboards::*;
use racer::search::*;

fn main() {
    let mut pos = Bitboard::new();
    let start = SystemTime::now();
    println!("{}", search(&mut pos, i8::MIN, i8::MAX, 15).1);
    println!("time: {}s", start.elapsed().unwrap().as_millis() as f64 / 1000 as f64);
}
