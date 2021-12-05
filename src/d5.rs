use std::convert::TryInto;

use crate::{Day, Runnable};
use const_format::formatcp;

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
        Self {
            start_pos,
            end_pos,
            k: {
                let x = end_pos.0 - start_pos.0;
                let y = end_pos.1 - start_pos.1;

                // This works because of the fact, that the lines only are
                // horizontal, vertical, or diagonal
                // If they allowed ks like (1, 2), you'd need to use GCD
                let divisor = std::cmp::max(x.abs(), y.abs());

                (x / divisor, y / divisor)
            },
        }
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
                let start: Vec<i16> = splitted[0]
                    .split(',')
                    .map(|num| num.parse().unwrap())
                    .collect();
                let end: Vec<i16> = splitted[1]
                    .split(',')
                    .map(|num| num.parse().unwrap())
                    .collect();

                Line::from((start[0], start[1]), (end[0], end[1]))
            })
            .collect();

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 {
        let mut positions: [u8; 1000 * 1000] = [0; 1000 * 1000];

        data.iter()
            .filter(|line| line.k.0 == 0 || line.k.1 == 0)
            .for_each(|line| {
                let k = &line.k;

                let mut pos = (line.start_pos.0, line.start_pos.1);
                loop {
                    positions[(pos.1 as usize) * 1000 + pos.0 as usize] += 1;

                    if pos == line.end_pos {
                        break;
                    }

                    pos.0 += k.0;
                    pos.1 += k.1;
                }
            });

        positions
            .iter()
            .filter(|f| f >= &&2)
            .count()
            .try_into()
            .unwrap()

        // let mut positions: HashMap<Position, u32> = HashMap::new();
        // data.iter()
        //     .filter(|line| line.k.0 == 0 || line.k.1 == 0)
        //     .for_each(|line| {
        //         let k = &line.k;

        //         let mut pos = (line.start_pos.0, line.start_pos.1);
        //         loop {
        //             *positions.entry(pos).or_insert(0) += 1;

        //             if pos == line.end_pos {
        //                 break;
        //             }

        //             pos.0 += k.0;
        //             pos.1 += k.1;
        //         }
        //     });

        // positions
        //     .iter()
        //     .filter(|(_, f)| f >= &&2)
        //     .count()
        //     .try_into()
        //     .unwrap()
    }
    fn two(&self, data: &mut Data) -> u64 {
        let mut positions: [u8; 1000 * 1000] = [0; 1000 * 1000];

        data.iter().for_each(|line| {
            let k = &line.k;

            let mut pos = (line.start_pos.0, line.start_pos.1);
            loop {
                positions[(pos.1 as usize) * 1000 + pos.0 as usize] += 1;

                if pos == line.end_pos {
                    break;
                }

                pos.0 += k.0;
                pos.1 += k.1;
            }
        });

        positions
            .iter()
            .filter(|f| f >= &&2)
            .count()
            .try_into()
            .unwrap()

        // let mut positions: HashMap<Position, u8> = HashMap::new();
        // data.iter().for_each(|line| {
        //     let k = &line.k;

        //     let mut pos = (line.start_pos.0, line.start_pos.1);
        //     loop {
        //         *positions.entry(pos).or_insert(0) += 1;

        //         if pos == line.end_pos {
        //             break;
        //         }

        //         pos.0 += k.0;
        //         pos.1 += k.1;
        //     }
        // });

        // positions
        //     .iter()
        //     .filter(|(_, f)| f >= &&2)
        //     .count()
        //     .try_into()
        //     .unwrap()
    }
}
