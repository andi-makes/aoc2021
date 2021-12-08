use crate::{Day, Runnable};
use const_format::formatcp;
use num::pow;

const CURRENT_DAY: u8 = 8;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

#[derive(Clone, Debug)]
pub struct SegmentData {
    pre: Vec<String>,
    post: Vec<String>,
}

type Data = Vec<SegmentData>;
impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init(_input: &str) -> (Self, Data) {
        let v: Data = std::fs::read_to_string(FILE)
            .unwrap()
            .lines()
            .map(|s| {
                let mut split = s.split(" | ");
                SegmentData {
                    pre: split
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|f| {
                        let mut buf = [0; 7];
                        f.split("").for_each(|c| match c {
                            "a" => buf[0] = 1,
                            "b" => buf[1] = 1,
                            "c" => buf[2] = 1,
                            "d" => buf[3] = 1,
                            "e" => buf[4] = 1,
                            "f" => buf[5] = 1,
                            "g" => buf[6] = 1,
                            _ => {}
                        });
                        let mut out = String::new();
                        for i in 0..7 {
                            if buf[i] == 1 {
                                match i {
                                    0 => out += "a",
                                    1 => out += "b",
                                    2 => out += "c",
                                    3 => out += "d",
                                    4 => out += "e",
                                    5 => out += "f",
                                    6 => out += "g",
                                    _ => panic!(),
                                }
                            }
                        }
                        out
                    })
                    .collect(),
                    post: split
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .map(|f| {
                            let mut buf = [0; 7];
                            f.split("").for_each(|c| match c {
                                "a" => buf[0] = 1,
                                "b" => buf[1] = 1,
                                "c" => buf[2] = 1,
                                "d" => buf[3] = 1,
                                "e" => buf[4] = 1,
                                "f" => buf[5] = 1,
                                "g" => buf[6] = 1,
                                _ => {}
                            });
                            let mut out = String::new();
                            for i in 0..7 {
                                if buf[i] == 1 {
                                    match i {
                                        0 => out += "a",
                                        1 => out += "b",
                                        2 => out += "c",
                                        3 => out += "d",
                                        4 => out += "e",
                                        5 => out += "f",
                                        6 => out += "g",
                                        _ => panic!(),
                                    }
                                }
                            }
                            out
                        })
                        .collect(),
                }
            })
            .collect();

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 {
        data.iter().fold(0, |sum, d| {
            sum + d
                .post
                .iter()
                .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
                .collect::<Vec<&String>>()
                .len() as u64
        })
    }
    /// BIG WARNING!
    /// THE FOLLOWING CODE IS SO BEAUTIFUL, IT MIGHT MAKE YOUR EYES BLEED!
    /// YOU'VE BEEN WARNED!
    fn two(&self, data: &mut Data) -> u64 {
        data.iter().fold(0, |sum, d| {
            // Ok, lets do this. I need to know which Character Sequence is a specific
            // number.
            // I know:
            let one: Vec<&str> = d
                .pre
                .iter()
                .filter(|c| c.len() == 2)
                .next()
                .unwrap()
                .split("")
                .filter(|s| !s.is_empty())
                .collect();
            let four: Vec<&str> = d
                .pre
                .iter()
                .filter(|c| c.len() == 4)
                .next()
                .unwrap()
                .split("")
                .filter(|s| !s.is_empty())
                .collect();
            let seven: Vec<&str> = d
                .pre
                .iter()
                .filter(|c| c.len() == 3)
                .next()
                .unwrap()
                .split("")
                .filter(|s| !s.is_empty())
                .collect();
            let eight: Vec<&str> = d
                .pre
                .iter()
                .filter(|c| c.len() == 7)
                .next()
                .unwrap()
                .split("")
                .filter(|s| !s.is_empty())
                .collect();
                
            // Out of 1, we can determine 2, 5 and 6, because that are the only
            // three numbers that don't contain 1 as a whole.
            let two_five_or_six: Vec<&String> = d
                .pre
                .iter()
                .filter(|s| !s.contains(one[0]) || !s.contains(one[1]))
                .collect();

            // Six is the only number out of 2, 5 and 6 that has 6 segments on
            let six: Vec<&str> = two_five_or_six
                .iter()
                .filter(|s| s.len() == 6)
                .next()
                .unwrap()
                .split("")
                .filter(|s| !s.is_empty())
                .collect();

            // Now, with the remaining options of two or five and 4, we can determine 5.
            // 5 is the one that's only one segment off of 4.
            let five: Vec<&str> = two_five_or_six
                .iter()
                .filter(|s| s.len() != 6)
                .filter(|s| {
                    let mut counter = 0;
                    for c in &four {
                        if !s.contains(c) {
                            counter += 1;
                        }
                    }
                    counter == 1
                })
                .next()
                .unwrap()
                .split("")
                .filter(|s| !s.is_empty())
                .collect();

            // And two is the one with 2 segments off.
            let two_num: Vec<&str> = two_five_or_six
                .iter()
                .filter(|s| s.len() != 6)
                .filter(|s| {
                    let mut counter = 0;
                    for c in &four {
                        if !s.contains(c) {
                            counter += 1;
                        }
                    }
                    counter == 2
                })
                .next()
                .unwrap()
                .split("")
                .filter(|s| !s.is_empty())
                .collect();

            // There is only one segment left with the length 5, and this is three:
            let three: Vec<&str> = d
                .pre
                .iter()
                .filter(|s| s.len() == 5)
                .filter(|s| !s.contains(&two_num.join("")) && !s.contains(&five.join("")))
                .next()
                .unwrap()
                .split("")
                .filter(|s| !s.is_empty())
                .collect();

            // There are only two numbers left: 0 and 9!
            let zero_or_nine: Vec<&String> = d
                .pre
                .iter()
                .filter(|s| (s.len() == 6) && (!s.contains(&six.join(""))))
                .collect();

            // Nine completely contains four:
            let nine: Vec<&str> = zero_or_nine
                .iter()
                .filter(|s| {
                    let mut counter = 0;
                    for c in &four {
                        if !s.contains(c) {
                            counter += 1;
                        }
                    }
                    counter == 0
                })
                .next()
                .unwrap()
                .split("")
                .filter(|s| !s.is_empty())
                .collect();

            // Zero does not contain one segment of four:
            let zero: Vec<&str> = zero_or_nine
                .iter()
                .filter(|s| {
                    let mut counter = 0;
                    for c in &four {
                        if !s.contains(c) {
                            counter += 1;
                        }
                    }
                    counter == 1
                })
                .next()
                .unwrap()
                .split("")
                .filter(|s| !s.is_empty())
                .collect();

            // println!("{:?} | {:?}", d.pre, d.post);

            // Now we have determined all numbers (at least I hope so). YAY!

            // Make the numbers whole again
            let zero = zero.join("");
            let one = one.join("");
            let two_num = two_num.join("");
            let three = three.join("");
            let four = four.join("");
            let five = five.join("");
            let six = six.join("");
            let seven = seven.join("");
            let eight = eight.join("");
            let nine = nine.join("");

            // Let's parse the output, shall we?
            let number = d.post.iter().map(|f| {
                if f == &zero {
                    print!("0");
                    return 0;
                }
                if f == &one {
                    print!("1");
                    return 1;
                }
                if f == &two_num {
                    print!("2");
                    return 2;
                }
                if f == &three {
                    print!("3");
                    return 3;
                }
                if f == &four {
                    print!("4");
                    return 4;
                }
                if f == &five {
                    print!("5");
                    return 5;
                }
                if f == &six {
                    print!("6");
                    return 6;
                }
                if f == &seven {
                    print!("7");
                    return 7;
                }
                if f == &eight {
                    print!("8");
                    return 8;
                }
                if f == &nine {
                    print!("9");
                    return 9;
                }
                0
            }).enumerate().fold(0, |sum, (i, f)| sum + f*pow(10, 3-i));
            print!(", {}", sum);
            println!("");
            sum + number
        })
    }
}
