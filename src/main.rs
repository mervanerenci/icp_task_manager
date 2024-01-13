fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    let task1 = add_task("Task 1", &mut tasks);
    let task2 = add_task("Task 2", &mut tasks);

    complete_task(task1.id, &mut tasks);
    complete_task(3, &mut tasks);

    list_tasks(&tasks);
    
    complete_task(task2.id, &mut tasks);
    list_tasks(&tasks);
}

#[derive(Clone)]
struct Task {
    id: i32,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: i32, description: String) -> Task {
        Task {
            id,
            description,
            completed: false,
        }
    }
}

fn add_task(description: &str, tasks: &mut Vec<Task>) -> Task {
    let id = tasks.len() as i32 + 1;
    let task = Task::new(id, description.to_string());
    tasks.push(task.clone());
    task
}

fn complete_task(id: i32, tasks: &mut Vec<Task>) -> Option<&Task> {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        Some(task)
    } else {
        None
    }
}

fn list_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        println!("ID: {}, Description: {}, Completed: {}", task.id, task.description, task.completed);
    }
}

