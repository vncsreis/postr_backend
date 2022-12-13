CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS posts (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    content text NOT NULL,
    deleted boolean NOT NULL DEFAULT FALSE,
    edited boolean NOT NULL DEFAULT FALSE,
    created_at timestamp NOT NULL DEFAULT NOW(),
    last_updated_at timestamp NOT NULL DEFAULT NOW()
);

