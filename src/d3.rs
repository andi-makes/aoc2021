use std::convert::TryInto;

use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 3;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

type Data = Vec<u32>;
impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init() -> (Self, Data) {
        let v: Data = std::fs::read_to_string(FILE)
            .unwrap()
            .lines()
            .map(|s| {
                let mut number = 0;
                let mut copy = s.clone().chars();
                for i in 0..(s.len()) {
                    number |= match copy.next().unwrap() {
                        '0' => 0,
                        '1' => 1,
                        _ => panic!("BLARGH"),
                    } << (s.len() - 1 - i);
                }
                // println!("{}: {}", s, number);
                number
            })
            .collect();

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) {
        let mut gamma: u32 = 0;
        let length: u32 = data.len().try_into().unwrap();
        println!("{}", length);
        for i in 0..12 {
            // Dumb dumb thing, there has to be a better way
            let mut count = 0;
            for number in data.into_iter() {
                count += match *number & (1 << i) {
                    0 => 0,
                    _ => 1,
                };
            }
            if count > length / 2 {
                gamma |= 1 << i;
            }
        }

        println!(
            "y: {}, e: {} => {}",
            gamma,
            (!gamma) & 0xFFF,
            gamma * ((!gamma) & 0xFFF)
        );
    }
    fn two(&self, data: &mut Data) {
        let mut o2_data = data.clone();
        let mut remaining = o2_data.len();
        for i in (0..12).rev() {
            // Dumb dumb thing, there has to be a better way
            let mut count = 0;
            for j in 0..o2_data.len() {
                if o2_data[j] == 0 {
                    continue;
                }
                count += match o2_data[j] & (1 << i) {
                    0 => 0,
                    _ => 1,
                };
            }
            if 2*count >= remaining {
                for k in 0..o2_data.len() {
                    if o2_data[k] == 0 {
                        continue;
                    }
                    if o2_data[k] & (1 << i) == 0 {
                        o2_data[k] = 0;
                        remaining -= 1;
                    }
                }
            } else {
                for k in 0..o2_data.len() {
                    if o2_data[k] == 0 {
                        continue;
                    }
                    if match o2_data[k] & (1 << i) {
                        0 => 0,
                        _ => 1,
                    } == 1 {
                        o2_data[k] = 0;
                        remaining -= 1;
                    }
                }
            }
        }

        let mut co2 = data.clone();
        let mut remaining = co2.len();
        for i in (0..12).rev() {
            if remaining == 1 {
                break;
            }
            // Dumb dumb thing, there has to be a better way
            let mut count = 0;
            for j in 0..co2.len() {
                if co2[j] == 0 {
                    continue;
                }
                count += match co2[j] & (1 << i) {
                    0 => 0,
                    _ => 1,
                };
            }
            if 2*count >= remaining {
                for k in 0..co2.len() {
                    if co2[k] == 0 {
                        continue;
                    }
                    if match co2[k] & (1 << i) {
                        0 => 0,
                        _ => 1,
                    } == 1 {
                        co2[k] = 0;
                        remaining -= 1;
                    }
                }
            } else {
                for k in 0..co2.len() {
                    if co2[k] == 0 {
                        continue;
                    }
                    if co2[k] & (1 << i) == 0 {
                        co2[k] = 0;
                        remaining -= 1;
                    }
                }
            }
        }

        // Yeah, this solution is ugly as it is, multiply the two numbers
        // yourself
        for d in o2_data.into_iter() {
            if d == 0 {
                continue;
            }
            println!("{}", d);
        }

        for d in co2.into_iter() {
            if d == 0 {
                continue;
            }
            println!("{}", d);
        }
    }
}
