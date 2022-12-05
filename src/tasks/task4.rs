use crate::task_handler::get_task;

pub fn tasks() {
    let task = get_task(4);
    println!("{}", task1(&task));
    println!("{}", task2(&task));
}

fn task1(input: &str) -> usize {
    parse_task(input)
        .iter()
        .filter(|((a1, a2), (b1, b2))| (a1 >= b1 && a2 <= b2) || (b1 >= a1 && b2 <= a2))
        .count()
}

fn parse_task(input: &str) -> Vec<((usize, usize), (usize, usize))> {
    input
        .lines()
        .map(|x| {
            x.split_once(',')
                .and_then(|(a, b)| {
                    a.split_once('-')
                        .map(|(a1, a2)| {
                            (a1.parse::<usize>().unwrap(), a2.parse::<usize>().unwrap())
                        })
                        .zip(b.split_once('-').map(|(b1, b2)| {
                            (b1.parse::<usize>().unwrap(), b2.parse::<usize>().unwrap())
                        }))
                })
                .unwrap()
        })
        .collect::<Vec<_>>()
}

fn task2(input: &str) -> usize {
    parse_task(input)
        .iter()
        .filter(|((a1, a2), (b1, b2))| a1 <= b2 && a2 >= b1)
        .count()
}

#[cfg(test)]
mod tests {
    use super::task1;

    #[test]
    fn example_1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(task1(input), 2);
    }
}
