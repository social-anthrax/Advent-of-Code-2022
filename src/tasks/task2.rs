use std::str::FromStr;

use crate::task_handler::get_task;

pub fn tasks() {
    println!("{}", task1());
    // println!("{}", task2());
}

static SCORE_TASK1: [[usize; 3]; 3] = [
    // . r .p .s
    [3, 6, 0],
    [0, 3, 6],
    [6, 0, 3],
];

#[derive(PartialEq, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

impl Choice {
    pub fn play(game: &[Self]) -> usize {
        SCORE_TASK1[game[0].score() - 1][game[1].score() - 1]
    }

    pub const fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }
}

impl FromStr for Choice {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissor),
            _ => panic!("Failed to parse"),
        }
    }
}

fn task1() -> usize {
    get_task(2)
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| Choice::from_str(x).unwrap())
                .collect::<Vec<_>>()
        })
        .fold(0_usize, |prev, curr| {
            prev + Choice::play(&curr) + curr[1].score()
        })
}
