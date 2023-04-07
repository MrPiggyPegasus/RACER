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
