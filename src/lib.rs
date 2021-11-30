aom::mod_days!();

pub enum Run {
    Both,
    One,
    Two,
}

struct Day<const DAY: u8>;

trait Runnable<T: std::clone::Clone> {
    fn init() -> (Self, T)
    where
        Self: Sized;
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
