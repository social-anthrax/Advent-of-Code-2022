use std::collections::HashSet;

use crate::task_handler::get_task;

pub fn tasks() {
    println!("Day 3");
    println!("{}", task1());
    println!("{}", task2());
}

fn prio(c: char) -> usize {
    if c.is_uppercase() {
        (c as usize) - 38
    } else {
        (c as usize) - 96
    }
}

fn task1() -> usize {
    let task = get_task(3);
    task.lines()
        .map(|x| x.split_at(x.len() / 2))
        .map(|(a, b)| -> (HashSet<char>, HashSet<char>) {
            (
                a.chars().collect::<HashSet<_>>(),
                b.chars().collect::<HashSet<_>>(),
            )
        })
        .map(|(a, b)| a.intersection(&b).copied().map(prio).sum::<usize>())
        .sum()
}

fn task2() -> usize {
    let task = get_task(3);
    task.lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            group
                .iter()
                .map(|y| y.chars().collect::<HashSet<_>>())
                .reduce(|curr, next| curr.intersection(&next).copied().collect())
                .unwrap()
                .iter()
                .fold(0, |c, n| c + prio(*n))
        })
        .sum()
}
