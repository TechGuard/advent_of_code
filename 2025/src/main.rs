use anyhow::*;
use std::io::Read;

macro_rules! register_days {
    ( $( $day:ident ), +$(,)* ) => {
        $(
            mod $day;
        )+
        fn execute_day(day: u32, example: bool) -> Result<(String, String)> {
            $(
                if $day::DAY == day {
                    let results;
                    if example {
                        results = $day::main($day::EXAMPLE_INPUT)?;
                    } else {
                        let mut input = String::new();
                        std::io::stdin()
                            .read_to_string(&mut input)
                            .context("Expected input")?;
                        results = $day::main(&input)?;
                    }
                    // Convert answers to string
                    return Ok((results.0.to_string(), results.1.to_string()));
                }
            )+
            bail!("Day {:02} is not implemented", day)
        }
    };
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let day = args
        .get(1)
        .context("Missing required <DAY> argument")?
        .parse()
        .context("<DAY> argument needs to be a valid number")?;

    let mut example = false;
    if let Some(arg1) = args.get(2) {
        example = arg1 == "--example" || arg1 == "-e";
    }

    let measure = std::time::Instant::now();
    let (ans1, ans2) = execute_day(day, example)?;
    println!(
        "Answer Part One: {}\nAnswer Part Two: {}\nElapsed time: {:.2?}",
        ans1,
        ans2,
        measure.elapsed()
    );
    Ok(())
}

register_days!(
    day01_secret_entrance,
    day02_gift_shop,
    day03_lobby,
    day04_printing_department,
    day05_cafeteria,
    day06_trash_compactor,
    day07_laboratories,
    day08_playground,
);
