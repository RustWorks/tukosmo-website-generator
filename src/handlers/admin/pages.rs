use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::i18n::error_admin_route::error_admin_route;
use crate::templates::admin::pages::Pages;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct PagesARequest {
    pub req: types::AdminRequest,
}

impl QueryFunction for PagesARequest {
    fn query(&self) -> &str {
        "SELECT awa_pages($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct PagesAResponse {
    pub data: types::AdminDataDB,
}


pub async fn pages(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            PagesARequest {
                req: user_req.clone(),
            },
        ) {

            Ok(row) => {

                let q: PagesAResponse = row.get(0);

                let html = Pages {
                    title: &format!(
                        "{a} - {b}",
                        a = &t("Pages", &q.data.lang.code),
                        b = &t("Tukosmo Admin Panel", &q.data.lang.code)
                    ),
                    q: &q,
                };

                HttpResponse::Ok().body(html.to_string())

            },

            Err(e) => error_admin_route(e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

