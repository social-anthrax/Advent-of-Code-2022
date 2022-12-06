#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

mod task_handler;
mod tasks;
#[allow(clippy::wildcard_imports)]
use tasks::*;

fn main() {
    task1::tasks();
    task2::tasks();
    task3::tasks();
    task4::tasks();
    task5::tasks();
}
