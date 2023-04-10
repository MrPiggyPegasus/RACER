use racer::bitboards::*;
use racer::search::*;

fn main() {
    let mut pos = Bitboard::new();
    pos.play(2);
    pos.play(2);
    pos.play(2);
    println!("{}", search(&mut pos, i8::MIN, i8::MAX, 12).1);
}
