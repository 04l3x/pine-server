CREATE TABLE IF NOT EXISTS git.record
(
    id uuid NOT NULL,
    name text COLLATE pg_catalog."default" NOT NULL,
    created_at timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    visibility text COLLATE pg_catalog."default" NOT NULL,
    owner_id uuid NOT NULL,
    CONSTRAINT record_pkey PRIMARY KEY (id),
    CONSTRAINT record_name_key UNIQUE (name),
    CONSTRAINT owner_id FOREIGN KEY (owner_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
)

TABLESPACE pg_default;

ALTER TABLE git.record
    OWNER to dev;

COMMENT ON TABLE git.record
    IS 'registro de todos los repositorios alojados';
