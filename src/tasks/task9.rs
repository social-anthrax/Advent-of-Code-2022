use std::{cmp::min, collections::HashSet};

use crate::task_handler::get_task;

pub fn tasks() {
    let input = get_task(9);
    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

fn task1(input: &str) -> usize {
    solve::<2>(input)
}

fn task2(input: &str) -> usize {
    solve::<10>(input)
}

fn solve<const N: usize>(input: &str) -> usize {
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    input
        .lines()
        .map(parse_direction)
        .fold([(0, 0); N], |mut knots, (direction, steps)| {
            for _ in 0..steps {
                make_step(&mut knots, &direction, &mut visited);
            }
            knots
        });
    visited.len()
}

type Move = (Direction, u8);
type Pos = (i32, i32);
type Knots<const N: usize> = [Pos; N];

fn make_step<const N: usize>(
    knots: &mut Knots<N>,
    curr_move: &Direction,
    visited: &mut HashSet<Pos>,
) {
    match curr_move {
        Direction::Up => knots[0].1 += 1,
        Direction::Down => knots[0].1 -= 1,
        Direction::Left => knots[0].0 -= 1,
        Direction::Right => knots[0].0 += 1,
    }

    for i in 0..N - 1 {
        let (front_x, front_y) = knots[i];
        let (back_x, back_y) = &mut knots[i + 1];
        let (diff_x, diff_y) = (front_x - *back_x, front_y - *back_y);
        if (diff_x.abs() == 2 && diff_y == 0)
            || (diff_y.abs() == 2 && diff_x == 0)
            || (diff_y.abs() + diff_x.abs() > 2)
        {
            *back_x += diff_x.signum() * min(diff_x.abs(), 1);
            *back_y += diff_y.signum() * min(diff_y.abs(), 1);
        }
    }

    visited.insert(knots[N - 1]);
}

fn parse_direction(input: &str) -> Move {
    let (direction, steps) = input.split_once(' ').unwrap();
    (
        match direction {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            &_ => panic!("Unrecognized string"),
        },
        steps.parse().unwrap(),
    )
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
pub mod tests {
    use super::task1;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_task1() {
        assert_eq!(task1(INPUT), 13);
    }
}
