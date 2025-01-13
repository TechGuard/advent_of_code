use crate::utils::intcode;
use anyhow::*;
use std::collections::BTreeMap;

pub static DAY: u32 = 11;
pub static EXAMPLE_INPUT: &str = "";

pub fn main(input: &str) -> Result<(usize, String)> {
    let data = intcode::parse_data(input)?;
    Ok((black_panels(&data)?, white_panels(&data)?))
}

pub fn black_panels(data: &Vec<i64>) -> Result<usize> {
    Ok(paint_panels(&data, 0)?.keys().len())
}

pub fn white_panels(data: &Vec<i64>) -> Result<String> {
    let panels = paint_panels(&data, 1)?;
    let mut height = 0;
    let mut width = 0;
    for (y, x) in panels.keys() {
        height = height.max(*y + 1);
        width = width.max(*x + 1);
    }
    let mut identifier = String::new();
    for y in 0..height {
        identifier.push('\n');
        for x in 0..width {
            if let Some(panel) = panels.get(&(y, x)) {
                if *panel == 1 {
                    identifier.push('█');
                    continue;
                }
            }
            identifier.push(' ');
        }
    }
    Ok(identifier)
}

pub fn paint_panels(data: &Vec<i64>, default_color: i64) -> Result<BTreeMap<(i64, i64), i64>> {
    use intcode::Action;

    let mut program = intcode::Program::new(data.clone());
    let mut panels = BTreeMap::new();
    let mut pos = (0, 0);
    let mut dir = (-1, 0);

    loop {
        if !panels.contains_key(&pos) {
            panels.insert(pos, default_color);
        }

        let panel = panels.get_mut(&pos).unwrap();
        program.give_input(*panel);

        match program.execute()? {
            Action::WaitingForInput => bail!("Program should not ask for input"),
            Action::Output(color) => *panel = color,
            Action::Halt => break,
        }
        match program.execute()? {
            Action::WaitingForInput => bail!("Program should not ask for input"),
            Action::Output(direction) => match direction {
                0 => dir = (-dir.1, dir.0),
                1 => dir = (dir.1, -dir.0),
                _ => bail!("Unexpected direction: {}", direction),
            },
            Action::Halt => break,
        }

        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }
    Ok(panels)
}
