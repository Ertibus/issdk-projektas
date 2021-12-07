use actix_identity::Identity;
use actix_web::HttpResponse;
use actix_web::http::{StatusCode};
use actix_web::{error, web, get, Result};

pub mod auth;
pub mod dashboard;

#[get("/")]
pub async fn index(
    id: Identity,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse> {
    let mut ctx = tera::Context::new();

    ctx.insert("is_loggedin", &(None != id.identity()));

    let body = tmpl.render("index.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::build(StatusCode::OK)
       .content_type("text/html; charset=utf-8")
    .body(body))
}
