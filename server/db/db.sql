CREATE TABLE user(
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    is_admin INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS article(
    id INTEGER PRIMARY KEY,
    owner TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL
);

INSERT OR IGNORE INTO user(id, username, password, is_admin)
VALUES (0, 'root', 'toor', 1);

INSERT OR IGNORE INTO article(id, owner, title, description)
VALUES (0, 'root', 'Example', 'This is a test case. This part is the description');
