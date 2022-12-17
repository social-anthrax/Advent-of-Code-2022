use crate::task_handler::get_task;

pub fn tasks() {
    let input = get_task(8);
    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

fn scan_line(state: &mut u8, height: u8, already_vis: bool) -> (u8, bool) {
    let vis = if height > *state {
        *state = height;
        true
    } else {
        already_vis
    };
    (height, vis)
}

fn task1(input: &str) -> usize {
    let len = input.lines().next().unwrap().chars().count();
    #[allow(clippy::nursery)]
    let mut iters: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c as u8)
                .scan(10, |state, c| Some(scan_line(state, c, false)))
                .collect::<Vec<_>>()
        })
        .map(|line| {
            line.iter()
                .rev()
                .scan(10, |max_height, (height, vis)| {
                    Some(scan_line(max_height, *height, *vis))
                })
                .collect::<Vec<_>>()
        })
        .map(std::iter::IntoIterator::into_iter)
        .collect();

    // Transpose
    let grid = (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|line| {
            line.iter()
                .scan(10, |max_height, (height, vis)| {
                    Some(scan_line(max_height, *height, *vis))
                })
                .collect::<Vec<_>>()
        })
        .map(|line| {
            line.iter()
                .rev()
                .scan(10, |max_height, (height, vis)| {
                    Some(scan_line(max_height, *height, *vis))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    grid.iter()
        .map(|x| x.iter().filter(|(_, vis)| *vis).count())
        .sum()
}

fn count<'a, I>(input: I, height: u8) -> usize
where
    I: Iterator<Item = &'a u8>,
{
    let mut x = 0;
    for (i, val) in input.enumerate() {
        if val >= &height {
            return i + 1;
        }
        x += 1;
    }
    x
}

fn task2(input: &str) -> usize {
    let mut max = 0;
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();
    for (i, row) in grid.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            let height = *char;
            let l_iter = count(row.iter().take(j).rev(), height);
            let r_iter = count(row.iter().skip(j + 1), height);

            let vert = (0..row.len()).map(|i| grid[i][j]).collect::<Vec<_>>();
            let up = count(vert.iter().take(i).rev(), height);
            let down = count(vert.iter().skip(i + 1), height);

            let score = l_iter * r_iter * up * down;
            if score > max {
                max = score;
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::{task1, task2};

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_task1() {
        assert_eq!(task1(INPUT), 21);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(INPUT), 8);
    }
}
