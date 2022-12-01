use std::{fs::read_to_string, io::Write};

use reqwest::header::COOKIE;

pub fn get_task(task_id: u8) -> String {
    std::fs::read_to_string(format!("src/task_input/task{}.txt", task_id)).map_or_else(
        |_| {
            let request = reqwest::blocking::Client::new()
                .get(format!(
                    "https://adventofcode.com/2022/day/{}/input",
                    task_id
                ))
                .header(COOKIE, read_to_string("cookie.env").unwrap())
                .send();

            request.map_or_else(
                |_| panic!("Error in file fetch."),
                |response| {
                    let mut file =
                        std::fs::File::create(format!("src/task_input/task{}.txt", task_id))
                            .unwrap();
                    let body = response.text().unwrap();
                    file.write_all(body.as_bytes()).unwrap();
                    body
                },
            )
        },
        |x| x,
    )
}
