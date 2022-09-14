use crate::task::Task;

pub trait TaskRepo {
    /// Adds a task
    fn add(&mut self, task: Task);

    /// Lists all tasks
    fn list(&self) -> Vec<Task>;
}
