CREATE TABLE IF NOT EXISTS users (
    id TEXT NOT NULL PRIMARY KEY, -- UUID
    username TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    deleted_at TEXT DEFAULT NULL -- DATETIME
);

CREATE TABLE IF NOT EXISTS lists (
    id TEXT NOT NULL PRIMARY KEY, -- UUID
    title TEXT NOT NULL,
    created_at TEXT NOT NULL, -- DATETIME
    created_by TEXT NOT NULL, -- UUID
    deleted_at TEXT DEFAULT NULL -- DATETIME
);

CREATE TABLE IF NOT EXISTS lists_users (
    list_id TEXT NOT NULL, -- UUID
    user_id TEXT NOT NULL, -- UUID
    PRIMARY KEY (list_id, user_id)
);

CREATE TABLE IF NOT EXISTS tasks (
    id TEXT NOT NULL PRIMARY KEY, -- UUID
    list_id TEXT NOT NULL, -- UUID
    title TEXT NOT NULL,
    due_date TEXT NOT NULL, -- DATETIME
    created_at TEXT NOT NULL, -- DATETIME
    created_by TEXT NOT NULL, -- UUID
    completed_at TEXT DEFAULT NULL, -- DATETIME
    completed_by TEXT DEFAULT NULL -- UUID
);

-- CREATE TABLE IF NOT EXISTS sessions (
--     id TEXT NOT NULL PRIMARY KEY, -- UUID
--     user_id TEXT NOT NULL, -- UUID
--     created_at TEXT NOT NULL, -- DATETIME
--     expires_at TEXT NOT NULL -- DATETIME
-- );
