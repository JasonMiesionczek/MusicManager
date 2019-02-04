use crate::models::*;
use crate::repos::Repository;
use mysql as my;
use mysql::params;
use uuid;

pub struct TaskRepository {}

impl TaskRepository {
    pub fn new() -> TaskRepository {
        TaskRepository {}
    }
}

impl Repository for TaskRepository {
    type Item = Task;

    fn select_query(&self) -> String {
        String::from("SELECT id, status, task_type, external_id, task_data FROM tasks")
    }

    fn insert_query(&self) -> String {
        String::from("INSERT INTO tasks (status, task_type, external_id, task_data) VALUES (:status, :task_type, :external_id, :task_data)")
    }

    fn create<'a>(
        &self,
        item: &'a mut Self::Item,
        pool: &my::Pool,
    ) -> Result<&'a mut Self::Item, String> {
        let mut stmt = pool.prepare(self.insert_query()).unwrap();
        let exid = uuid::Uuid::new_v4().to_string();

        let task_data: String = match &item.task_type {
            TaskType::GetArtistImage(artist_name) => artist_name.to_string(),
            TaskType::GetAlbumData(artist_name) => artist_name.to_string(),
            TaskType::GetSongData(album_meta) | TaskType::GetAlbumImage(album_meta) => {
                serde_json::to_string(&album_meta).unwrap()
            }
            TaskType::DownloadSong(song_meta) => serde_json::to_string(&song_meta).unwrap(),
        };

        let result = stmt.execute(params! {
            "status" => &item.status.to_string(),
            "task_type" => &item.task_type.to_string(),
            "external_id" => exid.clone(),
            "task_data" => task_data.as_str(),
        });

        match result {
            Err(err) => Err(err.to_string()),
            Ok(_) => {
                item.external_id = exid;
                Ok(item)
            }
        }
    }
}
