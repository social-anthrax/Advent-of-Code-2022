use crate::task_handler::get_task;

pub fn tasks() {
    println!("{}", task1());
    println!("{}", task2());
}

static SCORE_TASK1: [[usize; 3]; 3] = [
// . r .p .s
    [3, 6, 0],
    [0, 3, 6],
    [6, 0, 3],
];

fn match_score(input: &str) -> usize {
    match input {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => panic!("Failed to parse"),
    }
}

fn task1() -> usize {
    get_task(2)
    .lines()
    .map(|line| line.split_whitespace().map(match_score).collect::<Vec<_>>())
    .fold(0, |curr, game| curr + SCORE_TASK1[game[0] -1][game[1] - 1] + game[1])
}

//--------------------------------------------


fn task2() -> usize {
     get_task(2)
        .lines()
        .map(|line| line.split_whitespace().map(match_score).collect::<Vec<_>>())
        .fold(0, |curr, game| {
            // store as (p2, result) as that what we need to get the result
            curr + 
            SCORE_TASK1[game[0] - 1]
                    .iter()
                    .position(|x| *x == ((game[1] - 1) * 3)).unwrap() + 1 +  // This gets the score of the index of the relevant move as win = 6, lose is 0 and draw is 3
                (game[1] - 1) * 3
        })
}


#[cfg(test)]
mod tests{
    use super::{task1, task2};

    #[test]
    fn test_task1_short() {
        assert_eq!(task1(), 11063);
    }
    #[test]
    fn test_task2() {
        assert_eq!(task2(), 10349);
    }

}