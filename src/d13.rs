use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 13;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

type Data = Vec<String>;
impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init(_input: &str) -> (Self, Data) {
        let v: Data = std::fs::read_to_string(FILE)
            .unwrap()
            .lines()
            .map(|s| s.to_string())
            .collect();

        (Self {}, v)
    }
    fn one(&self, _data: &mut Data) -> u64 { todo!() }
    fn two(&self, _data: &mut Data) -> u64 { todo!() }
}


