-- Add migration script here

-- init public schema
CREATE SCHEMA IF NOT EXISTS public;

-- init git schema
CREATE SCHEMA IF NOT EXISTS git;

-- init type visibility
CREATE TYPE visibility AS ENUM ('Public', 'Private');

-- init users table
CREATE TABLE IF NOT EXISTS public.users
(
    id uuid NOT NULL,
    email text COLLATE pg_catalog."default" NOT NULL,
	avatar text COLLATE pg_catalog."default" NOT NULL DEFAULT 'url.png',
    username text COLLATE pg_catalog."default" NOT NULL,
    password text COLLATE pg_catalog."default" NOT NULL,
    verified boolean NOT NULL,
    created_at timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login timestamp without time zone[],
    CONSTRAINT users_pkey PRIMARY KEY (id),
    CONSTRAINT users_email_key UNIQUE (email),
    CONSTRAINT users_username_key UNIQUE (username)
)

TABLESPACE pg_default;

ALTER TABLE public.users
    OWNER to dev;

COMMENT ON TABLE public.users
    IS 'registro de todos los usuarios';

-- init record table
CREATE TABLE IF NOT EXISTS git.record
(
    id uuid NOT NULL,
    name text COLLATE pg_catalog."default" NOT NULL,
    created_at timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    owner_id uuid NOT NULL,
    description text COLLATE pg_catalog."default",
    visibility visibility NOT NULL,
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

-- init repositories table
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

