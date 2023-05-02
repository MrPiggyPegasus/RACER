// Copyright (c) 2023.  "MrPiggyPegasus"
// This file is part of the RACER connect 4 engine,see
// https://github.com/MrPiggyPegasus/RACER
// All components of this project are subject to the MIT License, see LICENSE.txt

// Bitboard representation of the position by this map:
//     . .  .  .  .  .  .
//     . .  .  .  .  .  .
//     5 13 21 29 37 45 53
//     4 12 20 28 36 44 52
//     3 11 19 27 35 43 51
//     2 10 18 26 34 42 50
//     1  9 17 25 33 41 49
//     0  8 16 24 32 40 48

pub struct Bitboard {
    pub p1: u64,
    pub p2: u64,
}

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard { p1: 0, p2: 0 }
    }

    pub fn is_over(&self) -> bool {
        self.is_draw() || self.p2_won() || self.p1_won()
    }

    pub fn is_draw(&self) -> bool {
        self.p1 | self.p2 == 17802464409370431
    }

    pub fn p1_won(&self) -> bool {
        // compress to the right by one and remove horizontal gaps of 1
        // compress again, if any bits are 1 then p1 has a horizontal win
        if (self.p1 & (self.p1 >> 8)) & ((self.p1 & (self.p1 >> 8)) >> 16) != 0 {
            return true;
        }
        // check verticals by compressing up by one
        if (self.p1 & (self.p1 >> 1)) & ((self.p1 & (self.p1 >> 1)) >> 2) != 0 {
            return true;
        }
        // check \ facing diagonals
        if (self.p1 & (self.p1 >> 7)) & ((self.p1 & (self.p1 >> 7)) >> 14) != 0 {
            return true;
        }
        // check / facing diagonals
        if ((self.p1 & (self.p1 >> 9)) & ((self.p1 & (self.p1 >> 9)) >> 18)) != 0 {
            return true;
        }
        false
    }

    pub fn p2_won(&self) -> bool {
        // compress to the right by one and remove horizontal gaps of 1
        // compress again, if any bits are 1 then p1 has a horizontal win
        if (self.p2 & (self.p2 >> 8)) & ((self.p2 & (self.p2 >> 8)) >> 16) != 0 {
            return true;
        }
        // check verticals by compressing up by one
        if (self.p2 & (self.p2 >> 1)) & ((self.p2 & (self.p2 >> 1)) >> 2) != 0 {
            return true;
        }
        // check \ facing diagonals
        if (self.p2 & (self.p2 >> 7)) & ((self.p2 & (self.p2 >> 7)) >> 14) != 0 {
            return true;
        }
        // check / facing diagonals
        if ((self.p2 & (self.p2 >> 9)) & ((self.p2 & (self.p2 >> 9)) >> 18)) != 0 {
            return true;
        }
        false
    }
    pub fn is_legal_move(&self, col: i8) -> bool {
        self.p1 & (1 << ((col * 8) + 4)) == 0 && self.p2 & (1 << ((col * 8) + 4)) == 0
    }

    /// 1 -> true
    /// 2 -> false
    pub fn current_player(&self) -> bool {
        self.p1.count_ones() <= self.p2.count_ones()
    }

    pub fn play(&mut self, col: i8) {
        for row in 0..6 {
            // ensure that the slot is empty
            if (self.p1 & 1 << (row as u32 + (col * 8) as u32)) == 0
                && (self.p2 & 1 << (row as u32 + (col * 8) as u32)) == 0
            {
                if self.current_player() {
                    self.p1 ^= 1 << row + (col * 8);
                    return;
                } else {
                    self.p2 ^= 1 << row + (col * 8);
                    return;
                }
            }
        }
    }

    /// removes the highest token from a column
    pub fn pop(&mut self, col: i8) {
        if !self.current_player() {
            for row in (0..6).rev() {
                if self.p1 & (1 << ((col * 8) + row)) != 0 {
                    self.p1 ^= 1 << ((col * 8) + row);
                    return;
                }
            }
        } else {
            for row in (0..6).rev() {
                if self.p2 & (1 << ((col * 8) + row)) != 0 {
                    self.p2 ^= 1 << ((col * 8) + row);
                    return;
                }
            }
        }
    }

    pub fn is_empty_column(&self, col: i8) -> bool {
        ((self.p1 & (1 << (col * 8))) == 0) && (self.p2 & (1 << (col * 8))) == 0
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        for row in (0..5).rev() {
            for col in 0..7 {
                let target_bit = 1 << (row + (col * 8));
                if (self.p1 & target_bit) > 0 {
                    output += "X  "
                } else if (self.p2 & target_bit) > 0 {
                    output += "O  "
                } else {
                    output += ".  "
                }
                if (self.p1 & target_bit > 0) && (self.p2 & target_bit > 0) {
                    println!("aaa");
                }
            }
            output += "\n";
        }
        output
    }

    pub fn number_of_pieces(&self) -> i8 {
        (self.p1.count_ones() + self.p2.count_ones()) as i8
    }
}
