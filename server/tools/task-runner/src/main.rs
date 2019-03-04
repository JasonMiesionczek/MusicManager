use data::models::{Task, TaskStatus};
use data::repos::{Repository, TaskRepository, UpdateValue};
use dotenv::dotenv;
use log::info;
use std::collections::HashMap;
use std::sync::mpsc;
use std::{thread, time::Duration};
use task_runner::ThreadPool;

fn get_pending_tasks(pool: &mysql::Pool) -> Vec<Task> {
    let task_repo = TaskRepository::new();
    task_repo.find_by(core::map! {"status" => "pending"}, &pool)
}

fn update_task(task: &Task, status: TaskStatus, pool: &mysql::Pool) {
    let task_repo = TaskRepository::new();
    let mut values = HashMap::new();
    let status_string = status.to_string();
    info!("Task {}: {}", task, status_string);
    values.insert("status", UpdateValue::Str(status_string));
    task_repo.update("tasks", values, task.id, &pool);
}

fn main() {
    dotenv().ok();
    simple_logger::init().unwrap();
    let task_repo = TaskRepository::new();
    let db_pool = data::get_pool();

    let (tx, rx) = mpsc::channel();
    let pool = ThreadPool::new(8);

    thread::spawn(move || loop {
        let tasks = get_pending_tasks(&db_pool);
        info!("Found {} new tasks", tasks.len());
        for task in tasks {
            update_task(&task, TaskStatus::Queued, &db_pool);
            tx.send(task.id).unwrap();
        }
        info!("Sleeping 10 seconds");
        std::thread::sleep(Duration::from_secs(10));
    });

    let db_pool = data::get_pool();
    for id in rx {
        let task = task_repo.find_by_id(id, &db_pool).unwrap();
        pool.execute_task(task);
    }
}
