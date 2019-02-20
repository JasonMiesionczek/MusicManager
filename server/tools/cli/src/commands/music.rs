use core::services::youtube::YoutubeService;
use data::{
    models::*,
    repos::{Repository, TaskRepository},
};
use dotenv;
use mysql as my;
use prettytable::{cell, format, row, Table};

fn get_pool() -> my::Pool {
    let url = dotenv::var("DATABASE_URL").expect("could not find DATABASE_URL");
    let pool = my::Pool::new(url).unwrap();
    pool
}

pub fn search_command(artist_name: &str) {
    let ms = YoutubeService::new();
    if let Ok(albums) = ms.get_album_data(artist_name) {
        let mut table = Table::new();
        let format = format::FormatBuilder::new()
            .column_separator('|')
            .borders('|')
            .separators(
                &[format::LinePosition::Top, format::LinePosition::Bottom],
                format::LineSeparator::new('-', '+', '+', '+'),
            )
            .padding(1, 1)
            .build();
        table.set_format(format);

        table.set_titles(row![bFg -> "Album", bFg -> "Year"]);
        for album in &albums {
            table.add_row(row![album.name, album.year]);
        }

        table.printstd();
    }
}

pub fn queue_download(artist_name: &str) {
    let task_repo = TaskRepository {};
    let pool = get_pool();
    let mut task = Task::new(TaskType::GetAlbumData(artist_name.to_string()));
    task_repo.create(&mut task, &pool).unwrap();
}
