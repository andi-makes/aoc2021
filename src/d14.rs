use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 14;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

type Data = (Vec<char>, Vec<(Vec<char>, char)>);
impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init(_input: &str) -> (Self, Data) {
        let mut v: Data = (Vec::new(), Vec::new());
        std::fs::read_to_string(FILE)
            .unwrap()
            .lines().enumerate()
            .for_each(|(i,s)| {
                if i == 0 {
                    v.0 = s.chars().collect();
                }
                if i >= 2 {
                    let split: Vec<&str> = s.split(" -> ").collect();
                    v.1.push((split[0].chars().collect(), split[1].chars().next().unwrap()));
                }
            });

        println!("{:?}", v);

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 {
        for step in 0..10 {
            let mut inserted = 0;
            for i in 0..data.0.len()-1 {  
                for (pattern, insert) in &data.1 {
                    if data.0[i+inserted] == pattern[0] && data.0[i+1+inserted] == pattern[1] {
                        data.0.insert(i+1+inserted, *insert);
                        inserted += 1;
                        break;
                    }
                }
            }
            // println!("{:?}", data.0);
        }

        data.0.sort();
        let mut numbers = Vec::new();
        let mut old_char = '0';
        for c in &data.0 {
            if old_char != *c {
                old_char = *c;
                numbers.push(1);
            } else {
                let index = numbers.len()-1;
                numbers[index] += 1;
            }
        }

        numbers.sort();
        println!("{:?}", numbers);

        (numbers.last().unwrap() - numbers.first().unwrap()) as u64
    }
    fn two(&self, data: &mut Data) -> u64 { 
        for step in 0..40 {
            let mut inserted = 0;
            for i in 0..data.0.len()-1 {  
                for (pattern, insert) in &data.1 {
                    if data.0[i+inserted] == pattern[0] && data.0[i+1+inserted] == pattern[1] {
                        data.0.insert(i+1+inserted, *insert);
                        inserted += 1;
                        break;
                    }
                }
            }
            // println!("{:?}", data.0);
        }

        data.0.sort();
        let mut numbers = Vec::new();
        let mut old_char = '0';
        for c in &data.0 {
            if old_char != *c {
                old_char = *c;
                numbers.push(1);
            } else {
                let index = numbers.len()-1;
                numbers[index] += 1;
            }
        }

        numbers.sort();
        println!("{:?}", numbers);

        (numbers.last().unwrap() - numbers.first().unwrap()) as u64
     }
}


