
CREATE OR REPLACE FUNCTION err_wrong_user_email()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:9CF674E24CDC';

END;

$$;