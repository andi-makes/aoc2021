use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 3;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);
type Data = Vec<u32>;

fn get_bit(number: u32, bit: u32) -> u32 {
    match number & (1 << bit) {
        0 => 0,
        _ => 1,
    }
}

fn sum_ones_in_column(data: &Data, col: u32) -> u32 {
    // .fold() basically sums the amount of 1s up
    data.iter().fold(0, |sum, d| sum + get_bit(*d, col))
}

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
                number
            })
            .collect();

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 {
        let data: &Data = data; // Remove mut
        let mut gamma: u32 = 0;
        let length = data.len() as u32;

        // For each "column" in our data
        for i in 0..12 {
            let count = sum_ones_in_column(data, i);
            // If we have more ones than zeros, set the corresponding bit in
            // gamma
            if 2 * count > length {
                gamma |= 1 << i;
            }
        }
        // We get epsilon by bitwise inverting gamma.
        // We must only invert the used bits, otherwise the number is wrong.
        // println!(
        //     "y: {}, e: {} => {}",
        //     gamma,
        //     gamma ^ 0xFFF,
        //     gamma * (gamma ^ 0xFFF)
        // );
        (gamma * (gamma ^ 0xFFF)) as u64
    }
    fn two(&self, data: &mut Data) -> u64 {
        let mut o2 = data.clone();
        let mut co2 = data.clone();
        for i in (0..12).rev() {
            // Calculate o2 data
            {
                let remaining = o2.len() as u32;
                let count = sum_ones_in_column(&o2, i);
                let delete = if 2 * count >= remaining { 0 } else { 1 };
                if remaining != 1 {
                    o2 = o2
                        .iter()
                        .filter(|d| get_bit(**d, i) != delete)
                        .copied()
                        .collect();
                }
            }

            // Calculate co2 data
            {
                let remaining = co2.len() as u32;
                let count = sum_ones_in_column(&co2, i);
                let delete = if 2 * count >= remaining { 0 } else { 1 };
                if remaining != 1 {
                    co2 = co2
                        .iter()
                        .filter(|d| get_bit(**d, i) == delete)
                        .copied()
                        .collect();
                }
            }
        }

        let o2 = o2[0];
        let co2 = co2[0];

        //println!("o2: {}, co2: {} => {}", o2, co2, o2 * co2);
        (o2 * co2).into()
    }
}
