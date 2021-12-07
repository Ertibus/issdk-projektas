use actix_identity::Identity;
use actix_web::HttpResponse;
use actix_web::http::{StatusCode};
use actix_web::{error, web, get, Result};

pub mod auth;

#[get("/")]
pub async fn index(
    id: Identity,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse> {
    let mut ctx = tera::Context::new();

    if let Some(id) = id.identity() {
        ctx.insert("is_loggedin", &true);
        ctx.insert("username", &id);
    } else {
        ctx.insert("is_loggedin", &false);
        ctx.insert("username", "");
    }

    let body = tmpl.render("index.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::build(StatusCode::OK)
       .content_type("text/html; charset=utf-8")
    .body(body))

}
