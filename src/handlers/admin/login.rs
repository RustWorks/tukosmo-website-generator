use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::handlers::website::user_request::user_request;
use crate::i18n::t::t;
use crate::templates::admin::login::Login;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct LoginARequest {
    pub req: types::WebsiteRequest,
}

impl QueryFunction for LoginARequest {
    fn query(&self) -> &str {
        "SELECT awa_login($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct LoginAResponse {
    pub data: types::WebsiteDataDB,
}


pub async fn login(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    let user_req = user_request(req.clone(), id.clone());

    match query_db(
        LoginARequest {
            req: user_req,
        },
    ) {

        Ok(row) => {

            let q: LoginAResponse = row.get(0);

            if let Some(_user) = q.data.userd {

                let dashboard_route = "/{lang}/admin/"
                    .replace("{lang}", &q.data.lang.code);

                HttpResponse::Found()
                    .header("Location", dashboard_route)
                    .finish()

            } else {

                // Delete cookie
                id.forget();

                let html = Login {
                    title: &format!(
                        "{a} - {b}",
                        a = &t("Login [noun]", &q.data.lang.code),
                        b = &t("Tukosmo Admin Panel", &q.data.lang.code)
                    ),
                    q: &q,
                    error: &None,
                };

                HttpResponse::Ok().body(html.to_string())

            }

        },

        Err(e) => {
            println!("{}", e);
            HttpResponse::Found()
                .header("Location", "/")  // TODO
                .finish()
        },

    }

}

