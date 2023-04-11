// Copyright (c) 2023.  "MrPiggyPegasus"
// This file is part of the RACER connect 4 engine,see
// https://github.com/MrPiggyPegasus/RACER
// All components of this project are subject to the MIT License, see LICENSE.txt

mod nogui;

use racer::Board;
use crate::nogui::play;


fn main() {
    play(1, Board::new());
}
