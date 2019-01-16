-- Your SQL goes here
CREATE TABLE songs (
    id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255),
    track_num INT,
    duration INT,
    album_id INT,
    INDEX album_idx (album_id),
    FOREIGN KEY (album_id)
        REFERENCES albums(id)
        ON DELETE CASCADE
) ENGINE=INNODB;