
CREATE OR REPLACE FUNCTION err_wrong_rpp_number()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:1632986F3CB9';

END;

$$;
