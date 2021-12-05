use std::{collections::HashMap, convert::TryInto};

use crate::{Day, Runnable};
use const_format::formatcp;
use num::Integer;

const CURRENT_DAY: u8 = 5;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

type Position = (i16, i16);

#[derive(Clone, Debug)]
pub struct Line {
    start_pos: Position,
    end_pos: Position,
    k: (i16, i16),
}

impl Line {
    fn from(start_pos: Position, end_pos: Position) -> Self {
        Self{
            start_pos,
            end_pos,
            k: {
                let x = end_pos.0 - start_pos.0;
                let y = end_pos.1 - start_pos.1;

                let divisor = x.gcd(&y);

                (x / divisor, y / divisor)
            }
        }
    }

    fn get_k(&self) -> (i16, i16) {
        self.k
        // let x = self.end_pos.0 - self.start_pos.0;
        // let y = self.end_pos.1 - self.start_pos.1;

        // let divisor = x.gcd(&y);

        // (x / divisor, y / divisor)
    }
}

type Data = Vec<Line>;
impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init() -> (Self, Data) {
        let v: Data = std::fs::read_to_string(FILE)
            .unwrap()
            .lines()
            .map(|s| {
                let splitted = s.split(" -> ").collect::<Vec<&str>>();
                let start: Vec<i16> = splitted[0].split(',').map(|num| num.parse().unwrap()).collect();
                let end: Vec<i16> = splitted[1].split(',').map(|num| num.parse().unwrap()).collect();

                Line::from((start[0], start[1]), (end[0], end[1]))
            })
            .collect();

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 {
        let mut positions: HashMap<Position, u32> = HashMap::new();
        data.iter().filter(|line| {
            let k = line.get_k();
            k.0 == 0 || k.1 == 0
        }).for_each(|line| {
            // In this part, we have k amount of steps
            let k = line.get_k();
            let mut step = 0;
            loop {
                let pos = (line.start_pos.0 + step * k.0, line.start_pos.1 + step * k.1);

                let already_existing = positions.get(&pos).unwrap_or(&0).clone();
                positions.insert(pos, already_existing + 1);

                if pos == line.end_pos {
                    break;
                }

                step += 1;
            }
        });

        positions.iter().filter(|(_,f)| {
            f >= &&2
        }).count().try_into().unwrap()
    }
    fn two(&self, data: &mut Data) -> u64 {
        let mut positions: HashMap<Position, u8> = HashMap::new();
        data.iter().for_each(|line| {
            // In this part, we have k amount of steps
            let k = line.get_k();
            let mut step = 0;
            loop {
                let pos = (line.start_pos.0 + step * k.0, line.start_pos.1 + step * k.1);

                let already_existing = positions.get(&pos).unwrap_or(&0).clone();
                positions.insert(pos, already_existing + 1);

                if pos == line.end_pos {
                    break;
                }

                step += 1;
            }
        });

        positions.iter().filter(|(_,f)| {
            f >= &&2
        }).count().try_into().unwrap()
    }
}


