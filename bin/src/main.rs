// Copyright (c) 2023.  "MrPiggyPegasus"
// This file is part of the RACER connect 4 engine,see
// https://github.com/MrPiggyPegasus/RACER
// All components of this project are subject to the MIT License, see LICENSE.txt

mod nogui;

use racer::Board;
use crate::nogui::menu;

fn main() {
    let mut pos = Board::new();
    pos.play(2).unwrap();
    pos.play(2).unwrap();
    pos.undo_move();
    println!("{}", pos.to_string());
}
