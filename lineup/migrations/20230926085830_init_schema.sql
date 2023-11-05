-- Add migration script here
CREATE TABLE account(
    id UUID NOT NULL,
    username VARCHAR(256) NOT NULL UNIQUE,
    password text NOT NULL,
    email VARCHAR(256) NOT NULL UNIQUE,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL,
    closed BOOLEAN NOT NULL DEFAULT FALSE,
    closed_at TIMESTAMPTZ,
    PRIMARY KEY (id)
);

CREATE TABLE personnel(
    id UUID NOT NULL,
    username VARCHAR(256) NOT NULL,
    password text NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (id) REFERENCES account(id)
);

CREATE TABLE queue(
    id UUID NOT NULL,
    name VARCHAR(256) NOT NULL,
    prefix VARCHAR(3) NOT NULL DEFAULT '',
    owned_by UUID NOT NULL,
    created_by VARCHAR(256) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (owned_by) REFERENCES account(id)
);

CREATE TABLE queue_instance(
    id UUID NOT NULL,
    queue_id UUID NOT NULL,
    head INTEGER NOT NULL DEFAULT 0,
    tail INTEGER NOT NULL DEFAULT 0,
    created_by VARCHAR(256) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (queue_id) REFERENCES queue(id)
);
