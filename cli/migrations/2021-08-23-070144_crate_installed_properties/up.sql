-- Your SQL goes here

CREATE TABLE IF NOT EXISTS new_dapps (
    id INTEGER NOT NULL PRIMARY KEY ,
    name VARCHAR NOT NULL,
    version VARCHAR NOT NULL,
    install_location VARCHAR NOT NULL DEFAULT "",
    installed_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

INSERT INTO new_dapps (id, name, version) SELECT id, name, version FROM dapps;

DROP TABLE dapps;

ALTER TABLE new_dapps RENAME TO dapps;
