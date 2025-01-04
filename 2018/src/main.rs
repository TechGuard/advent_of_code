use std::io::Read;

#[macro_use]
extern crate lazy_static;

macro_rules! register_days {
    ( $( $day:ident ), +$(,)* ) => {
        $(
            mod $day;
        )+
        fn main() {
            let args: Vec<String> = std::env::args().collect();

            let day = args
                .get(1)
                .expect("Expected <DAY> argument")
                .parse::<u32>()
                .expect("Expected <DAY> argument to be an integer");

            let mut example = false;
            if let Some(arg1) = args.get(2) {
                example = arg1 == "--example" || arg1 == "-e";
            }

            $(
                if $day::DAY == day {
                    let results;
                    if example {
                        results = $day::main($day::EXAMPLE_INPUT);
                    } else {
                        let mut input = String::new();
                        std::io::stdin()
                            .read_to_string(&mut input)
                            .expect("Expected input");
                        results = $day::main(&input);
                    }
                    println!("Answer Part One: {}", results.0);
                    println!("Answer Part Two: {}", results.1);
                    return;
                }
            )+

            eprintln!("Day {:02} not found", day);
            std::process::exit(1);
        }
    };
}

register_days!(
    day01_chronal_calibration,
    day02_inventory_management_system,
    day03_no_matter_how_you_slice_it,
    day04_repose_record,
    day05_alchemical_reduction,
    day06_chronal_coordinates,
    day07_the_sum_of_its_parts,
    day08_memory_maneuver,
    day09_marble_mania,
    day10_the_stars_align,
    day11_chronal_charge,
    day12_subterranean_sustainability,
    day13_mine_cart_madness,
    day14_chocolate_charts,
    day15_beverage_bandits,
    day16_chronal_classification,
    day17_reservoir_research,
    day18_settlers_of_the_north_pole,
    day19_go_with_the_flow,
    day20_a_regular_map,
    day21_chronal_conversion,
    day22_mode_maze,
    day23_experimental_emergency_teleportation,
);
