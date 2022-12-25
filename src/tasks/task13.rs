use std::fmt::{Debug, Display};

use chumsky::{prelude::Simple, primitive::just, recursive::recursive, text, Parser};

use crate::task_handler::get_task;

pub fn tasks() {
    let input = get_task(13);
    let input = Input::parser().parse(input).unwrap();
    println!("{}", task1(&input));
    println!("{}", task2(&input));
}
fn task1(input: &Input) -> usize {
    input
        .pairs
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a < b)
        .map(|(x, _)| x + 1)
        .sum()
}

fn task2(input: &Input) -> usize {
    let (mut l, mut r): (Vec<Packet>, Vec<Packet>) = input.pairs.iter().cloned().unzip();
    l.append(&mut r);
    let marker_1 = Packet::parser().parse("[[2]]").unwrap();
    let marker_2 = Packet::parser().parse("[[6]]").unwrap();
    l.push(marker_1.clone());
    l.push(marker_2.clone());
    l.sort();
    (l.binary_search(&marker_1).unwrap() + 1) * (l.binary_search(&marker_2).unwrap() + 1)
}

#[derive(PartialEq, Clone, Eq)]
enum Packet {
    Item(isize),
    List(Vec<Self>),
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Item(x) => write!(f, "{x}"),
            Self::List(content) => {
                write!(
                    f,
                    "[{}]",
                    content
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}

impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Packet {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        recursive(|tree| {
            tree.separated_by(just(','))
                .delimited_by(just('['), just(']'))
                .map(Packet::List)
                .or(text::int(10)
                    .from_str::<isize>()
                    .unwrapped()
                    .map(Packet::Item))
        })
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Item(value1), Self::Item(value2)) => value1.partial_cmp(value2),
            (Self::Item(value), Self::List(values)) => vec![Self::Item(*value)].partial_cmp(values),
            (Self::List(values), Self::Item(value)) => {
                values.partial_cmp(&vec![Self::Item(*value)])
            }
            (Self::List(values1), Self::List(values2)) => values1.partial_cmp(values2),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone)]
struct Input {
    pairs: Vec<(Packet, Packet)>,
}

impl Input {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        Packet::parser()
            .then_ignore(just("\n"))
            .then(Packet::parser())
            .separated_by(just("\n\n"))
            .map(|pairs| Self { pairs })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use super::{task1, task2, Input};

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() {
        let x = Input::parser().parse(INPUT).unwrap();
        assert_eq!(task1(&x), 13);
    }

    #[test]
    fn test_part2() {
        let x = Input::parser().parse(INPUT).unwrap();
        assert_eq!(task2(&x), 140);
    }
}
