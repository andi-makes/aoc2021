use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 9;
const FILE: &'static str = formatcp!("./inputs/input{}test.txt", CURRENT_DAY);

const WIDTH: u8 = 10;
const HEIGHT: u8 = 5;

const SIZE: u8 = WIDTH * HEIGHT;

type Data = [u8; SIZE as usize];
impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init(_input: &str) -> (Self, Data) {
        let mut v: Data = [0; SIZE as usize];
        std::fs::read_to_string(FILE)
            .unwrap()
            .lines()
            .enumerate()
            .for_each(|(y, f)| {
                f.split("")
                    .filter(|f| !f.is_empty())
                    .enumerate()
                    .for_each(|(x, g)| {
                        println!("{}|{}", x, y);
                        v[(y * WIDTH as usize + x) as usize] = g.parse().unwrap();
                    });
            });

        (Self {}, v)
    }
    fn one(&self, _data: &mut Data) -> u64 {
        todo!()
    }
    fn two(&self, _data: &mut Data) -> u64 {
        todo!()
    }
}
