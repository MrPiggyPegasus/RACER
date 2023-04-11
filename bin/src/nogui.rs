// Copyright (c) 2023.  "MrPiggyPegasus"
// This file is part of the RACER connect 4 engine,see
// https://github.com/MrPiggyPegasus/RACER
// All components of this project are subject to the MIT License, see LICENSE.txt

use racer::Board;
use std::io;

pub fn play(computer_player: i8, mut pos: Board) {
    if computer_player == pos.current_player() {
        loop {
            if pos.is_over() {
                break;
            }
            engine_turn(&mut pos);
            if pos.is_over() {
                break;
            }
            user_turn(&mut pos)
        }
    } else {
        loop {
            if pos.is_over() {
                break;
            }
            user_turn(&mut pos);
            if pos.is_over() {
                break;
            }
            engine_turn(&mut pos)
        }
    }
}

pub fn engine_turn(pos: &mut Board) {
    println!("Calculating...");
    let best_move = pos.best_move().unwrap();
    pos.play(best_move).unwrap();
    println!("Engine's move: {}\n", best_move);
}

pub fn user_turn(pos: &mut Board) {
    println!("{}", pos.to_string());
    loop {
        let mut square_str = String::new();
        if pos.current_player() == 1 {
            println!("\nEnter move for player 1:");
        } else {
            println!("\nEnter move for player 2:");
        }
        io::stdin().read_line(&mut square_str).expect("---");
        square_str.pop();
        if square_str.len() == 1 {
            if square_str.chars().next().unwrap().is_numeric() {
                let square: i8 = square_str.parse().unwrap();
                if pos.is_valid_move(square) {
                    pos.play(square).unwrap();
                    break;
                }
            }
        }
    }
}
