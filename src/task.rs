use chrono::{NaiveDate, Utc};

#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub done: bool,
    created: NaiveDate,
    pub due_date: Option<NaiveDate>,
}

impl Task {
    pub fn new(name: String, done: bool, due_date: Option<NaiveDate>) -> Task {
        Task {
            name,
            done,
            due_date,
            created: Utc::today().naive_utc(),
        }
    }
}
