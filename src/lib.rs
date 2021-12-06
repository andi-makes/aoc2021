use std::time::Instant;

aom::mod_days!();

pub enum Run {
    Both,
    One,
    Two,
}

struct Day<const DAY: u8>;

trait Runnable<T: std::clone::Clone> {
    fn init(input: &str) -> (Self, T)
    where
        Self: Sized;
    fn one(&self, data: &mut T) -> u64;
    fn two(&self, data: &mut T) -> u64;
    fn run(&self, data: &mut T, run: Run) {
        match run {
            Run::Both => {
                let mut data_one = data.clone();
                let mut data_two = data.clone();
                let start = Instant::now();
                let res = self.one(&mut data_one);
                println!("Part 1: [{}],\ttook {}us", res, start.elapsed().as_micros());
                let start = Instant::now();
                let res = self.two(&mut data_two);
                println!("Part 2: [{}],\ttook {}us", res, start.elapsed().as_micros());
            }
            Run::One => {
                let start = Instant::now();
                let res = self.one(data);
                println!("Part 1: [{}],\ttook {}us", res, start.elapsed().as_micros());
            }
            Run::Two => {
                let start = Instant::now();
                let res = self.two(data);
                println!("Part 2: [{}],\ttook {}us", res, start.elapsed().as_micros());
            }
        };
    }
}

pub fn run(day: u8, run: Run) {
    println!("Running Day {}", day);
    use aom::generate_day_match;
    generate_day_match!();
}
