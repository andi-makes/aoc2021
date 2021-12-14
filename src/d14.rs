use std::collections::HashMap;

use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 14;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

type Data = (HashMap<String, u64>, HashMap<String, char>);
impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init(_input: &str) -> (Self, Data) {
        let mut v: Data = (HashMap::new(), HashMap::new());
        std::fs::read_to_string(FILE)
            .unwrap()
            .lines().enumerate()
            .for_each(|(i,s)| {
                if i == 0 {
                    for i in 0..s.len()-1 {
                        *v.0.entry(s[i..(i+2)].to_string()).or_insert(0) += 1;
                    }
                }
                if i >= 2 {
                    v.1.insert(s[0..2].to_string(), s[6..7].chars().next().unwrap());
                }
            });

        // println!("{:?}", v);

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 {
        // println!("INITIAL DATA: \n{:?}", data.0);
        for step in 0..10 {
            // println!("STEP {}:", step+1);
            let mut working = HashMap::new();
            for (key, entry) in &data.0 {  
                for (pattern, insert) in &data.1 {
                    if key == pattern {
                        let p1 = format!("{}{}", pattern[0..1].to_string(), insert);
                        let p2 = format!("{}{}", insert, pattern[1..2].to_string());
                        *working.entry(p1.clone()).or_insert(0) += entry;
                        *working.entry(p2.clone()).or_insert(0) += entry;
                        // break;
                    }
                }
            }
            data.0 = working.clone();
            // println!("{:?}", data.0);
        }
 
        let mut amount: HashMap<char, u64> = HashMap::new();

        for (key, entry) in &data.0 {
            *amount.entry(key[1..2].chars().next().unwrap()).or_insert(0) += entry;
        }

        let mut num: Vec<u64> = amount.clone().into_values().collect();
        num.sort();
        // println!("{:?}", num);
        num.last().unwrap() - num.first().unwrap()
    }
    fn two(&self, data: &mut Data) -> u64 { 
        // println!("INITIAL DATA: \n{:?}", data.0);
        for step in 0..40 {
            // println!("STEP {}:", step+1);
            let mut working = HashMap::new();
            for (key, entry) in &data.0 {  
                for (pattern, insert) in &data.1 {
                    if key == pattern {
                        let p1 = format!("{}{}", pattern[0..1].to_string(), insert);
                        let p2 = format!("{}{}", insert, pattern[1..2].to_string());
                        *working.entry(p1.clone()).or_insert(0) += entry;
                        *working.entry(p2.clone()).or_insert(0) += entry;
                        // break;
                    }
                }
            }
            data.0 = working.clone();
            // println!("{:?}", data.0);
        }
 
        let mut amount: HashMap<char, u64> = HashMap::new();

        for (key, entry) in &data.0 {
            *amount.entry(key[1..2].chars().next().unwrap()).or_insert(0) += entry;
        }

        let mut num: Vec<u64> = amount.clone().into_values().collect();
        num.sort();
        // println!("{:?}", num);
        num.last().unwrap() - num.first().unwrap()
     }
}


