use std::convert::TryInto;

use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 2;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

#[derive(Clone, Debug)]
pub enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Clone, Debug)]
pub struct DataPoint {
    dir: Direction,
    amount: i64,
}

impl From<String> for DataPoint {
    fn from(s: String) -> Self {
        if s.is_empty() {
            return DataPoint{dir: Direction::Forward, amount: 0};
        } 
        let mut s = s.split(' ');
        if s.clone().count() != 2 {
            return DataPoint{dir: Direction::Forward, amount: 0};
        }
        let dir = match s.next().unwrap() {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => return DataPoint{dir: Direction::Forward, amount: 0},
        };
        let amount = s.next().unwrap().parse().unwrap();

        DataPoint{dir, amount}
    }
}

type Data = Vec<DataPoint>;
impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init(_input: &str) -> (Self, Data) {
        let v: Data = std::fs::read_to_string(FILE)
            .unwrap()
            .lines()
            .map(|s| {DataPoint::from(s.to_string())})
            .collect();

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 {
        let mut horizontal = 0;
        let mut vertical = 0;

        for point in data {
            match point.dir {
                Direction::Forward => horizontal += point.amount,
                Direction::Up => vertical -= point.amount,
                Direction::Down => vertical += point.amount,
            }
        }
        (horizontal*vertical).try_into().unwrap()
    }
    fn two(&self, data: &mut Data) -> u64 {
        let mut horizontal = 0;
        let mut vertical = 0;
        let mut aim = 0;

        for point in data {
            match point.dir {
                Direction::Forward => {
                    horizontal += point.amount;
                    vertical += point.amount * aim;
                },
                Direction::Up => aim-=point.amount,
                Direction::Down => aim+=point.amount,
            }
        }
        (horizontal*vertical).try_into().unwrap() // 385593268
    }
}


