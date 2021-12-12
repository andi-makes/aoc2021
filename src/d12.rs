use std::{collections::HashMap, borrow::BorrowMut};

use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 12;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

#[derive(Clone, Debug)]
pub struct Node {
    connected: Vec<String>,
    is_small: bool,
    visited: u8,
}

type Data = HashMap<String, Node>;

fn zeug(map: &mut Data, node: &String) -> u64 {
    if node == &"end".to_string() {
        return 1;
    }
    if map[node].is_small {
        map.get_mut(node).unwrap().visited += 1;
    }
    let mut sum = 0;
    for con in &map[node].connected {
        if map[con].visited != 1 {
            sum += zeug(&mut map.clone(), con);
        }
    }
    sum
}

fn zeug2(map: &mut Data, node: &String, twice: bool) -> u64 {
    if node == &"end".to_string() {
        return 1;
    }
    if map[node].is_small {
        map.get_mut(node).unwrap().visited += 1;
    }
    let mut sum = 0;
    for con in &map[node].connected {
        if map[con].visited == 0 {
            sum += zeug2(&mut map.clone(), con, twice);
        } else if map[con].visited == 1 {
            if !twice {
                sum += zeug2(&mut map.clone(), con, true);
            }
        }
    }
    sum
}

impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init(_input: &str) -> (Self, Data) {
        let mut v: Data = HashMap::new();
        std::fs::read_to_string(FILE)
            .unwrap()
            .lines()
            .for_each(|s| {
                let d: Vec<&str> = s.split('-').collect();
                let index = d[0];
                let connected = d[1];

                let is_index_small = index.chars().next().unwrap().is_ascii_lowercase();
                let is_conne_small = connected.chars().next().unwrap().is_ascii_lowercase();

                v.entry(index.to_string())
                    .or_insert(Node {
                        connected: Vec::new(),
                        visited: 0,
                        is_small: is_index_small,
                    })
                    .connected
                    .push(connected.to_string());

                v.entry(connected.to_string())
                    .or_insert(Node {
                        connected: Vec::new(),
                        visited: 0,
                        is_small: is_conne_small,
                    })
                    .connected
                    .push(index.to_string());
            });

        // v.iter().fold(0, |sum, elem| {
        //     sum + elem.1.connected.len()
        // });

        // println!("{:?}", v.iter().fold(0, |sum, elem| {
        //     sum + elem.1.connected.len()
        // }));

        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 {
        zeug(data, &"start".into())
    }
    fn two(&self, data: &mut Data) -> u64 {
        data.get_mut("start".into()).unwrap().visited = 2;
        zeug2(data, &"start".into(), false)
    }
}
