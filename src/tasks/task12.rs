use std::{collections::BinaryHeap, usize};

use crate::task_handler::get_task;

pub fn tasks() {
    let input = get_task(12);
    task1(&input);
}

type Coord = (isize, isize);

#[allow(clippy::needless_continue)]
fn task1(input: &str) {
    let mut frontier: BinaryHeap<Tile> = BinaryHeap::new();

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut heights = vec![0; width * height];

    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.char_indices() {
            let val = if char == 'S' {
                start = (x.try_into().unwrap(), y.try_into().unwrap());
                0
            } else if char == 'E' {
                end = (x.try_into().unwrap(), y.try_into().unwrap());
                25
            } else {
                char.to_ascii_lowercase() as isize - 97
            };
            heights[y * width + x] = val;
        }
    }

    let mut min_costs = vec![usize::MAX; heights.len()];
    frontier.push(Tile { pos: end, moves: 0 });
    while let Some(curr) = frontier.pop() {
        let idx = index_of_tile(curr.pos, width);
        if curr.moves < min_costs[idx] {
            min_costs[idx] = curr.moves;
        } else {
            continue;
        }

        for neighbour in ReachableCoords::new(&heights, curr.pos, width) {
            frontier.push(Tile {
                pos: neighbour,
                moves: curr.moves + 1,
            });
        }
    }

    println!("{}", min_costs[index_of_tile(start, width)]);
    let item = heights
        .iter()
        .enumerate()
        .filter(|x| *x.1 == 'a'.to_ascii_lowercase() as isize - 97)
        .map(|(idx, _)| (min_costs[idx]))
        .min()
        .unwrap();
    println!("{}", item);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

struct Tile {
    pos: Coord,
    moves: usize,
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.moves
            .partial_cmp(&other.moves)
            .map(std::cmp::Ordering::reverse)
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.moves.cmp(&other.moves).reverse()
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
const fn index_of_tile(coord: Coord, width: usize) -> usize {
    (coord.1 * width as isize + coord.0) as usize
}

struct ReachableCoords<'a> {
    heights: &'a Vec<isize>,
    step: usize,
    curr: Coord,
    width: usize,
}

impl<'a> ReachableCoords<'a> {
    const fn new(heights: &'a Vec<isize>, curr: Coord, width: usize) -> Self {
        Self {
            heights,
            curr,
            step: 0,
            width,
        }
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
impl Iterator for ReachableCoords<'_> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let (curr_x, curr_y) = self.curr;
        let curr_height = self.heights[index_of_tile(self.curr, self.width)];
        loop {
            let new_coord @ (x, y) = match self.step {
                0 => (curr_x, curr_y + 1),
                1 => (curr_x + 1, curr_y),
                2 => (curr_x, curr_y - 1),
                3 => (curr_x - 1, curr_y),
                _ => break,
            };
            self.step += 1;
            if (x >= 0
                && x < self.width as isize
                && y >= 0
                && y < (self.heights.len() / self.width) as isize)
                && 1 >= curr_height - self.heights[index_of_tile(new_coord, self.width)]
            {
                return Some((x, y));
            }
        }
        None
    }
}

// 383
// 377
