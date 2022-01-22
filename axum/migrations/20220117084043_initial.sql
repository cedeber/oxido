CREATE TABLE users
(
    userid   INTEGER NOT NULL PRIMARY KEY,               -- auto, /!\ can be reused if previously deleted, use AUTOINCREMENT to prevent that
    created  TEXT    NOT NULL DEFAULT CURRENT_TIMESTAMP, -- YYYY-MM-DD HH:MM:SS
    active   INTEGER NOT NULL DEFAULT FALSE,             -- need to validate account via email
    username TEXT    NOT NULL UNIQUE,
    email    TEXT    NOT NULL UNIQUE,
    password TEXT    NOT NULL,
    name     TEXT
)