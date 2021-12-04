use std::fmt::Display;

use crate::{Day, Runnable};
use colored::*;
use const_format::formatcp;

const CURRENT_DAY: u8 = 4;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

#[derive(Clone, Debug)]
pub struct Data {
    bingo_numbers: Vec<u32>,
    bingo_boards: Vec<[(u32, bool); 5 * 5]>,
}

impl Data {
    fn new() -> Self {
        Self {
            bingo_numbers: Vec::new(),
            bingo_boards: Vec::new(),
        }
    }

    fn is_bingo(&mut self) -> Vec<i32> {
        // Check rows
        let mut vec = Vec::new();
        for (i, board) in self.bingo_boards.iter().enumerate() {
            for row in 0..5 {
                for x in 0..5 {
                    if board[row * 5 + x].1 == false {
                        break;
                    }
                    if x == 4 {
                        vec.push(i as i32);
                    }
                }
                for y in 0..5 {
                    if board[y * 5 + row].1 == false {
                        break;
                    }
                    if y == 4 {
                        vec.push(i as i32);
                    }
                }
            }
        }

        if vec.is_empty() {
            vec![-1]
        } else {
            vec
        }
    }

    fn check_number(&mut self) -> u32 {
        let number = self.bingo_numbers.pop();
        if let Some(num) = number {
            for board in &mut self.bingo_boards {
                for mut entry in board {
                    if entry.0 == num {
                        entry.1 = true;
                    }
                }
            }
        }

        number.unwrap()
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for elem in &self.bingo_boards {
            for y in 0..5 {
                for x in 0..5 {
                    if elem[y * 5 + x].1 == true {
                        output += &format!("{}\t", elem[y * 5 + x].0.to_string().bold().red());
                    } else {
                        output += &format!("{}\t", elem[y * 5 + x].0);
                    }
                }
                output += "\n";
            }
            output += "\n";
        }
        write!(f, "{}", output)
    }
}

impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init() -> (Self, Data) {
        let mut v: Data = Data::new();
        std::fs::read_to_string(FILE)
            .unwrap()
            .lines()
            .enumerate()
            .for_each(|(i, s)| {
                // In the first row, there are the drawn bingo numbers
                if i == 0 {
                    for n in s.split(',').rev() {
                        v.bingo_numbers.push(n.parse().unwrap())
                    }
                } else if i == 1 {
                    v.bingo_boards.push([(0, false); 5 * 5]);
                } else {
                    // i-2 basically "cuts" the header away
                    let row = (i - 1) % 6;
                    // If we are on an empty line, allocate space for a new bingo card
                    if row == 0 {
                        v.bingo_boards.push([(0, false); 5 * 5]);
                    } else {
                        // index is a value between 1 and 5
                        // s is a space separated list of numbers
                        for (col, n) in s.split_whitespace().enumerate() {
                            v.bingo_boards.last_mut().unwrap()[(row - 1) * 5 + col] =
                                (n.parse().unwrap(), false);
                        }
                    }
                }
            });

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) {
        let (number, board) = loop {
            let num = data.check_number();
            let board = *data.is_bingo().first().unwrap();
            if board != -1 {
                break (num, board);
            }
        };

        let unchecked_number_sum = data.bingo_boards[board as usize]
            .iter()
            .fold(
                0,
                |sum, elem| {
                    if elem.1 == false {
                        sum + elem.0
                    } else {
                        sum
                    }
                },
            );

        println!(
            "Unchecked Numbers: {}, Winning Number: {} => {}",
            unchecked_number_sum,
            number,
            unchecked_number_sum * number
        ); // Too low: 56420
    }
    fn two(&self, data: &mut Data) {
        // Remove all but one bingo board
        loop {
            data.check_number();
            let board = data.is_bingo();
            let mut removed = 0;
            for b in board {
                if data.bingo_boards.len() == 1 {
                    break;
                }
                if b == -1 {
                    break;
                }
                data.bingo_boards.remove((b - removed) as usize);
                removed += 1;
            }
            if data.bingo_boards.len() == 1 {
                // If we broke out of the for loop
                break;
            }
        }

        // Now play this board until we've got a bingo!
        let number = loop {
            let num = data.check_number();
            let board = *data.is_bingo().first().unwrap();
            if board != -1 {
                break (num);
            }
        };

        let unchecked_number_sum =
            data.bingo_boards[0].iter().fold(
                0,
                |sum, elem| {
                    if elem.1 == false {
                        sum + elem.0
                    } else {
                        sum
                    }
                },
            );

        println!(
            "Unchecked Numbers: {}, Winning Number: {} => {}",
            unchecked_number_sum,
            number,
            unchecked_number_sum * number
        );
        // Too low: 3640
    }
}
