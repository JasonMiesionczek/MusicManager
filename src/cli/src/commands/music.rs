use core::services::music::MusicService;
use prettytable::{cell, format, row, Table};

pub fn search_command(artist_name: &str) {
    let ms = MusicService::new();
    let albums = ms.get_album_data(artist_name);
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
    //table.add_row(row!["Value 1", "Value 2"]);
    //table.add_row(row!["Value three", "Value four"]);
    table.printstd();
}
