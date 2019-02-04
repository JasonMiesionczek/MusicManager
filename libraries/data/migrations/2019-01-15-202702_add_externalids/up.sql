-- Your SQL goes here
ALTER TABLE artists ADD COLUMN external_id VARCHAR(36);
ALTER TABLE albums ADD COLUMN external_id VARCHAR(36);
ALTER TABLE songs ADD COLUMN external_id VARCHAR(36);