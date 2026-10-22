-- project_tags had no uniqueness guarantee, so re-attaching a tag stored the
-- same (project_id, tag_id) row again and every tag list showed the duplicate.
-- Collapse existing duplicates before enforcing the invariant.
DELETE FROM project_tags
WHERE id NOT IN (SELECT MIN(id) FROM project_tags GROUP BY project_id, tag_id);

CREATE UNIQUE INDEX IF NOT EXISTS idx_project_tags_project_tag
    ON project_tags (project_id, tag_id);
