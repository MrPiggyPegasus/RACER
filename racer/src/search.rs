use crate::bitboards::Bitboard;

pub fn search(pos: &mut Bitboard, mut alpha: i8, mut beta: i8, depth: u64) -> (i8, i8) {
    if pos.p1_won() {
        println!("p1_won:\n{}", pos.to_string());
        return ((100 - pos.number_of_pieces()), 9);
    }
    if pos.p2_won() {
        println!("p2_won:\n{}", pos.to_string());
        return ((-100) + pos.number_of_pieces(), 9);
    }
    if pos.is_draw() || depth == 0 {
        println!("draw:\n{}", pos.to_string());
        return (0, 9);
    }
    return if pos.current_player() {
        let mut max_eval = i8::MIN + 10;
        let mut max_move = 9;
        for col in vec![3,4,2,5,1,6,0] {
            if !pos.is_legal_move(col) {
                continue
            }
            pos.play(col);
            let eval = search(pos, alpha, beta, depth-1).0;
            pos.pop(col);
            if eval > max_eval {
                max_eval = eval;
                max_move = col;
                if eval >= beta {
                    break;
                }
                if eval > alpha {
                    alpha = eval;
                }
            }
        }
        (max_eval, max_move)
    } else {
        let mut min_eval = i8::MAX - 10;
        let mut min_move = 9;
        for col in vec![3,4,2,5,1,6,0] {
            if !pos.is_legal_move(col) {
                continue
            }
            pos.play(col);
            let eval = search(pos, alpha, beta, depth-1).0;
            pos.pop(col);
            if eval < min_eval {
                min_eval = eval;
                min_move = col;
                if eval <= alpha {
                    break
                }
                if eval < beta {
                    beta = eval
                }
            }
        }
        (min_eval, min_move)
    }
}
