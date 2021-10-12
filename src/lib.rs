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

macro_rules! run_day {
    ($d:literal, $run:expr) => {{
        let day = Day::<$d>::init();
        day.run($run);
    }};
}

struct Day<const DAY: u8>;

trait Runnable {
    fn init() -> Self;
    fn one(&self);
    fn two(&self);
    fn run(&self, run: Run) {
        match run {
            Run::Both => {
                self.one();
                self.two();
            }
            Run::One => self.one(),
            Run::Two => self.two(),
        };
    }
}

pub fn run(day: u8, run: Run) {
    println!("Running Day {}", day);

    match day {
        1 => run_day!(1, run),
        // 2 => run_day!(D2, run),
        // 3 => run_day!(D3, run),
        // 4 => run_day!(D4, run),
        // 5 => run_day!(D5, run),
        // 6 => run_day!(D6, run),
        // 7 => run_day!(D7, run),
        // 8 => run_day!(D8, run),
        // 9 => run_day!(D9, run),
        // 10 => run_day!(D10, run),
        // 11 => run_day!(D11, run),
        // 12 => run_day!(D12, run),
        // 13 => run_day!(D13, run),
        // 14 => run_day!(D14, run),
        // 15 => run_day!(D15, run),
        // 16 => run_day!(D16, run),
        // 17 => run_day!(D17, run),
        // 18 => run_day!(D18, run),
        // 19 => run_day!(D19, run),
        // 20 => run_day!(D20, run),
        // 21 => run_day!(D21, run),
        // 22 => run_day!(D22, run),
        // 23 => run_day!(D23, run),
        // 24 => run_day!(D24, run),
        // 25 => run_day!(D25, run),
        _ => panic!("Days out of Bounds! No presents for you!"),
    }
}
