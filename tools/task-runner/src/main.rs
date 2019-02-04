use data::{
    models::*,
    repos::{Repository, TaskRepository},
};
use dotenv::dotenv;
use log::info;
use std::{thread, time::Duration};

fn main() {
    dotenv().ok();
    simple_logger::init().unwrap();
    let task_repo = TaskRepository::new();
    let pool = data::get_pool();
    loop {
        let tasks = task_repo.find_by(core::map! {"status" => "pending"}, &pool);
        info!("Found {} tasks", tasks.len());
        for task in tasks {
            match task.status {
                TaskStatus::Pending => {
                    info!("Processing task {}: {}", task.id, task);
                    task_runner::do_task(task, &pool);
                }
                _ => {}
            }
        }
        info!("Sleeping one minute");
        thread::sleep(Duration::from_secs(60));
    }
}
