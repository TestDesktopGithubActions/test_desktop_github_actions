-- Add migration script here
CREATE TABLE IF NOT EXISTS latest_login (
    email varchar (120) DEFAULT '',
    token text DEFAULT '',
    language varchar (20) DEFAULT 'en',
    count int DEFAULT '0',
    expired int DEFAULT '0',
    PRIMARY KEY (email)
);