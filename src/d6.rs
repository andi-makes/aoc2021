use crate::{Day, Runnable};
// use const_format::formatcp;

const CURRENT_DAY: u8 = 6;
// const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

type Data = [u64; 9];

fn evolve(d: &mut Data, steps: u64) -> u64 {
    for _ in 0..steps {
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

        // for i in 0..9 {
        //     match i {
        //         8 => buffer[8] = d[0],
        //         6 => buffer[6] = d[7] + d[0],
        //         _ => buffer[i] = d[i+1],
        //     }
        // }

        for i in 0..9 {
            d[i] = buffer[i];
        }
    }

    d.iter().fold(0, |sum, f| sum + f)
}

impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init(input: &str) -> (Self, Data) {
        let mut v: Data = [0;9];
        input
            .trim()
            .split(',')
            .for_each(|elem| v[elem.parse::<usize>().unwrap()] += 1);

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 {
        evolve(data, 80)
    }
    fn two(&self, data: &mut Data) -> u64 {
        evolve(data, 256)
    }
}
