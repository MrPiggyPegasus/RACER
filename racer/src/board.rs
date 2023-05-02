// Copyright (c) 2023.  "MrPiggyPegasus"
// This file is part of the RACER connect 4 engine,see
// https://github.com/MrPiggyPegasus/RACER
// All components of this project are subject to the MIT License, see LICENSE.txt

use crate::bitboards::Bitboard;
use crate::search::search;
use std::fmt;
use std::fmt::Formatter;

pub struct Board {
    pub bitboard: Bitboard,
    pub pgn: String,
}

impl Board {
    pub fn new() -> Board {
        Board {
            bitboard: Bitboard::new(),
            pgn: String::from(""),
        }
    }

    pub fn pop(&mut self, col: i8) -> Result<(), ColumnIsEmptyError> {
        if self.is_empty_column(col) {
            return Err(ColumnIsEmptyError)
        }
        self.bitboard.pop(col);
        Ok(())
    }

    pub fn is_empty_column(&self, col: i8) -> bool {
        if col < 0 || col > 6 {
            return false
        }
        self.bitboard.is_empty_column(col)
    }

    pub fn undo_move(&mut self) -> Result<(), NoMoveToUndoError> {
        if self.pgn.len() == 0 {
            return Err(NoMoveToUndoError)
        }
        self.bitboard.pop(
            self.pgn
                .chars()
                .nth(self.pgn.len() - 1)
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
        );
        self.pgn.truncate(self.pgn.len() - 1);
        Ok(())
    }

    pub fn parse_pgn(pgn: &str) -> Result<Board, InvalidPgnError> {
        if Board::is_valid_pgn(pgn) {
            let mut pos = Board::new();
            for char in pgn.chars() {
                pos.play(char.to_string().parse::<i8>().unwrap()).unwrap()
            }
            Ok(pos)
        } else {
            Err(InvalidPgnError)
        }
    }

    pub fn play(&mut self, col: i8) -> Result<(), IllegalMoveError> {
        if !self.is_valid_move(col) {
            Err(IllegalMoveError)
        } else if self.bitboard.is_over() {
            Err(IllegalMoveError)
        } else {
            self.bitboard.play(col);
            self.pgn += &*col.to_string();
            Ok(())
        }
    }

    pub fn is_valid_pgn(pgn: &str) -> bool {
        let mut pos = Board::new();
        for char in pgn.chars() {
            match char.to_string().parse::<i8>() {
                Ok(col) => match pos.play(col) {
                    Ok(_) => (),
                    Err(_) => return false,
                },
                Err(_) => return false,
            }
        }
        true
    }

    pub fn to_string(&self) -> String {
        self.bitboard.to_string()
    }

    pub fn is_over(&self) -> bool {
        self.bitboard.is_over()
    }

    pub fn p1_won(&self) -> bool {
        self.bitboard.p1_won()
    }

    pub fn p2_won(&self) -> bool {
        self.bitboard.p2_won()
    }

    pub fn is_draw(&self) -> bool {
        self.bitboard.is_draw()
    }

    pub fn best_move(&mut self) -> Result<i8, PositionAlreadyOverError> {
        if self.is_over() {
            return Err(PositionAlreadyOverError);
        }
        let best_move = search(&mut self.bitboard, i8::MIN, i8::MAX, 13).1;
        if best_move != 9 {
            return Ok(best_move)
        } else {
            // emergency mode if minimax fails to provide a move
            // this should never be called but lets just make sure that
            // it doesnt break anything if that happens in production
            for i in 0..7 {
                if self.is_valid_move(i) {
                    return Ok(i)
                }
            }
        }
        Err(PositionAlreadyOverError)
    }

    pub fn current_player(&self) -> i8 {
        (self.bitboard.current_player() as i8 * 2) - 1
    }

    pub fn is_valid_move(&self, col: i8) -> bool {
        col >= 0 && col < 7 && self.bitboard.is_legal_move(col)
    }

    pub fn eval(&mut self) -> Result<i8, PositionAlreadyOverError> {
        if self.is_over() {
            return Err(PositionAlreadyOverError)
        }
        Ok(search(&mut self.bitboard, i8::MIN, i8::MAX, 13).0)
    }

    pub fn best_move_with_eval(&mut self) -> Result<(i8, i8), PositionAlreadyOverError> {
        if self.is_over() {
            return Err(PositionAlreadyOverError)
        }
        let result = search(&mut self.bitboard, i8::MIN, i8::MAX, 13);
        Ok((result.0, result.1))
    }
}

#[derive(Debug, Clone)]
pub struct ColumnIsEmptyError;
impl fmt::Display for ColumnIsEmptyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "There are no tokens in that column to pop anyway!")
    }
}

#[derive(Debug, Clone)]
pub struct IllegalMoveError;
impl fmt::Display for IllegalMoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "That move is illegal!")
    }
}

#[derive(Debug, Clone)]
pub struct PositionAlreadyOverError;
impl fmt::Display for PositionAlreadyOverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "That position is already over, so it can't be analysed!")
    }
}

#[derive(Debug, Clone)]
pub struct InvalidPgnError;
impl fmt::Display for InvalidPgnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "That PGN is invalid!")
    }
}

#[derive(Debug, Clone)]
pub struct NoMoveToUndoError;
impl fmt::Display for NoMoveToUndoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "There are no more moves to undo!")
    }
}
