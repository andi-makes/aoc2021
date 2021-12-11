use crate::{Day, Runnable};
use colored::Colorize;
use const_format::formatcp;

const CURRENT_DAY: u8 = 11;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

type Data = [u8; 10 * 10];

fn pretty_print_data(data: &Data) {
    for y in 0..10 {
        for x in 0..10 {
            let d = if data[x + y * 10] == 0 {
                "0".red().to_string()
            } else {
                data[x + y * 10].to_string()
            };
            print!("{}", d)
        }
        println!("");
    }
}

/// Returns the index, the x and the y for the 1D Array for the neighbour
fn get_other(x: usize, y: usize, dx: isize, dy: isize) -> (usize, usize, usize) {
    let x = x as isize;
    let y = y as isize;

    // println!("{} + {} = {}", x, dx, x+dx);
    let nx = if x + dx < 0 {
        0xFF
    } else if x + dx >= 10 {
        0xFF
    } else {
        x + dx
    };

    let ny = if y + dy < 0 {
        0xFF
    } else if y + dy >= 10 {
        0xFF
    } else {
        y + dy
    };

    ((nx + ny * 10) as usize, nx as usize, ny as usize)
}

fn glow(sum: usize, data: &mut Data, x: usize, y: usize) -> usize{
    let mut sum = sum;
    if data[x + 10 * y] > 9 && data[x + 10 * y] != 0xFF {
        // println!("x:{}, y:{}", x, y);
        // data[x+10*y] += 1;
        data[x + 10 * y] = 0xFF;
        sum += 1;
        for dy in -1..2 {
            for dx in -1..2 {
                // println!("dx: {}, dy: {}", dx, dy);
                let pos = get_other(x, y, dx, dy);
                if pos.1 == 0xFF || pos.2 == 0xFF {
                    continue;
                }
                if (pos.1 == x && pos.2 == y) || data[pos.0] == 0xFF {
                    continue;
                }
                data[pos.0] += 1;

                sum += glow(0, data, pos.1, pos.2);
            }
        }
    }
    sum
}

impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init(_input: &str) -> (Self, Data) {
        let mut v: Data = [0; 10 * 10];

        std::fs::read_to_string(FILE)
            .unwrap()
            .lines()
            .enumerate()
            .for_each(|(y, s)| {
                s.chars().enumerate().for_each(|(x, c)| {
                    v[x + 10 * y] = c.to_string().parse().unwrap();
                })
            });

        // println!("{:?}", v);

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 {
        let mut sum = 0;
        for step in 0..100 {
            // println!("==========\nSTEP: {}", step + 1);
            for y in 0..10 {
                for x in 0..10 {
                    data[x + y * 10] += 1;
                }
            }

            // println!("==========");
            // pretty_print_data(data);

            for y in 0..10 {
                for x in 0..10 {
                    sum = glow(sum, data, x, y);
                }
            }

            // println!("==========");
            // pretty_print_data(data);

            for y in 0..10 {
                for x in 0..10 {
                    if data[x + 10 * y] == 0xFF {
                        data[x + 10 * y] = 0;
                    }
                }
            }
            // println!("==========");
            // pretty_print_data(data);
        }
        // println!("==========");
        // pretty_print_data(data);
        sum as u64
    }
    fn two(&self, data: &mut Data) -> u64 {
        let mut step = 0;
        loop {
            step += 1;
            // println!("==========\nSTEP: {}", step + 1);
            for y in 0..10 {
                for x in 0..10 {
                    data[x + y * 10] += 1;
                }
            }

            // println!("==========");
            // pretty_print_data(data);

            for y in 0..10 {
                for x in 0..10 {
                    glow(0, data, x, y);
                }
            }

            // println!("==========");
            // pretty_print_data(data);

            let mut flashes = 0;
            for y in 0..10 {
                for x in 0..10 {
                    if data[x + 10 * y] == 0xFF {
                        data[x + 10 * y] = 0;
                        flashes+=1;
                    }
                }
            }
            if flashes == 100 {
                break;
            }
            // println!("==========");
            // pretty_print_data(data);
        }
        // println!("==========");
        // pretty_print_data(data);
        step as u64
    }
}
