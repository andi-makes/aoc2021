use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 9;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

// const WIDTH: usize = 10;
// const HEIGHT: usize = 5;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

const SIZE: usize = WIDTH * HEIGHT;

type Data = [u8; SIZE];

type Visited = [bool; SIZE];

fn is_lowest(data: &Data, x: usize, y: usize) -> bool {
    let mut lowest: bool; // RAII is unhappy now :(
    if x == 0 {
        lowest = data[y * WIDTH] < data[y * WIDTH + 1];
    } else if x == WIDTH - 1 {
        lowest = data[y * WIDTH + x] < data[y * WIDTH + x - 1];
    } else {
        lowest = data[y * WIDTH + x] < data[y * WIDTH + x + 1]
            && data[y * WIDTH + x] < data[y * WIDTH + x - 1];
    }
    if !lowest {
        return false;
    }
    if y == 0 {
        lowest = lowest && data[x] < data[WIDTH + x];
    } else if y == HEIGHT - 1 {
        lowest = lowest && data[y * WIDTH + x] < data[(y - 1) * WIDTH + x];
    } else {
        lowest = lowest
            && data[y * WIDTH + x] < data[(y + 1) * WIDTH + x]
            && data[y * WIDTH + x] < data[(y - 1) * WIDTH + x];
    }
    lowest
}

fn get_basin_size(data: &Data, x: usize, y: usize, visited: &mut Visited) -> usize {
    if data[y * WIDTH + x] == 9 {
        return 0;
    }

    let mut size = 1;
    visited[y*WIDTH+x] = true;

    if x == 0 {
        if visited[y*WIDTH+x+1] == false {
            // println!("{}", data[y*WIDTH+x+1]);
            size += get_basin_size(data, x+1, y, visited);
        }
    } else if x == WIDTH - 1 {
        if visited[y*WIDTH+x-1] == false {
            // println!("{}", data[y*WIDTH+x-1]);
            size += get_basin_size(data, x-1, y, visited);
        }
    } else {
        if visited[y*WIDTH+x+1] == false {
            // println!("{}", data[y*WIDTH+x+1]);
            size += get_basin_size(data, x+1, y, visited);
        }
        if visited[y*WIDTH+x-1] == false {
            // println!("{}", data[y*WIDTH+x-1]);
            size += get_basin_size(data, x-1, y, visited);
        }
    }
    if y == 0 {
        if visited[(y+1)*WIDTH+x] == false {
            // println!("{}", data[(y+1)*WIDTH+x]);
            size += get_basin_size(data, x, y+1, visited);
        }
    } else if y == HEIGHT - 1 {
        if visited[(y-1)*WIDTH+x] == false {
            // println!("{}", data[(y-1)*WIDTH+x]);
            size += get_basin_size(data, x, y-1, visited);
        }
    } else {
        if visited[(y+1)*WIDTH+x] == false {
            // println!("{}", data[(y+1)*WIDTH+x]);
            size += get_basin_size(data, x, y+1, visited);
        }
        if visited[(y-1)*WIDTH+x] == false {
            // println!("{}", data[(y-1)*WIDTH+x]);
            size += get_basin_size(data, x, y-1, visited);
        }
    }

    return size;
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
                        // println!("{}|{}", x, y);
                        v[y * WIDTH + x] = g.parse().unwrap();
                    });
            });

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 {
        let mut sum: u64 = 0;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let low = is_lowest(&data, x, y);
                // println!(
                //     "[{}|{}]: {} -> {}",
                //     x,
                //     y,
                //     data[y * WIDTH + x],
                //     low
                // );
                if low {
                    sum += data[y * WIDTH + x] as u64 + 1;
                }
            }
        }
        sum
    }
    fn two(&self, data: &mut Data) -> u64 {
        let mut low_point: Vec<(usize, usize)> = Vec::new();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let low = is_lowest(&data, x, y);
                // println!(
                //     "[{}|{}]: {} -> {}",
                //     x,
                //     y,
                //     data[y * WIDTH + x],
                //     low
                // );
                if low {
                    low_point.push((x, y));
                }
            }
        }

        let mut basin_size = Vec::new();
        let mut visited = [false; SIZE];
        for p in low_point {
            let size = get_basin_size(data, p.0, p.1, &mut visited);
            // println!("[{}|{}]: {}", p.0, p.1, size);
            basin_size.push(size as u64);
        }
        basin_size.sort();
        // println!("{:?}", basin_size);
        basin_size.pop().unwrap() * basin_size.pop().unwrap() * basin_size.pop().unwrap()
    }
}
