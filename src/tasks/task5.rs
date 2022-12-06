use std::{array, usize};

use crate::task_handler::get_task;

pub fn tasks() {
    let (mut stacks, instructions) = parse_task(&get_task(5));
    println!("{}", task1(&mut stacks.clone(), &instructions));
    println!("{}", task2(&mut stacks, &instructions));
}

type Stacks = [Vec<char>; 9];
type Instructions = Vec<(usize, usize, usize)>;

fn parse_task(input: &str) -> (Stacks, Instructions) {
    let mut stacks: Stacks = array::from_fn(|_| Vec::with_capacity(72));
    let (stacks_str, moves) = input.split_once("\n\n").unwrap();
    stacks_str.lines().rev().for_each(|x| {
        x.chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|x| x.iter().collect::<String>())
            .enumerate()
            .filter(|(_, str)| !str.trim().is_empty())
            .for_each(|(i, block)| stacks[i].push(block.chars().nth(1).unwrap()));
    });
    let parsed_moves = moves
        .lines()
        .map(|x| x.split_whitespace().filter_map(|x| x.parse::<usize>().ok()))
        .map(|mut x| (x.next().unwrap(), x.next().unwrap(), x.next().unwrap()))
        .collect::<Vec<_>>();

    (stacks, parsed_moves)
}

fn task1(stacks: &mut Stacks, instructions: &Instructions) -> String {
    for (c, from, to) in instructions.iter() {
        (0..*c).for_each(|_| {
            let value = stacks.get_mut(*from - 1).unwrap().pop().unwrap();
            stacks.get_mut(*to - 1).unwrap().push(value);
        });
    }
    stacks.iter().map(|x| x.last().unwrap()).collect()
}

fn task2(stacks: &mut Stacks, instructions: &Instructions) -> String {
    for (c, from, to) in instructions.iter() {
        let height = stacks[*from - 1].len() - c;
        let x = stacks[*from - 1].drain(height..).collect::<Vec<_>>();
        x.into_iter().for_each(|x| stacks[*to - 1].push(x));
    }
    stacks.iter().map(|x| x.last().unwrap()).collect()
}

#[cfg(test)]
mod test {
    use crate::task_handler::get_task;

    use super::{parse_task, task1};

    #[test]
    fn test_task1() {
        let (mut stacks, instructions) = parse_task(&get_task(5));
        assert_eq!(task1(&mut stacks, &instructions), "JCMHLVGMG");
    }
}
