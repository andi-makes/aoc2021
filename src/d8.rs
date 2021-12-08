use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 8;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

#[derive(Clone, Debug)]
pub struct SegmentData {
    pre: Vec<String>,
    post: Vec<String>,
}

type Data = Vec<SegmentData>;
impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init(_input: &str) -> (Self, Data) {
        let v: Data = std::fs::read_to_string(FILE)
            .unwrap()
            .lines()
            .map(|s| {
                let mut split = s.split(" | ");
                SegmentData {
                    pre: split
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .map(|f| f.to_string())
                        .collect(),
                    post: split
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .map(|f| f.to_string())
                        .collect(),
                }
            })
            .collect();

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 {
        data.iter().fold(0, |sum, d| {
            sum + d
                .post
                .iter()
                .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
                .collect::<Vec<&String>>()
                .len() as u64
        })
    }
    fn two(&self, _data: &mut Data) -> u64 {
        todo!()
    }
}
