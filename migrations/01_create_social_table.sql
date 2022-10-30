CREATE TYPE social_type AS ENUM (
    'telegram'
);

CREATE TABLE social (
    id bigserial PRIMARY KEY,
    social_type social_type NOT NULL,
    created_at timestamp NOT NULL,
    created_by bigserial NOT NULL,
    updated_at timestamp NOT NULL,
    updated_by bigserial NOT NULL
);
