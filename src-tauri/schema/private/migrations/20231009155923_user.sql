-- Add migration script here
CREATE TABLE IF NOT EXISTS user (
    email varchar (120) DEFAULT '',
    token text DEFAULT '',
    PRIMARY KEY (email)
);