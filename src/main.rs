use std::io;
use std::process;

enum Status {
    NotStarted,
    Started,
    Completed,
}

impl Status {
    fn next_status(&self) -> Status {
        match self {
            Status::NotStarted => Status::Started,
            Status::Started => Status::Completed,
            Status::Completed => Status::Completed,
        }
    }
}

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

fn new_task(tasks: &mut Vec<Task>) {
    clearscreen::clear().expect("Failed to clear console");
    println!("What is the name of your new task?");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("Invalid input!");
    let name = name.trim().to_string();

    println!("What is the description of your new task?");
    let mut description: String = String::new();
    io::stdin().read_line(&mut description).expect("Invalid input!");
    let description = description.trim().to_string();

    let status: Status = Status::NotStarted;
    tasks.push(Task {
        name,
        description,
        status,
    })
}

fn update_task(tasks: &mut Vec<Task>) {
    clearscreen::clear().expect("Failed to clear console");
    println!("What task would you like to update?");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input!");
    let input = input.trim();

    if let Some(pos) = tasks.iter().position(|t| t.name == input) {
        let previous_status = tasks[pos].get_status();
        tasks[pos].status = tasks[pos].status.next_status();
        println!(
            "Updated task {} from {} to {}",
            input,
            previous_status,
            tasks[pos].get_status()
        );
    } else {
        println!("Invalid task");
    }
}

fn remove_task(tasks: &mut Vec<Task>) {
    clearscreen::clear().expect("Failed to clear console");
    println!("What task would you like to remove?");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input!");
    let input = input.trim();

    if let Some(pos) = tasks.iter().position(|t| t.name == input) {
        tasks.remove(pos);
        println!("Removed task {}", input);
    } else {
        println!("Invalid task");
    }
}

fn print_tasks(tasks: &mut Vec<Task>) {
    for task in tasks {
        println!(
            "name: {}, description: {}, status: {}",
            task.name,
            task.description,
            task.get_status()
        );
    }
}
