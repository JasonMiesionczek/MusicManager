-- Your SQL goes here
CREATE TABLE tasks (
    `id` INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    `name` VARCHAR(50) NULL,
    `status` INT NOT NULL,
    `task_type` INT NOT NULL
);