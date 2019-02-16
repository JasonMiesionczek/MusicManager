use barrel::backend::MySql;
use barrel::{types::*, Migration};

pub fn get_migrations() -> Vec<String> {
    let migrations = vec![
        migration(|m| {
            m.create_table("artists", |t| {
                t.add_column("name", varchar(255));
                t.add_column("external_id", varchar(36));
            });
        }),
        migration(|m| {
            m.create_table("albums", |t| {
                t.add_column("name", varchar(255));
                t.add_column("image", varchar(2048));
                t.add_column("year", integer());
                t.add_column("artist_id", integer());
                t.add_column("external_id", varchar(255));
                let artist_fk = ForeignKey {
                    child_column: "artist_id",
                    parent_table: "artists",
                    parent_column: "id",
                    actions: vec![ForeignKeyAction::Delete(ForeignKeyOption::Cascade)],
                };
                t.add_column("", foreign_key(artist_fk));
            });
        }),
        migration(|m| {
            m.create_table("songs", |t| {
                t.add_column("name", varchar(255));
                t.add_column("duration", integer());
                t.add_column("track", integer());
                t.add_column("album_id", integer());
                t.add_column("external_id", varchar(255));
                let album_fk = ForeignKey {
                    child_column: "album_id",
                    parent_table: "albums",
                    parent_column: "id",
                    actions: vec![ForeignKeyAction::Delete(ForeignKeyOption::Cascade)],
                };
                t.add_column("", foreign_key(album_fk));
            });
        }),
        migration(|m| {
            m.create_table("tasks", |t| {
                t.add_column("name", varchar(255));
            });
        }),
        migration(|m| {
            m.change_table("tasks", |t| {
                t.add_column("status", varchar(255));
                t.add_column("task_type", varchar(255));
                t.add_column("external_id", varchar(36));
            });
        }),
        migration(|m| {
            m.change_table("tasks", |t| {
                t.add_column("task_data", text());
            });
        }),
        migration(|m| {
            m.change_table("tasks", |t| {
                t.drop_column("name");
            });
        }),
        migration(|m| {
            m.change_table("songs", |t| {
                t.add_column("filename", varchar(255));
            });
        }),
        migration(|m| {
            m.create_table("playlists", |t| {
                t.add_column("name", varchar(255));
            });
        }),
        migration(|m| {
            m.create_table("playlists_songs", |t| {
                t.add_column("playlist_id", integer());
                t.add_column("song_id", integer());
                let playlist_fk = ForeignKey {
                    child_column: "playlist_id",
                    parent_table: "playlists",
                    parent_column: "id",
                    actions: vec![ForeignKeyAction::Delete(ForeignKeyOption::Cascade)],
                };
                t.add_column("playlist_fk", foreign_key(playlist_fk));
                let song_fk = ForeignKey {
                    child_column: "song_id",
                    parent_table: "songs",
                    parent_column: "id",
                    actions: vec![ForeignKeyAction::Delete(ForeignKeyOption::Cascade)],
                };
                t.add_column("song_fk", foreign_key(song_fk));
            });
        }),
    ];

    migrations
}

fn migration<M: 'static>(m: M) -> String
where
    M: Fn(&mut Migration),
{
    let mut migration = Migration::new();

    m(&mut migration);

    migration.make::<MySql>()
}
