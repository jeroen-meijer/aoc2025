use std::str::FromStr;

use anyhow::{anyhow, bail};
use itertools::Itertools;

use super::prelude::*;

pub fn get_assignment() -> Assignment {
    Assignment::new(AssignmentOptions {
        day: 1,
        description: "Secret Entrance",
        run: _run,
        example_input_day_1: Some(
            "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        ),
        answer_example_day_1: Some(3.into()),
        example_input_day_2: Some(
            "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        ),
        answer_example_day_2: Some(6.into()),
        answer_day_1: Some(992.into()),
        answer_day_2: Some(6133.into()),
    })
}

#[derive(Debug, Copy, Clone)]
pub struct Dial {
    max_positions: u16,
    position: u16,
}

impl Dial {
    fn turn_left_mut(&mut self, clicks: &u16) -> u16 {
        let mut zero_passes = clicks / self.max_positions;
        let wrapped_clicks = clicks % self.max_positions;

        let post_turn_pos = self.position as i32 - (wrapped_clicks as i32);
        if self.position != 0 && post_turn_pos < 0 {
            zero_passes += 1;
        }

        let new_position = ((post_turn_pos + (self.max_positions as i32)) as u16) % 100;

        self.position = new_position;

        zero_passes
    }

    fn turn_right_mut(&mut self, clicks: &u16) -> u16 {
        let mut zero_passes = clicks / self.max_positions;
        let wrapped_clicks = clicks % self.max_positions;

        let post_turn_pos = self.position + wrapped_clicks;
        if self.position != self.max_positions && post_turn_pos > self.max_positions {
            zero_passes += 1;
        }

        let new_position = post_turn_pos % self.max_positions;

        self.position = new_position;

        zero_passes
    }

    fn turn_mut(&mut self, clicks: &u16, direction: &TurnDirection) -> u16 {
        match direction {
            TurnDirection::Left => self.turn_left_mut(clicks),
            TurnDirection::Right => self.turn_right_mut(clicks),
        }
    }

    #[must_use]
    #[allow(dead_code)]
    fn turn_right(&self, clicks: &u16) -> Self {
        let mut new = self.clone();
        new.turn_right_mut(clicks);
        new
    }

    #[must_use]
    #[allow(dead_code)]
    fn turn_left(&self, clicks: &u16) -> Self {
        let mut new = self.clone();
        new.turn_left_mut(clicks);
        new
    }

    #[must_use]
    #[allow(dead_code)]
    fn turn(&self, clicks: &u16, direction: &TurnDirection) -> Self {
        let mut new = self.clone();
        new.turn_mut(clicks, direction);
        new
    }
}

struct Instruction {
    clicks: u16,
    direction: TurnDirection,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let chars = s.chars().collect_vec();

        let dir_char = chars.first().ok_or(anyhow!("No first char found"))?;

        let direction = match dir_char {
            'L' => TurnDirection::Left,
            'R' => TurnDirection::Right,
            _ => {
                bail!(format!(
                    "Invalid diirection char in instruction string: {:?}",
                    s
                ));
            }
        };

        let clicks = chars.iter().skip(1).collect::<String>().parse::<u16>()?;

        Ok(Instruction { clicks, direction })
    }
}

enum TurnDirection {
    Left,
    Right,
}

fn _run(context: AssignmentRuntimeContext) -> Result<Option<Answer>> {
    let mut dial = Dial {
        position: 50,
        max_positions: 100,
    };

    let mut zero_hits = 0;
    let mut zero_passes = 0;

    for line in context.data {
        let instruction = line.parse::<Instruction>()?;
        zero_passes += dial.turn_mut(&instruction.clicks, &instruction.direction);
        if dial.position == 0 {
            zero_hits += 1;
        }
    }

    if context.part_number == 1 {
        Ok(Some(zero_hits.into()))
    } else {
        Ok(Some((zero_hits + zero_passes).into()))
    }
}
