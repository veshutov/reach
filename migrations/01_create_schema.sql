CREATE TYPE social_type AS ENUM (
    'Telegram'
);

CREATE TABLE social (
    id bigserial,
    social_type social_type,
    created timestamp,
    updated timestamp
);

ALTER SEQUENCE social_id_seq RESTART WITH 1000;