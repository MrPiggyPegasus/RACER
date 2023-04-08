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

    pub fn is_draw(&self) -> bool {
        self.p1 == !self.p2
    }

    pub fn p1_won(&self) -> bool {
        // compress to the right by one and remove horizontal gaps of 1
        // compress again, if any bits are 1 then p1 has a horizontal win
        let mut y = self.p1 & (self.p1 >> 8);
        if y & (y << 16) > 0 {
            return true;
        }
        // check verticals by compressing up by one
        y = self.p1 & (self.p1 >> 1);
        if y & (y << 2) > 0 {
            return true;
        }
        y = self.p1 & (self.p1 >> 7); // check \ facing diagonals
        if y & (y << 14) > 0 {
            return true;
        }
        y = self.p1 & (self.p1 >> 9);
        if (y & (y >> 2 * 9)) > 0 {
            return true;
        }
        false
    }

    pub fn p2_won(&self) -> bool {
        // compress to the right by one and remove horizontal gaps of 1
        // compress again, if any bits are 1 then p1 has a horizontal win
        let mut y = self.p2 & (self.p2 >> 8);
        if y & (y << 16) > 0 {
            return true;
        }
        // check verticals by compressing up by one
        y = self.p2 & (self.p2 >> 1);
        if y & (y << 2) > 0 {
            return true;
        }
        y = self.p2 & (self.p2 >> 7); // check \ facing diagonals
        if y & (y << 14) > 0 {
            return true;
        }
        y = self.p2 & (self.p2 >> 9);
        if (y & (y >> 2 * 9)) > 0 {
            return true;
        }
        false
    }
    /// 1 -> true
    /// 2 -> false
    pub fn current_player(&self) -> bool {
        self.p1.count_ones() <= self.p2.count_ones()
    }

    pub fn play(&mut self, col: i8) {
        for row in 0..5 {
            // ensure that the slot is empty
            if (self.p1 & (2 as u64).pow(row as u32 + (col * 8) as u32)) == 0
                && (self.p2 & (2 as u64).pow(row as u32 + (col * 8) as u32)) == 0
            {
                if self.current_player() {
                    self.p1 += (2 as u64).pow(row as u32 + (col * 8) as u32) as u64;
                    return;
                } else {
                    self.p2 += (2 as u64).pow(row as u32 + (col * 8) as u32) as u64;
                    return;
                }
            }
        }
    }

    pub fn format_bb(bb: u64) {
        let mut output = String::new();
        for row in (0..5).rev() {
            for col in 0..7 {
                let target_bit = (2 as u64).pow(row + (col * 8));
                if (bb & target_bit) > 0 {
                    output += "1  "
                } else {
                    output += ".  "
                }
            }
            output += "\n";
        }
        println!("{}", output);
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        for row in (0..5).rev() {
            for col in 0..7 {
                let target_bit = (2 as u64).pow(row + (col * 8));
                if (self.p1 & target_bit) > 0 {
                    output += "1  "
                } else if (self.p2 & target_bit) > 0 {
                    output += "2  "
                } else {
                    output += ".  "
                }
            }
            output += "\n";
        }
        output
    }
}
