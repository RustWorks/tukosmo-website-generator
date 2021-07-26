
CREATE OR REPLACE FUNCTION s_user_id_by_email(

    email_value TEXT

)

RETURNS BIGINT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT tu_id
FROM t_users
WHERE tu_email = email_value
LIMIT 1

$$;
