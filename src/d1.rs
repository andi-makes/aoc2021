use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 1;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

type Data = Vec<u32>;
impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init(_input: &str) -> (Self, Data) {
        let v: Data = std::fs::read_to_string(FILE)
            .unwrap()
            .lines()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 {
        let mut increased = 0;
        for i in 1..data.len() {
            if data[i] > data[i-1] {
                increased += 1;
            }
        }
        increased
    }
    fn two(&self, data: &mut Data) -> u64 {
        let mut incresed = 0;
        for i in 3..(data.len()) {
            if (data[i] + data[i-1] + data[i-2]) > (data[i-1] + data[i-2] + data[i-3]) {
                incresed += 1;
            }
        }
        incresed // 1851, 1852
    }
}
