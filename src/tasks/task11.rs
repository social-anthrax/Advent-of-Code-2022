use std::{cmp::Reverse, collections::VecDeque, usize};

use crate::task_handler::get_task;

pub fn tasks() {
    let input = get_task(11);
    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

fn task1(input: &str) -> usize {
    solve::<3>(input, 20)
}

fn task2(input: &str) -> usize {
    solve::<1>(input, 10000)
}

fn solve<const DIVISOR: usize>(input: &str, iterations: usize) -> usize {
    let mut monkeys = input
        .lines()
        .collect::<Vec<_>>()
        .split(|x| x.is_empty())
        .map(Monkey::from_block)
        .collect::<Vec<_>>();
    let common_divisor = monkeys.iter().map(|x| x.conditional).product();
    (0..iterations).for_each(|_| {
        (0..monkeys.len()).for_each(|j| {
            inspect::<DIVISOR>(&mut monkeys, j, common_divisor);
        });
    });
    monkeys.sort_unstable_by_key(|x| Reverse(x.inspected));
    monkeys.iter().take(2).fold(1, |x, a| x * a.inspected)
}

fn inspect<const DIVISOR: usize>(monkeys: &mut [Monkey], i: usize, common_multiple: usize) {
    while !monkeys[i].items.is_empty() {
        monkeys[i].inspected += 1;
        let mut score = monkeys[i].items.pop_front().unwrap();
        // If the value is unparsable then it's "old" and it can be replace with score
        let (l, r) = (
            monkeys[i].operation[0].parse().unwrap_or(score),
            monkeys[i].operation[2].parse().unwrap_or(score),
        );

        score = match monkeys[i].operation[1].as_str() {
            "*" => l * r,
            "+" => l + r,
            _ => panic!("Unknown operator"),
        };
        score = (score / DIVISOR) % common_multiple;

        if score % monkeys[i].conditional == 0 {
            monkeys[monkeys[i].cond_true].items.push_back(score);
        } else {
            monkeys[monkeys[i].cond_false].items.push_back(score);
        }
    }
}

#[derive(Debug)]
struct Monkey {
    pub items: VecDeque<usize>,
    operation: Vec<String>,
    conditional: usize,
    cond_true: usize,
    cond_false: usize,
    pub inspected: usize,
}

impl Monkey {
    pub fn from_block(input: &[&str]) -> Self {
        let mut lines = input.iter().skip(1);
        Self {
            items: lines
                .next()
                .unwrap()
                .chars()
                .skip(17)
                .collect::<String>()
                .split(',')
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<VecDeque<_>>(),
            operation: lines
                .next()
                .unwrap()
                .chars()
                .skip(19)
                .collect::<String>()
                .split_whitespace()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>(),
            conditional: lines
                .next()
                .unwrap()
                .chars()
                .skip(21)
                .collect::<String>()
                .parse::<usize>()
                .unwrap(),
            cond_true: lines
                .next()
                .unwrap()
                .chars()
                .skip(29)
                .collect::<String>()
                .parse::<usize>()
                .unwrap(),
            cond_false: lines
                .next()
                .unwrap()
                .chars()
                .skip(30)
                .collect::<String>()
                .parse::<usize>()
                .unwrap(),
            inspected: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{task1, task2};

    const INPUT: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;
    #[test]
    fn test_task1() {
        assert_eq!(task1(INPUT), 10605);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(INPUT), 2_713_310_158);
    }
}
