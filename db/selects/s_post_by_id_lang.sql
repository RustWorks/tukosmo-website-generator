
CREATE OR REPLACE FUNCTION s_post_by_id_lang(
    post_id BIGINT,
    post_lang BIGINT
)

RETURNS TABLE(
    id BIGINT,
    title TEXT,
    description TEXT,
    body TEXT,
    permalink TEXT,
    author BIGINT,
    author_name TEXT,
    translator BIGINT,
    translator_name TEXT,
    date TEXT,
    date_trans TEXT,
    draft BOOL,
    deleted BOOL
)

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT
    tpi_id AS id,
    tpt_title AS title,
    tpt_description AS description,
    tpt_body AS body,
    tpt_permalink AS permalink,
    tpi_author AS author,
    b.tu_name AS author_name,
    tpt_translator AS translator,
    a.tu_name AS translator_name,
    tpi_date AS date,
    tpt_date AS date_trans,
    tpt_draft AS draft,
    tpt_deleted AS deleted
FROM t_post_ids

INNER JOIN t_post_translations
ON tpi_id = tpt_post
    AND tpt_lang = post_lang
    AND tpi_id = post_id

INNER JOIN t_users a
ON tpt_translator = a.tu_id

INNER JOIN t_users b
ON tpi_author = b.tu_id

ORDER BY tpi_date DESC
LIMIT 1

$$;
