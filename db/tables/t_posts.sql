
CREATE TABLE t_posts (

    tp_id     BIGSERIAL   PRIMARY KEY,
    tp_date   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    tp_author BIGINT      NOT NULL
                          REFERENCES t_users ON DELETE CASCADE

);

