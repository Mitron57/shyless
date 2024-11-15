CREATE TABLE Users
(
    id       BIGSERIAL UNIQUE,
    login    VARCHAR(255) UNIQUE,
    password VARCHAR(255),
    PRIMARY KEY (id, login)
);

CREATE TABLE Posts
(
    id        BIGSERIAL PRIMARY KEY,
    author_id BIGINT,
    content   TEXT,
    likes     BIGINT,
    FOREIGN KEY (author_id) REFERENCES Users (id)
)

