CREATE TABLE project_issue_counters (
    project_id     UUID    PRIMARY KEY REFERENCES projects(id) ON DELETE CASCADE,
    current_number INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE issues (
    id                   UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id           UUID        NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    project_issue_number INTEGER     NOT NULL,
    author_id            UUID        NOT NULL REFERENCES users(id),
    assignee_id          UUID        REFERENCES users(id),
    subject              TEXT        NOT NULL,
    description          TEXT        NOT NULL DEFAULT '',
    issue_type           TEXT        NOT NULL DEFAULT 'task'
                             CHECK (issue_type IN ('bug', 'feature', 'task', 'support')),
    status               TEXT        NOT NULL DEFAULT 'open'
                             CHECK (status IN ('open', 'in_progress', 'resolved', 'closed', 'feedback')),
    priority             TEXT        NOT NULL DEFAULT 'normal'
                             CHECK (priority IN ('low', 'normal', 'high', 'urgent', 'immediate')),
    due_date             DATE,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (project_id, project_issue_number)
);

CREATE INDEX idx_issues_project_id  ON issues (project_id);
CREATE INDEX idx_issues_author_id   ON issues (author_id);
CREATE INDEX idx_issues_assignee_id ON issues (assignee_id);
CREATE INDEX idx_issues_status      ON issues (status);
CREATE INDEX idx_issues_priority    ON issues (priority);
