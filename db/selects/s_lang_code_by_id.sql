
CREATE OR REPLACE FUNCTION s_lang_code_by_id(

    lang_id BIGINT

)

RETURNS TEXT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT tlc_code
FROM t_lang_codes
WHERE tlc_id = lang_id
LIMIT 1

$$;
