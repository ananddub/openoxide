CREATE TABLE project_tags (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    project_id BIGINT NOT NULL,
    tag_id BIGINT NOT NULL
);

CREATE UNIQUE INDEX idx_project_tags_project_tag ON project_tags (project_id, tag_id);
