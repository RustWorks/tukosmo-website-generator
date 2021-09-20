use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;

use crate::handlers::admin::admin_handler::admin_handler;
use crate::i18n::t::t;
use crate::templates::admin::users::Users;


pub async fn users(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match admin_handler(req, id) {

        Ok(data) => {

            let html = Users {
                title: &format!(
                    "{a} - {b}",
                    a = &t("Users", &data.lang.code),
                    b = &t("Tukosmo Admin Panel", &data.lang.code)
                ),
                data: &data,
            };

            HttpResponse::Ok().body(html.to_string())

        }

        Err(r) => {r}

    }

}

