#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

mod task_handler;
mod tasks;
extern crate proc_macro;
#[allow(clippy::wildcard_imports)]
use tasks::*;

fn main() {
    task1::tasks();
    task2::tasks();
    task3::tasks();
    task4::tasks();
    task5::tasks();
    task6::tasks();
    task7::tasks();
    task8::tasks();
    task9::tasks();
    task10::tasks();
    task11::tasks();
}
