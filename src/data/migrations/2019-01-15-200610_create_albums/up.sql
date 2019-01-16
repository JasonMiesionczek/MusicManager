-- Your SQL goes here
CREATE TABLE albums (
    id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255),
    year VARCHAR(4),
    artist_id INT NOT NULL,
    INDEX artist_idx (artist_id),
    FOREIGN KEY (artist_id)
        REFERENCES artists(id)
        ON DELETE CASCADE
) ENGINE=INNODB;