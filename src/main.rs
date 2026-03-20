use serde::{Serialize, Deserialize};
use std::io;
use std::process;

#[derive(Serialize, Deserialize)]
enum Status {
    NotStarted,
    Started,
    Completed,
}

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    description: String,
    status: Status,
}

impl Task {
    fn get_status(&self) -> String {
        match self.status {
            Status::NotStarted => String::from("NS"),
            Status::Started => String::from("S"),
            Status::Completed => String::from("C"),
        }
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    clearscreen::clear().expect("Failed to clear console");
    loop {
        println!(
            "What would you like to do?
        (1) New Task
        (2) Update Task
        (3) Remove Task
        (4) Print Tasks
        (0) Exit"
        );
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input!");
        let input: u8 = match input.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };
        match input {
            1 => new_task(&mut tasks),
            2 => update_task(&mut tasks),
            3 => remove_task(&mut tasks),
            4 => print_tasks(&mut tasks),
            0 => process::exit(0),
            _ => {
                println!("Invalid input!");
                continue;
            }
        };
    }
}

fn new_task(tasks: &mut Vec<Task>) {}
fn update_task(tasks: &mut Vec<Task>) {}
fn remove_task(tasks: &mut Vec<Task>) {}
fn print_tasks(tasks: &mut Vec<Task>) {}
fn exit() {}

fn example() {
    let example_task = Task {
        name: String::from("name"),
        description: String::from("description"),
        status: Status::NotStarted,
    };

    println!(
        "{}, {}, {}",
        example_task.name,
        example_task.description,
        example_task.get_status()
    );
}
