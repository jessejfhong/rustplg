-- Add migration script here
CREATE TABLE login_name (
    id TEXT NOT NULL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    remark TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE account (
    id TEXT NOT PRIMARY KEY,
    name TEXT NOT NULL,
    username TEXT NOT NULL REFERENCES login_name (username),
    password TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE license (
    id TEXT NOT PRIMARY KEY,
    name TEXT NOT NULL,
    license TEXT NOT NULL,
    remark TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE recovery_code (
    id TEXT NOT NULL PRIMARY KEY,
    for TEXT REFERENCES account (id),
    code TEXT NOT NULL,
    used INTEGER NOT NULL DEFAULT 0,
    used_at TEXT,
    created_at text NOT NULL
);

CREATE TABLE security_question (
    id TEXT NOT NULL PRIMARY KEY,
    for TEXT NOT NULL REFERENCES account (id),
    question TEXT NOT NULL,
    answer TEXT NOT NULL,
    created_at TEXT NOT NULL
);
