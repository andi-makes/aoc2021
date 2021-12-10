use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 10;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

type Data = Vec<String>;
impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init(_input: &str) -> (Self, Data) {
        let v: Data = std::fs::read_to_string(FILE)
            .unwrap()
            .lines()
            .map(|s| s.to_string())
            .collect();

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 {
        let mut count_round: u64 = 0;
        let mut count_brackets: u64 = 0;
        let mut count_curly: u64 = 0;
        let mut count_pointy: u64 = 0;

        // This solution was partly brought to you by:
        // https://www.geeksforgeeks.org/check-for-balanced-parentheses-in-an-expression/
        for line in data {
            let mut line_buffer = Vec::new();
            for char in line.chars() {
                match char {
                    '(' => line_buffer.push('('),
                    ')' => {
                        let popped = line_buffer.pop().unwrap();
                        if popped != '(' {
                            count_round += 1;
                            break;
                        }
                    }
                    '[' => line_buffer.push('['),
                    ']' => {
                        let popped = line_buffer.pop().unwrap();
                        if popped != '[' {
                            count_brackets += 1;
                            break;
                        }
                    }
                    '{' => line_buffer.push('{'),
                    '}' => {
                        let popped = line_buffer.pop().unwrap();
                        if popped != '{' {
                            count_curly += 1;
                            break;
                        }
                    }
                    '<' => line_buffer.push('<'),
                    '>' => {
                        let popped = line_buffer.pop().unwrap();
                        if popped != '<' {
                            count_pointy += 1;
                            break;
                        }
                    }
                    _ => panic!("Unexpected char"),
                }
            }
        }

        // println!(
        //     "{} {} {} {}",
        //     count_round, count_brackets, count_curly, count_pointy
        // );
        count_round * 3 + count_brackets * 57 + count_curly * 1197 + count_pointy * 25137
    }
    fn two(&self, data: &mut Data) -> u64 {
        let mut filter_me_later: Vec<String> = Vec::new();
        for line in data.clone() {
            let mut line_buffer = Vec::new();
            for char in line.chars() {
                match char {
                    '(' => line_buffer.push('('),
                    ')' => {
                        let popped = line_buffer.pop().unwrap();
                        // Corrupt line
                        if popped != '(' {
                            filter_me_later.push(line.to_string());
                            break;
                        }
                    }
                    '[' => line_buffer.push('['),
                    ']' => {
                        let popped = line_buffer.pop().unwrap();
                        // Corrupt line
                        if popped != '[' {
                            filter_me_later.push(line.to_string());
                            break;
                        }
                    }
                    '{' => line_buffer.push('{'),
                    '}' => {
                        let popped = line_buffer.pop().unwrap();
                        // Corrupt line
                        if popped != '{' {
                            filter_me_later.push(line.to_string());
                            break;
                        }
                    }
                    '<' => line_buffer.push('<'),
                    '>' => {
                        let popped = line_buffer.pop().unwrap();
                        // Corrupt line
                        if popped != '<' {
                            filter_me_later.push(line.to_string());
                            break;
                        }
                    }
                    _ => panic!("Unexpected char"),
                }
            }
        }
        let data: Vec<String> = data
            .iter()
            .filter(|f| !filter_me_later.contains(f))
            .map(|f| f.to_string())
            .collect();

        let mut sums = Vec::new();

        for line in data {
            let mut buffer: Vec<char> = Vec::new();
            for char in line.chars() {
                // Those lines aren't corrupt, so no need for checking
                match char {
                    '(' => buffer.push('('),
                    ')' => {
                        buffer.pop().unwrap();
                    }
                    '[' => buffer.push('['),
                    ']' => {
                        buffer.pop().unwrap();
                    }
                    '{' => buffer.push('{'),
                    '}' => {
                        buffer.pop().unwrap();
                    }
                    '<' => buffer.push('<'),
                    '>' => {
                        buffer.pop().unwrap();
                    }
                    _ => panic!("Unexpected char"),
                }
            }
            let mut sum = 0;
            for char in buffer.iter().rev() {
                sum *= 5;
                sum += match char {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!(),
                };
                // println!("{}, {}", i, char);
            }
            sums.push(sum);
            // println!("=============================")
        }
        sums.sort();
        // println!("{:?}: {}", sums, sums.len());
        sums[sums.len()/2]
    }
}
