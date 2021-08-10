use actix_web::{get, HttpRequest, HttpResponse, Responder};

//use crate::auth::current_session::current_session;
use crate::templates::admin::website::Website;


#[get("/website")]
async fn website(
    req: HttpRequest,
) -> impl Responder {
    let lang_value: String = req.match_info().get("lang").unwrap().parse().unwrap();

    let resp;

    // '_' from _session_id is temporal
    //if let Some(_session_id) = current_session(req.clone()) {

        let html = Website {
            title: "Website - Tukosmo Admin Panel",
            lang_code: &lang_value,
        };

        resp = HttpResponse::Ok().body(html.to_string());

    // ERROR: Auth error
    /*} else {

        // TODO: Use correct language
        resp = HttpResponse::Found().header("Location", "/en/admin/login").finish()

    }*/

    resp
}

