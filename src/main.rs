mod in_memory_task_repo;
mod task;
mod task_repo;

use task::Task;

use crate::{in_memory_task_repo::InMemoryTaskRepo, task_repo::TaskRepo};

fn main() {
    let task = Task::new("Do the dishes".to_string(), false, None);

    let mut repo = InMemoryTaskRepo::new();

    repo.add(task);

    println!("{}", repo)
}
