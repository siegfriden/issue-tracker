CREATE TABLE comments (
    id         UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    issue_id   UUID        NOT NULL REFERENCES issues(id) ON DELETE CASCADE,
    author_id  UUID        NOT NULL REFERENCES users(id),
    body       TEXT        NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_comments_issue_id  ON comments (issue_id);
CREATE INDEX idx_comments_author_id ON comments (author_id);
