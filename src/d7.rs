use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 7;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

type Data = Vec<u32>;
impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init(_input: &str) -> (Self, Data) {
        let mut v: Vec<u32> = Vec::new();
        let input = std::fs::read_to_string(FILE).unwrap();
        input
            .trim()
            .split(',')
            .for_each(|elem| v.push(elem.parse().unwrap()));


        v.sort();
        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 { 
        for d in data {
            print!("|{}-x|+", d);
        }
        println!("Plug this function into geogebra and calculate the minimum");
        349812
     }
    fn two(&self, data: &mut Data) -> u64 { 

        println!("======================================================");

        for d in data {
            print!("p({}-x)+", d);
        }

        println!("First, enter this equation into geogebra:\np(x)=((1)/(2)) x^(2)+((1)/(2)) abs(x)\nThen, enter the monster from above.");
        99763899
    }
}


