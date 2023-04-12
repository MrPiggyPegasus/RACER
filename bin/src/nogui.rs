// Copyright (c) 2023.  "MrPiggyPegasus"
// This file is part of the RACER connect 4 engine,see
// https://github.com/MrPiggyPegasus/RACER
// All components of this project are subject to the MIT License, see LICENSE.txt

use racer::Board;
use std::io::{stdin, stdout, Write};

pub fn menu() {
    loop {
        println!("\n\n\n\n\n\n\nSelect an option:");
        println!("1. Play against engine");
        println!("2. Play from PGN");
        println!("3. Get best move from PGN");
        println!("4. Exit");

        let mut input = String::new();

        print!("Enter your choice: ");
        let _ = stdout().flush();

        stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(1) => play_against_engine(),
            Ok(2) => play_from_pgn(),
            Ok(3) => get_best_move_from_pgn(),
            Ok(4) => break,
            _ => println!("Invalid input, please try again"),
        }
    }
}

fn play_from_pgn() {
    println!("unimplemented");
}

fn play_against_engine() {
    loop {
        let mut first_player = String::new();
        println!("Who goes first? (1) You or (2) Engine?");
        stdin()
            .read_line(&mut first_player)
            .expect("Failed to read line");

        let first_player: u32 = match first_player.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match first_player {
            1 => {
                play(-1, Board::new());
                break;
            }
            2 => {
                play(1, Board::new());
                break;
            }
            _ => {
                println!("Please enter a valid option");
                continue;
            }
        }
    }
}

fn get_best_move_from_pgn() {

}

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
    if pos.p1_won() {
        if computer_player == 1 {
            println!("Engine Wins!");
        } else {
            println!("You Win!");
        }
    } else if pos.p2_won() {
        if computer_player == -1 {
            println!("Engine Win!");
        } else {
            println!("You Win!");
        }
    } else {
        println!("Draw!");
    }
    println!("{}", pos.to_string());
    println!("\nPress enter to continue!");
    stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");
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
        stdin().read_line(&mut square_str).expect("---");
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
