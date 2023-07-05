ALTER TABLE image
ALTER COLUMN width TYPE integer USING (width::integer),
ALTER COLUMN height TYPE integer USING (height::integer);

ALTER TABLE episode
ALTER COLUMN website_link SET NOT NULL,
ALTER COLUMN title SET NOT NULL;

