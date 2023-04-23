-- Your SQL goes here

CREATE TABLE items (
    id INT PRIMARY KEY NOT NULL,
    when_type INT,
    when_date DATE,
    deadline DATETIME,
    parent INT REFERENCES items(id),
    title TEXT NOT NULL,
    body TEXT,
    item_type INT NOT NULL,
    item_status INT NOT NULL
);
