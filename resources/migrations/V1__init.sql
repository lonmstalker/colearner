CREATE TABLE IF NOT EXISTS user_info
(
    id               VARCHAR(255) PRIMARY KEY,
    username         VARCHAR(255) UNIQUE                      NOT NULL,
    current_position VARCHAR(100)             DEFAULT 'START' NOT NULL,
    age              INTEGER                                  NULL,
    experience_years INTEGER                                  NOT NULL,
    biography        VARCHAR(500)                             NULL,
    icon             BYTEA                                    NULL,
    data             JSONB                                    NOT NULL,
    first_name       VARCHAR(255)                             NULL,
    last_name        VARCHAR(255)                             NULL,
    created_date     TIMESTAMP WITH TIME ZONE DEFAULT now()   NOT NULL
);

CREATE TABLE IF NOT EXISTS user_project
(
    id           SERIAL PRIMARY KEY,
    user_id      VARCHAR(255) REFERENCES user_info NOT NULL,
    name         VARCHAR(255)                      NOT NULL,
    description  VARCHAR(500)                      NOT NULL,
    achievements VARCHAR(255)                      NULL
);

CREATE TABLE IF NOT EXISTS proposed_project
(
    id           SERIAL PRIMARY KEY,
    author_id    VARCHAR(255) REFERENCES user_info      NOT NULL,
    title        VARCHAR(255) UNIQUE                    NOT NULL,
    description  VARCHAR(500)                           NOT NULL,
    data         JSONB                                  NOT NULL,
    status       INT                                    NOT NULL,
    type         INT                                    NOT NULL,
    created_date TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL
);

CREATE INDEX user_info_data_gin_idx ON user_info USING gin (data);
CREATE INDEX proposed_project_data_gin_idx ON proposed_project USING gin (data);