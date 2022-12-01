use core::num;
use std::{
    cmp::{self, Reverse},
    usize,
};

use itertools::Itertools;

use crate::task_handler::get_task;

pub fn tasks() {
    println!("{}", task1());
    println!("{}", task2());
}
fn task1() -> usize {
    *total_calories().iter().max().unwrap()
}

fn task2() -> usize {
    total_calories()
        .iter()
        .fold([0_usize, 0_usize, 0_usize], |mut curr_top_3, x| {
            if *x > curr_top_3[2] {
                // Replace lowest
                curr_top_3[2] = *x;
                // Re-sort
                curr_top_3.sort_unstable_by_key(|x| Reverse(*x));
            }
            curr_top_3
        })
        .iter()
        .sum()
}

fn total_calories() -> Vec<usize> {
    let task = get_task(1);
    task.lines()
        .group_by(|x| x != &"")
        .into_iter()
        .map(|x| x.1.map(|x| x.parse::<usize>().unwrap_or(0)).sum::<usize>())
        .collect()
}

fn task1_long() -> usize {
    let task = get_task(1);
    task.lines()
        .group_by(|x| x != &"")
        .into_iter()
        .map(|x| x.1.map(|x| x.parse::<usize>().unwrap_or(0)).sum::<usize>())
        .max()
        .unwrap()
}

fn task2_long() -> usize {
    let task = get_task(1);
    task.lines()
        .group_by(|x| x != &"")
        .into_iter()
        .map(|x| x.1.map(|x| x.parse::<usize>().unwrap_or(0)).sum::<usize>())
        .fold([0_usize, 0_usize, 0_usize], |mut curr_top_3, x| {
            if x > curr_top_3[2] {
                // Replace lowest
                curr_top_3[2] = x;
                // Re-sort
                curr_top_3.sort_unstable_by_key(|x| Reverse(*x));
            }
            curr_top_3
        })
        .iter()
        .sum()
}

#[cfg(test)]
mod test {
    use super::{task1, task2};

    #[test]
    fn test_task1() {
        assert_eq!(72511, task1());
    }

    #[test]
    fn test_task2() {
        assert_eq!(212_117, task2());
    }
}
