
CREATE OR REPLACE FUNCTION err_some_wrong_lang_name()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:39464FAE6EEB';

END;

$$;