-- Your SQL goes here

CREATE TABLE IF NOT EXISTS timeblocks (
    id INTEGER PRIMARY KEY NOT NULL,
    item INT REFERENCES items(id),
    start_time DATETIME,
    end_time DATETIME
);
