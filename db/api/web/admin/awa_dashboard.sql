
CREATE TYPE "DashboardARequest" AS (
    req "AdminRequest"
);

CREATE TYPE "DashboardAResponse" AS (
    data "AdminDataDB"
);


CREATE OR REPLACE FUNCTION awa_dashboard(
    r "DashboardARequest"
)

RETURNS "DashboardAResponse"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

BEGIN

    d := s_admin_handler_data(r.req);

    IF d IS NULL THEN

        RAISE EXCEPTION 'TODO: Wrong request or user not logged in.';

    END IF;

    language_of_user := (d.lang).id;

    RETURN ROW(
        -- data
        d
    );

END;

$$;
