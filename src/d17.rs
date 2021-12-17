use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 17;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

type Data = Vec<i64>;

// fn valid_vx(min: i64, max: i64) -> Vec<(i64, i64, i64)> {
//     let mut vec = Vec::new();
//     for pos in (min..max + 1).rev() {
//         for vel in (0..pos + 1).rev() {
//             let mut sum = vel;
//             for step in (0..vel).rev() {
//                 if sum == pos {
//                     vec.push((pos, vel, vel - step));
//                     break;
//                 } else {
//                     sum += step;
//                 }
//             }
//         }
//     }
//     vec
// }

// fn valid_vy(vx: i64, steps: i64, min: i64, max: i64) -> i64 {
//     // That means, that x won't change it's x position after even more steps.
//     // if vx == steps {
//     //     return -min;
//     // }
//     // if steps == 1 {
//     //     // Only one step, well, there's only one possibility for each y
//     //     return -min + max + 1;
//     // }
//     // Possible vy values:

//     let mut count = 0;

//     for vel in min..-min {
//         let mut py = 0;
//         println!("Try: {}", vel);
//         let mut dy = vel;
//         for ii in 1..steps {
//             dy -= 1;
//             py -= dy;
//             print!("{} -> ", py);
//         }
//         if py >= min && py <= max {
//             println!("\t{}", py);
//             count += 1;
//         }
//     }

//     // println!("Count: {}", count);

//     count
// }

impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init(_input: &str) -> (Self, Data) {
        // I've allowed myself to modify the data input a little bit:
        // Line 0: x range begin
        // Line 1: x range end
        // Line 2: y range begin
        // Line 3: y range end
        let v: Data = std::fs::read_to_string(FILE)
            .unwrap()
            .lines()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 {
        (1..(-data[2])).fold(0, |sum, val| sum + val) as u64
    }
    fn two(&self, data: &mut Data) -> u64 {
        // First, I tried to do a pretty solution. But that didn't work out.
        // println!("Valid vx: \n{:?}", valid_vx(data[0], data[1]));
        // println!("Valid vy: \n{:?}", valid_vy(11, 3, data[2], data[3]));

        // valid_vx(data[0], data[1]).iter().fold(0, |sum, elem|  {
        //     sum + valid_vy(elem.1, elem.2, data[2], data[3])
        // }) as u64
        // 0

        // So then I thought: Fuck pretty, let's do a bruteforce

        let mut count = 0;
        for vx in 0..data[1]+1 {
            for vy in data[2]..-data[2] {
                let mut mod_vx = vx;
                let mut mod_vy = vy;
                let mut x = 0;
                let mut y = 0;
                loop {
                    x += mod_vx;
                    y += mod_vy;
                    if mod_vx > 1 {
                        mod_vx -= 1;
                    } else {
                        mod_vx = 0;
                    }
                    mod_vy -= 1;

                    if x > data[1] || y < data[2] {
                        break;
                    }
                    if x >= data[0] && x <= data[1] && y >= data[2] && y <= data[3] {
                        count += 1;
                        break;
                    }
                }
            }
        }
        // Too high: 1921
        count // Right answer: 1908
    }
}
