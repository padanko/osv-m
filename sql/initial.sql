CREATE TABLE osvm_topics (
    topic_title TEXT NOT NULL,
    topic_id TEXT NOT NULL,
    topic_password VARCHAR(32),
    topic_default_name VARCHAR(10),
    bbs_id TEXT NOT NULL
);

CREATE TABLE osvm_posts (
    post_name TEXT NOT NULL,
    user_id TEXT NOT NULL,
    date_time TIMESTAMP NOT NULL,
    body TEXT NOT NULL,
    bbs_id TEXT NOT NULL,
    topic_id TEXT NOT NULL
);

CREATE TABLE osvm_userlist (
    user_ip TEXT NOT NULL,
    vacuum BOOLEAN NOT NULL,
    user_level INTEGER NOT NULL,
    token VARCHAR(32) NOT NULL
);