
CREATE OR REPLACE FUNCTION err_wrong_post_id()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:E4E9108D630B';

END;

$$;
