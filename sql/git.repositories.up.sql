CREATE TABLE IF NOT EXISTS git.repositories
(
    id uuid NOT NULL,
    tags text[] COLLATE pg_catalog."default",
    categories text[] COLLATE pg_catalog."default",
    languages text[] COLLATE pg_catalog."default",
    CONSTRAINT repositories_pkey PRIMARY KEY (id),
    CONSTRAINT id FOREIGN KEY (id)
        REFERENCES git.record (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE git.repositories
    OWNER to dev;

COMMENT ON TABLE git.repositories
    IS 'aggregate information of each repository';

