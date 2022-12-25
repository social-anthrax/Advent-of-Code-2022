#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

mod task_handler;
mod tasks;
extern crate proc_macro;
use std::collections::HashMap;

#[allow(clippy::wildcard_imports)]
use tasks::*;

type Callback = Box<dyn Fn()>;

fn main() {
    let x: Vec<(u8, Callback)> = vec![
        (1, Box::new(task1::tasks)),
        (2, Box::new(task2::tasks)),
        (3, Box::new(task3::tasks)),
        (4, Box::new(task4::tasks)),
        (5, Box::new(task5::tasks)),
        (6, Box::new(task6::tasks)),
        (7, Box::new(task7::tasks)),
        (8, Box::new(task8::tasks)),
        (9, Box::new(task9::tasks)),
        (10, Box::new(task10::tasks)),
        (11, Box::new(task11::tasks)),
        (12, Box::new(task12::tasks)),
        (13, Box::new(task13::tasks)),
    ];

    let functions = x.into_iter().collect::<HashMap<u8, Callback>>();
    for (day, func) in &functions {
        println!("\x1b[93mDay {day} \x1b[0m");
        func();
    }
}
