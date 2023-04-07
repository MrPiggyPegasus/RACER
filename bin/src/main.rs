use racer::bitboards::Bitboard;

fn main() {
    let mut pos = Bitboard::new();
    println!("{}", pos.current_player());
    pos.play(3);
    pos.play(3);
    println!("{}", pos.p2);
    println!("{}", pos.to_string());
}
