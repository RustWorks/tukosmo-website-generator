
CREATE OR REPLACE FUNCTION d_all_sessions()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    DELETE FROM t_sessions;

END;

$$;