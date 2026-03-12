CREATE TABLE projects (
    id          UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    name        VARCHAR(200) NOT NULL,
    identifier  VARCHAR(100) NOT NULL UNIQUE,
    description TEXT         NOT NULL DEFAULT '',
    is_public   BOOLEAN      NOT NULL DEFAULT true,
    owner_id    UUID         NOT NULL REFERENCES users(id),
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE INDEX idx_projects_owner_id  ON projects (owner_id);
CREATE INDEX idx_projects_is_public ON projects (is_public);
