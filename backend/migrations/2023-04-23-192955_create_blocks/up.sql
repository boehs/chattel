-- Your SQL goes here

CREATE TABLE IF NOT EXISTS timeblocks (
    id INT PRIMARY KEY NOT NULL,
    item INT REFERENCES items(id),
    start_time DATETIME,
    end_time DATETIME
);
