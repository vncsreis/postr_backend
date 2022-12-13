ALTER TABLE posts
    ADD user_id uuid NOT NULL;

ALTER TABLE posts
    ADD CONSTRAINT user_fk FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;

