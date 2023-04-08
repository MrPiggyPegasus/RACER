use crate::bitboards::Bitboard;

pub fn search(pos: &mut Bitboard, mut alpha: i8, mut beta: i8, depth: u8) -> (i8, i8) {
    if pos.p1_won() {
        return ((100 - pos.number_of_pieces()), 9)
    }
    if pos.p2_won() {
        return ((-100) + pos.number_of_pieces(), 9)
    }
    if pos.is_draw() {
        return (0, 9)
    }
    let mut best_move = 9;
    let mut best_eval:i8;
    if pos.current_player() {
        best_eval = i8::MIN;
        for col in 0..7 {
            if pos.is_legal_move(col) {
                pos.play(col);
                let result = search(pos, alpha, beta, depth-1);
                pos.pop(col);
                if result.0 > best_eval {
                    best_eval = result.0;
                    best_move = col;
                    if best_eval > beta {
                        break
                    }
                    alpha = best_eval;
                }
            }
        }
    } else {
        best_eval = i8::MAX;
        for col in 0..7 {
            if pos.is_legal_move(col) {
                pos.play(col);
                let result = search(pos, alpha, beta, depth-1);
                pos.pop(col);
                if result.0 < best_eval {
                    best_eval = result.0;
                    best_move = result.1;
                    if best_eval < alpha {
                        break
                    }
                    beta = best_eval
                }
            }
        }
    }
    (best_eval, best_move)
}
