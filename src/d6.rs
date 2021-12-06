use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 6;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

type Data = Vec<u64>;
impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init() -> (Self, Data) {
        let v: Data = std::fs::read_to_string(FILE)
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 {
        for _ in 0..80 {
            let mut new = Data::new();
            for d in &mut *data {
                if *d == 0 {
                    *d = 6;
                    new.push(8);
                } else {
                    *d -= 1;
                }
            }
            data.append(&mut new);
        }

        data.len() as u64
    }
    fn two(&self, data: &mut Data) -> u64 { 
        let mut d = [0; 9]; // 0, 1, 2, 3, 4, 5, 6, 7, 8
        for entry in data {
            d[*entry as usize] += 1;
        }
        for _ in 0..256 {
            let mut buffer = [0; 9];
            
            buffer[8] = d[0];
            buffer[7] = d[8];
            buffer[6] = d[7] + d[0];
            buffer[5] = d[6];
            buffer[4] = d[5];
            buffer[3] = d[4];
            buffer[2] = d[3];
            buffer[1] = d[2];
            buffer[0] = d[1];

            for i in 0..9 {
                d[i] = buffer[i];
            }
        }

        d.iter().fold(0, |sum, f| sum + f)
     }
}


