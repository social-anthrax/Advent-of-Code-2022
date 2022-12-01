use std::{cmp, usize};

use crate::task_handler::get_task;

pub fn tasks() {
    println!("{}", task1());
    println!("{}", task2());
}

fn total_calories() -> std::vec::Vec<usize> {
    let task = get_task(1);
    let lines = task.lines().collect::<Vec<_>>();
    lines
        .split(|x| x == &"")
        .map(|x| x.iter().map(|x| x.parse::<usize>().unwrap()).sum::<usize>())
        .collect::<Vec<_>>()
}
fn task1() -> usize {
    *total_calories().iter().max().unwrap()
}

fn task2() -> usize {
    let mut total = total_calories();
    total.sort_unstable_by_key(|x| cmp::Reverse(*x));
    total[0..3].iter().sum()
}
