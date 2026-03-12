CREATE TABLE project_members (
    project_id UUID        NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    user_id    UUID        NOT NULL REFERENCES users(id)    ON DELETE CASCADE,
    role       VARCHAR(20) NOT NULL DEFAULT 'viewer'
                   CHECK (role IN ('admin', 'member', 'viewer')),
    joined_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (project_id, user_id)
);

-- The composite PK index covers queries on the leading column (project_id).
-- A separate index is needed for isolated queries on the trailing column (user_id).
CREATE INDEX idx_project_members_user_id ON project_members (user_id);
