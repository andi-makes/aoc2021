use std::{collections::HashMap, convert::TryInto};

use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 13;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

type Point = (usize, usize);

#[derive(Clone, Debug)]
pub struct Fold {
    along_x: bool,
    number: usize,
}

type Data = (HashMap<Point, u8>, Vec<Fold>);
impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init(_input: &str) -> (Self, Data) {
        let mut v: Data = (HashMap::new(), Vec::new());
        std::fs::read_to_string(FILE)
            .unwrap()
            .lines()
            .for_each(|s| {
                if s.contains("fold along") {
                    if s.contains("x") {
                        v.1.push(Fold {
                            along_x: true,
                            number: s.split('=').last().unwrap().parse().unwrap(),
                        });
                    } else {
                        v.1.push(Fold {
                            along_x: false,
                            number: s.split('=').last().unwrap().parse().unwrap(),
                        });
                    }
                } else if s.contains(",") {
                    let split: Vec<&str> = s.split(',').collect();
                    v.0.entry((split[0].parse().unwrap(), split[1].parse().unwrap()))
                        .or_insert(1);
                    // v.0.push(Point{
                    //     x: split[0].parse().unwrap(),
                    //     y: split[1].parse().unwrap(),
                    // });
                }
            });

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 {
        let instruction = &data.1.first().unwrap();
        if instruction.along_x {
            for entry in data.0.clone() {
                if entry.0 .0 > instruction.number {
                    // *entry.1 = 0;
                    *data.0.entry(entry.0).or_insert(0) = 0;
                    *data
                        .0
                        .entry((
                            instruction.number - (entry.0 .0 - instruction.number),
                            entry.0 .1,
                        ))
                        .or_insert(1) = 1;
                }
            }
        } else {
            for entry in data.0.clone() {
                if entry.0 .1 > instruction.number {
                    // *entry.1 = 0;
                    *data.0.entry(entry.0).or_insert(0) = 0;
                    *data
                        .0
                        .entry((
                            entry.0 .0,
                            instruction.number - (entry.0 .1 - instruction.number),
                        ))
                        .or_insert(1) = 1;
                }
            }
        }
        data.0.iter().filter(|entry| entry.1 == &1).count() as u64

        // Too high: 749
    }
    fn two(&self, data: &mut Data) -> u64 {
        for instruction in &data.1 {
            if instruction.along_x {
                for entry in data.0.clone() {
                    if entry.0 .0 > instruction.number {
                        if data.0[&entry.0] != 0 {
                            *data.0.entry(entry.0).or_insert(0) = 0;
                            *data
                                .0
                                .entry((
                                    (instruction.number as isize
                                        - (entry.0 .0 as isize - instruction.number as isize))
                                        .try_into()
                                        .unwrap(),
                                    entry.0 .1,
                                ))
                                .or_insert(1) = 1;
                        }
                    }
                }
            } else {
                for entry in data.0.clone() {
                    if entry.0 .1 > instruction.number {
                        if data.0[&entry.0] != 0 {
                            *data.0.entry(entry.0).or_insert(0) = 0;
                            *data
                                .0
                                .entry((
                                    entry.0 .0,
                                    (instruction.number as isize
                                        - (entry.0 .1 as isize - instruction.number as isize))
                                        .try_into()
                                        .unwrap(),
                                ))
                                .or_insert(1) = 1;
                        }
                    }
                }
            }
        }

        for y in 0..6 {
            for x in 0..40 {
                if *data.0.entry((x, y)).or_insert(0) == 0 {
                    print!(" ");
                } else {
                    print!("X");
                }
            }
            println!("")
        }

        0
    }
}
