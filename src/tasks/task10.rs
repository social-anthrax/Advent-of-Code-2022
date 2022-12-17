use std::fmt::Display;

use crate::task_handler::get_task;

pub fn tasks() -> i32 {
    let input = get_task(10);
    priv_tasks(&input)
}

fn priv_tasks(input: &str) -> i32 {
    let mut cycle_sum = 0;
    let mut display = [' '; 6 * 40];
    let mut x_reg = 1;
    let mut cycle = 1;
    let to_draw = |cycle: i32, x_reg: i32| ((cycle - 1) % 40).abs_diff(x_reg) <= 1;
    let mut run_cycle = |x_reg| {
        if (cycle - 20 >= 0) && (cycle - 20) % 40 == 0 {
            let curr = x_reg * cycle;
            cycle_sum += curr;
        }
        display[cycle as usize - 1] = [' ', '#'][usize::from(to_draw(cycle, x_reg))];
        cycle += 1;
    };
    for instruction in input.lines() {
        match instruction {
            "noop" => run_cycle(x_reg),
            add_x => {
                run_cycle(x_reg);
                run_cycle(x_reg);
                x_reg += add_x[5..].parse::<i32>().unwrap();
            }
        }
    }
    println!("{}", cycle_sum);
    display
        .chunks(40)
        .for_each(|x| println!("{}", x.iter().collect::<String>()));
    cycle_sum
}

#[cfg(test)]
mod tests {
    use crate::tasks::task10::priv_tasks;

    #[test]
    fn test_task1() {
        let input = include_str!("../test_input/day10.1.txt");
        assert_eq!(priv_tasks(input), 13140);
    }
}
