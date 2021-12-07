CREATE TABLE user(
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    is_admin INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS article(
    id INTEGER PRIMARY KEY,
    owner_id INTEGER,
    document_path TEXT NOT NULL
);

INSERT OR IGNORE INTO user(id, username, password, is_admin)
VALUES (0, 'root', 'toor', 1);
