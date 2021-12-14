use std::collections::HashMap;

use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 14;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

type Data = (HashMap<[char; 2], u64>, Vec<(Vec<char>, char)>);
impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init(_input: &str) -> (Self, Data) {
        let mut v: Data = (HashMap::new(), Vec::new());
        std::fs::read_to_string(FILE)
            .unwrap()
            .lines().enumerate()
            .for_each(|(i,s)| {
                if i == 0 {
                    let chars: Vec<char> = s.chars().collect();
                    for i in 0..chars.len()-1 {
                        v.0.insert([chars[i], chars[i+1]], 1);
                    }
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
            for (key, entry) in data.0.clone() {  
                for (pattern, insert) in &data.1 {
                    if key == [pattern[0], pattern[1]] && entry != 0 {
                        *data.0.entry([pattern[0], pattern[1]]).or_insert(0) -= entry;
                        *data.0.entry([pattern[0], *insert]).or_insert(0) += entry;
                        *data.0.entry([*insert, pattern[1]]).or_insert(0) += entry;
                        // break;
                    }
                }
            }
            // println!("{:?}", data.0);
        }
 
        // data.0.sort();
        // let mut numbers = Vec::new();
        // let mut old_char = '0';
        let mut amount: HashMap<char, u64> = HashMap::new();

        for (key, entry) in &data.0 {
            //*amount.entry(key[0]).or_insert(0) += entry;
            *amount.entry(key[1]).or_insert(0) += entry;
        }

        let mut num: Vec<u64> = amount.clone().into_values().collect();
        num.sort();
        // println!("{:?}", num);
        num.last().unwrap() - num.first().unwrap()
    }
    fn two(&self, data: &mut Data) -> u64 { 
        // for step in 0..40 {
        //     let mut inserted = 0;
        //     for i in 0..data.0.len()-1 {  
        //         for (pattern, insert) in &data.1 {
        //             if data.0[i+inserted] == pattern[0] && data.0[i+1+inserted] == pattern[1] {
        //                 data.0.insert(i+1+inserted, *insert);
        //                 inserted += 1;
        //                 break;
        //             }
        //         }
        //     }
        //     // println!("{:?}", data.0);
        // }

        // data.0.sort();
        // let mut numbers = Vec::new();
        // let mut old_char = '0';
        // for c in &data.0 {
        //     if old_char != *c {
        //         old_char = *c;
        //         numbers.push(1);
        //     } else {
        //         let index = numbers.len()-1;
        //         numbers[index] += 1;
        //     }
        // }

        // numbers.sort();
        // println!("{:?}", numbers);

        // (numbers.last().unwrap() - numbers.first().unwrap()) as u64
        0
     }
}


