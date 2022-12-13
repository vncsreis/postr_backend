CREATE TABLE posts_history (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    content text NOT NULL,
    version timestamp NOT NULL DEFAULT NOW(),
    post_id uuid NOT NULL,
    CONSTRAINT fk_post FOREIGN KEY (post_id) REFERENCES posts (id) ON DELETE CASCADE
);

