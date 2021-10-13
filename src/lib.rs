macro_rules! include_days {
    ($($day:ident),+) => {
        $(
            mod $day;
        )+
    }
}

include_days!(
    d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13, d14, d15, d16, d17, d18, d19, d20, d21,
    d22, d23, d24, d25
);

pub enum Run {
    Both,
    One,
    Two,
}

struct Day<const DAY: u8>;

trait Runnable<T: std::clone::Clone> {
    fn init() -> (Self, T) where Self: Sized;
    fn one(&self, data: &mut T);
    fn two(&self, data: &mut T);
    fn run(&self, data: &mut T, run: Run) {
        match run {
            Run::Both => {
                self.one(&mut data.clone());
                self.two(&mut data.clone());
            }
            Run::One => self.one(data),
            Run::Two => self.two(data),
        };
    }
}

pub fn run(day: u8, run: Run) {
    println!("Running Day {}", day);
    use aom::generate_day_match;
    generate_day_match!();
}
