use super::{Runnable, Day};

impl Runnable for Day<1> {
    fn init() -> Self {
        println!("Init Day 1");
        Self {}
    }
    fn one(&self) {
        println!("Part 1 Day 1")
    }
    fn two(&self) {
        println!("Part 2 Day 1")
    }
}
