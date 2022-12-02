#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

mod task_handler;
mod tasks;
use tasks::*;

fn main() {
    task1::tasks();
    task2::tasks();
}
