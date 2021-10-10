use aoc2021::{run, Run};

fn main() {
    let mut day = std::env::args();

    match day.len() {
        1 => eprintln!("You didn't specify which day to run. How could you.\nUsage: cargo run -- <Day> [Part]\n\t<Day> is a required Integer between 1 and 25\n\t[Part] is an optional Integer between 1 and 2. If not specified, both parts will run."),
        2 => {
            let day_number = match day.nth(1).unwrap().parse::<u8>() {
                Ok(n) => n,
                Err(_) => panic!("AHAHHAHAHHAHHHHHH"),
            };
            run(day_number, Run::Both);
        },
        3 => {
            let day_number = match day.nth(1).unwrap().parse::<u8>() {
                Ok(n) => n,
                Err(_) => panic!("WHAT IS EVEN HAPPENINGGGGGG?!?!??!"),
            };
            let run_mode = match day.next().unwrap().parse::<u8>() {
                Ok(1) => Run::One,
                Ok(2) => Run::Two,
                Ok(_) => panic!("Only 1 and 2 are allowed values for Part. If you want to run both parts, just don't include the [Part] option."),
                Err(_) => panic!("SEND HELP PLEEEEEAAASEEEEE"),
            };
            run(day_number, run_mode);
        }
        _ => eprintln!("WHOA, BY THE ELFES! One must not supply more than 2 arguments to this program! How could you?!\nUsage: cargo run -- <Day> [Part]\n\t<Day> is a required Integer between 1 and 25\n\t[Part] is an optional Integer between 1 and 2. If not specified, both parts will run."), 
    }
}
