mod err;
use aoc2021::{run, Run};
use err::ChristmasError;

fn main() -> Result<(), ChristmasError> {
    let mut day = std::env::args();

    match day.len() {
        1 => {
            return Err(ChristmasError {
                message: "You didn't specify which day to run.",
            })
        }
        2 => {
            let day_number = day.nth(1).unwrap().parse::<u8>()?;
            if !(day_number >= 1 && day_number <= 25) {
                return Err(ChristmasError {
                    message: "You've entered a wrong day!",
                });
            }
            run(day_number, Run::Both);
        }
        3 => {
            let day_number = day.nth(1).unwrap().parse::<u8>()?;
            let run_mode = match day.next().unwrap().parse::<u8>()? {
                1 => Run::One,
                2 => Run::Two,
                _ => {
                    return Err(ChristmasError{message: "Only 1 and 2 are allowed values for Part. If you want to run both parts, just don't include the [Part] option."});
                }
            };
            run(day_number, run_mode);
        }
        _ => {
            return Err(ChristmasError {
                message: "You have specified to many arguments.",
            })
        }
    }
    Ok(())
}
