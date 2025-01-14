use anyhow::*;
use std::io::Read;

mod utils;

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

    let (ans1, ans2) = execute_day(day, example)?;
    println!("Answer Part One: {}\nAnswer Part Two: {}", ans1, ans2);
    Ok(())
}

register_days!(
    day01_the_tyranny_of_the_rocket_equation,
    day02_1202_program_alarm,
    day03_crossed_wires,
    day04_secure_container,
    day05_sunny_with_a_chance_of_asteroids,
    day06_universal_orbit_map,
    day07_amplification_circuit,
    day08_space_image_format,
    day09_sensor_boost,
    day10_monitoring_station,
    day11_space_police,
    day12_the_n_body_problem,
);
