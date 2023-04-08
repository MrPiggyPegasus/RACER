use racer::bitboards::Bitboard;

fn main() {
    let mut pos = Bitboard::new();
    pos.play(0);
    pos.play(1);
    pos.play(1);
    pos.play(2);
    pos.play(2);
    pos.play(3);
    pos.play(2);
    pos.play(3);
    pos.play(3);
    pos.play(0);
    pos.play(3);
    println!("{}", pos.to_string());
    println!("{}", pos.p1_won());
    println!("{}", pos.p2_won());
    println!("{}", pos.is_draw());
}
