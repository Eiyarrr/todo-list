use chrono::{ Local, NaiveDate };

enum Status {
    NotStarted,
    Started,
    Completed,
}

struct Task {
    name: String,
    description: String,
    due_date: NaiveDate,
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
    example();
}

fn example() {
    let example_task = Task {
        name: String::from("name"),
        description: String::from("description"),
        due_date: Local::now().date_naive(),
        status: Status::NotStarted,
    };

    println!(
        "{}, {}, {}, {}",
        example_task.name,
        example_task.description,
        example_task.due_date.to_string(),
        example_task.get_status()
    );
}
