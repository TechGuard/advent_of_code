use crate::utils::intcode;
use anyhow::*;
use itertools::Itertools;

pub static DAY: u32 = 13;
pub static EXAMPLE_INPUT: &str = "";

pub fn main(input: &str) -> Result<(usize, i64)> {
    let mut data = intcode::parse_data(input)?;
    data[0] = 2; // Freeplay

    let mut game = Game::new(data);
    game.update()?;

    Ok((game.count_blocks(), game.beat_game()?))
}

impl Game {
    fn new(data: Vec<i64>) -> Game {
        Game {
            program: intcode::Program::new(data),
            screen: vec![Tile::Empty; 40 * 20],
            width: 40,
            score: 0,
        }
    }

    fn count_blocks(&mut self) -> usize {
        self.screen
            .iter()
            .filter(|&tile| tile == &Tile::Block)
            .count()
    }

    fn beat_game(&mut self) -> Result<i64> {
        self.move_joystick(Input::Neutral);

        while self.update()? {
            let ball = self.index_to_x(self.index_of(Tile::Ball)?);
            let paddle = self.index_to_x(self.index_of(Tile::Paddle)?);

            // No smart AI: just keep the paddle underneath the ball
            if ball < paddle {
                self.move_joystick(Input::Left);
            } else if ball > paddle {
                self.move_joystick(Input::Right);
            } else {
                self.move_joystick(Input::Neutral);
            }
        }

        Ok(self.score)
    }

    fn index_to_x(&self, index: usize) -> usize {
        index % self.width
    }

    fn index_of(&self, tile: Tile) -> Result<usize> {
        Ok(self
            .screen
            .iter()
            .find_position(|&t| t == &tile)
            .with_context(|| anyhow!("Tile does not exist: {:?}", tile))?
            .0)
    }

    fn move_joystick(&mut self, input: Input) {
        self.program.give_input(input as i64);
    }

    fn update(&mut self) -> Result<bool> {
        use intcode::Action;

        let mut x = 0;
        let mut y = 0;
        let mut instruction_count = 0;

        Ok(loop {
            match self.program.execute()? {
                Action::Output(output) => {
                    instruction_count += 1;
                    if instruction_count == 1 {
                        x = output;
                    } else if instruction_count == 2 {
                        y = output;
                    } else {
                        instruction_count = 0;
                        if x < 0 {
                            self.score = output;
                        } else {
                            self.screen[y as usize * self.width + x as usize] =
                                output.try_into()?;
                        }
                    }
                }
                Action::WaitingForInput => break true,
                Action::Halt => break false,
            }
        })
    }
}

struct Game {
    program: intcode::Program,
    screen: Vec<Tile>,
    width: usize,
    score: i64,
}

#[derive(PartialEq, Clone, Debug)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

enum Input {
    Neutral = 0,
    Left = -1,
    Right = 1,
}

impl TryFrom<i64> for Tile {
    type Error = anyhow::Error;
    fn try_from(value: i64) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            _ => bail!("Invalid tile: {}", value),
        })
    }
}
