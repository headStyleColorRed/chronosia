-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    status INT NOT NULL DEFAULT 0,
    current_punch INT NULL
);

CREATE TABLE punches (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    entry VARCHAR NOT NULL,
    leave VARCHAR NULL,
    status INT NOT NULL DEFAULT 1,
    FOREIGN KEY (user_id) REFERENCES users(id)
);