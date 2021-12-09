use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 9;
const FILE: &'static str = formatcp!("./inputs/input{}test.txt", CURRENT_DAY);

const WIDTH: usize = 10;
const HEIGHT: usize = 5;

// const WIDTH: usize = 100;
// const HEIGHT: usize = 100;

const SIZE: usize = WIDTH * HEIGHT;

type Data = [u8; SIZE as usize];

fn is_lowest(data: &Data, x: usize, y: usize) -> bool {
    let mut lowest: bool; // RAII is unhappy now :(
    if x == 0 {
        lowest = data[(y * WIDTH) as usize] < data[(y * WIDTH + 1) as usize];
    } else if x == WIDTH - 1 {
        lowest = data[(y * WIDTH + x) as usize] < data[(y * WIDTH + x - 1) as usize];
    } else {
        lowest = data[(y * WIDTH + x) as usize] < data[(y * WIDTH + x + 1) as usize]
            && data[(y * WIDTH + x) as usize] < data[(y * WIDTH + x - 1) as usize];
    }
    if !lowest {
        return false;
    }
    if y == 0 {
        lowest = lowest && data[(x) as usize] < data[(WIDTH + x) as usize];
    } else if y == HEIGHT - 1 {
        lowest = lowest && data[(y * WIDTH + x) as usize] < data[((y - 1) * WIDTH + x) as usize];
    } else {
        lowest = lowest
            && data[(y * WIDTH + x) as usize] < data[((y + 1)*WIDTH + x) as usize]
            && data[(y * WIDTH + x) as usize] < data[((y - 1) * WIDTH + x) as usize];
    }
    lowest
}

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
    fn one(&self, data: &mut Data) -> u64 {
        let mut sum:u64 = 0;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let low = is_lowest(&data, x, y);
                println!("[{}|{}]: {} -> {}", x, y, data[(y*WIDTH+x) as usize], low);
                if low {
                    sum += data[(y*WIDTH+x) as usize] as u64 + 1;
                }
            }
        }
        sum
    }
    fn two(&self, _data: &mut Data) -> u64 {
        todo!()
    }
}
