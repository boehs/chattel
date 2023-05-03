-- Your SQL goes here

CREATE TABLE items (
    id INTEGER PRIMARY KEY NOT NULL,
    when_type TEXT,
    when_date DATE,
    deadline DATETIME,
    parent INT REFERENCES items(id),
    title TEXT NOT NULL,
    body TEXT,
    item_type TEXT NOT NULL,
    item_status TEXT NOT NULL
);
