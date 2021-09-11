CREATE TABLE IF NOT EXISTS comments (
    id SERIAL PRIMARY KEY,
    text TEXT NOT NULL,
    post_id INTEGER NOT NULL,
    author_id INTEGER NOT NULL,
    author_username VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS rates (
    id SERIAL PRIMARY KEY,
    value INTEGER NOT NULL,
    post_id INTEGER NOT NULL,
    author_id INTEGER NOT NULL
);