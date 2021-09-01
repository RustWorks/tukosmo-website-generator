use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;

use crate::handlers::admin::admin_handler::admin_handler;
use crate::i18n::t::t;
use crate::templates::admin::new_post::NewPost;


pub async fn new_post(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match admin_handler(req, id) {

        Ok((lang_code, _user_id)) => {

            let html = NewPost {
                title: &format!(
                    "{a} - {b}",
                    a = &t("New post", &lang_code),
                    b = &t("Tukosmo Admin Panel", &lang_code)
                ),
                lang_code: &lang_code,
            };

            HttpResponse::Ok().body(html.to_string())

        }

        Err(r) => {r}

    }

}
