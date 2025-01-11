use anyhow::*;
use itertools::Itertools;

pub static DAY: u32 = 08;
pub static EXAMPLE_INPUT: &str = "0222112222120000";

pub fn main(input: &str) -> Result<(String, String)> {
    let (width, height) = if input == EXAMPLE_INPUT {
        (2, 2)
    } else {
        (25, 6)
    };

    let layers: Vec<Pixel> = input
        .lines()
        .next()
        .context("Invalid input")?
        .chars()
        .map(|c| c.try_into())
        .try_collect()?;
    let layers = layers.chunks(width * height).collect_vec();

    Ok((ans1(&layers)?.to_string(), ans2(&layers, width)?))
}

fn ans1(layers: &Vec<&[Pixel]>) -> Result<usize> {
    let min_layer = layers
        .iter()
        .map(|layer| layer.iter().counts())
        .min_by_key(|layer| *layer.get(&Pixel::Black).unwrap_or(&usize::MAX))
        .context("Invalid image")?;
    let white = min_layer.get(&Pixel::White).cloned().unwrap_or(0);
    let transparent = min_layer.get(&Pixel::Transparent).cloned().unwrap_or(0);
    Ok(white * transparent)
}

fn ans2(layers: &Vec<&[Pixel]>, width: usize) -> Result<String> {
    let mut full_image = vec![Pixel::Black; layers[0].len()];

    for &layer in layers.iter().rev() {
        let mut pixel_iter = full_image.iter_mut();
        for &layer_pixel in layer {
            let image_pixel = pixel_iter.next().unwrap();
            if layer_pixel != Pixel::Transparent {
                *image_pixel = layer_pixel;
            }
        }
    }

    Ok(format!(
        "\n{}",
        full_image
            .chunks(width)
            .into_iter()
            .map(|line| line
                .iter()
                .map(|c| match c {
                    Pixel::White => '█',
                    _ => ' ',
                })
                .join(""))
            .join("\n")
    ))
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Pixel {
    Black,
    White,
    Transparent,
}

impl TryFrom<char> for Pixel {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '0' => Self::Black,
            '1' => Self::White,
            '2' => Self::Transparent,
            _ => bail!("Invalid pixel: {}", value),
        })
    }
}
