use anyhow::*;
use std::io::Read;

macro_rules! register_days {
    ( $( $day:ident ), +$(,)* ) => {
        $(
            mod $day;
        )+
        fn execute(day: u32, example: bool) -> Result<()> {
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
                    println!("Answer Part One: {}\nAnswer Part Two: {}", results.0, results.1);
                    return Ok(());
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

    execute(day, example)
}

register_days!(
    day01_the_tyranny_of_the_rocket_equation,
    day02_1202_program_alarm,
    day03_crossed_wires,
    day04_secure_container,
    day05_sunny_with_a_chance_of_asteroids,
    day06_universal_orbit_map,
    day07_amplification_circuit,
);
