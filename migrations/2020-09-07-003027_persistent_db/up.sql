-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE persistent_storage (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    value_text TEXT UNIQUE NOT NULL,
    date_begin TIMESTAMP NOT NULL DEFAULT current_timestamp,
    date_end TIMESTAMP
);
