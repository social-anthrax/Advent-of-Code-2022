use std::collections::HashSet;

use crate::task_handler::get_task;

pub fn tasks() {
    let input = get_task(6);
    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

fn task1(input: &str) -> usize {
    find_first_unique_sub(input, 4)
}

fn task2(input: &str) -> usize {
    find_first_unique_sub(input, 14)
}

fn find_first_unique_sub(input: &str, window_size: usize) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(window_size)
        .enumerate()
        .find(|(_, chars)| {
            let mut uniq = HashSet::new();
            chars.iter().all(move |x| uniq.insert(x))
        })
        .unwrap()
        .0
        + window_size
    // as zero index and we're getting the index of the first item, not hte last
}

#[cfg(test)]
mod tests {
    use super::{task1, task2};

    #[test]
    fn test_part1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(task1(input), 5);
    }

    #[test]
    fn test_part2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(task2(input), 19);
    }
}
