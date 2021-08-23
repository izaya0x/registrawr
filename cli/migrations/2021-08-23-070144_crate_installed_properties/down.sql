-- This file should undo anything in `up.sql`

CREATE TABLE IF NOT EXISTS new_dapps (
    id INTEGER NOT NULL PRIMARY KEY ,
    name VARCHAR NOT NULL,
    version VARCHAR NOT NULL
);

INSERT INTO new_dapps SELECT id, name, version FROM dapps;

DROP TABLE dapps;

ALTER TABLE new_dapps RENAME TO dapps;