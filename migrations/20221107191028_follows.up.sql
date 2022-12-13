CREATE TABLE follows (
    following_id uuid NOT NULL,
    followed_id uuid NOT NULL,
    PRIMARY KEY (following_id, followed_id),
    CONSTRAINT fk_following FOREIGN KEY (following_id) REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT fk_followed FOREIGN KEY (followed_id) REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE
);

