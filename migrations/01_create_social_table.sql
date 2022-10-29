CREATE TYPE social_type AS ENUM (
    'telegram'
);

CREATE TABLE social (
    id bigserial PRIMARY KEY,
    social_type social_type,
    created_at timestamp,
    created_by bigserial,
    updated_at timestamp,
    updated_by bigserial
);

ALTER SEQUENCE social_id_seq RESTART WITH 1000;