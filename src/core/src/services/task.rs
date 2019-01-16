use data::models::*;
use data::schema::tasks::dsl::*;
use diesel::*;
use std::error::Error;

pub struct TaskService<'a> {
    db: &'a MysqlConnection,
}

impl<'a> TaskService<'a> {
    pub fn new(db: &'a MysqlConnection) -> TaskService<'a> {
        TaskService { db }
    }

    pub fn create_task(&self, task_name: &str, tasktype: i32) -> Result<Task, Box<dyn Error>> {
        let task = Task {
            id: 0,
            name: Some(String::from(task_name)),
            task_type: tasktype,
            status: 0,
        };

        let _result = diesel::insert_into(tasks).values(&task).execute(self.db)?;
        Ok(task)
    }
}
