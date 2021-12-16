use crate::models::Article;
use crate::models::CreateArticleForm;
use actix_session::Session;
use actix_identity::Identity;
use actix_web::HttpResponse;
use actix_web::http::{StatusCode};
use actix_web::{error, web, get, post, Result};
use crate::Pool;
use crate::repo;

pub mod auth;
pub mod dashboard;

#[get("/")]
pub async fn index(
    id: Identity,
    tmpl: web::Data<tera::Tera>,
    db: web::Data<Pool>,
) -> Result<HttpResponse> {
    let pool = db.clone();
    let mut ctx = tera::Context::new();

    let articles = web::block(move || {
        let conn = pool.get().unwrap();
        repo::get_all_articles(conn)
    }).await?;

    ctx.insert("is_loggedin", &(None != id.identity()));
    ctx.insert("articles", &articles);

    let body = tmpl.render("index.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::build(StatusCode::OK)
       .content_type("text/html; charset=utf-8")
    .body(body))
}

#[get("/article/create")]
pub async fn create_article(
  _id: Identity,
  tmpl: web::Data<tera::Tera>,
  session: Session,
) -> Result<HttpResponse> {
  let mut ctx = tera::Context::new();
  ctx.insert("is_logedin", &false);

  if let Some(fail) = session.get::<String>("register_failure")? {
    ctx.insert("failed", &fail);
  } else {
    ctx.insert("failed", "");
  }

  let render = tmpl.render("post_article.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template error")).expect("Test");

  Ok(HttpResponse::build(StatusCode::OK)
      .content_type("text/html; charset=utf-8")
      .body(render))
}

#[post("/article")]
pub async fn post_new_article(
    id: Identity,
    params: web::Form<CreateArticleForm>,
    db: web::Data<Pool>,
    session: Session,
) -> HttpResponse {
    let pool = db.clone();
    let data = params.clone();

    if let Some(id) = id.identity() {
        let _res = web::block(move || {
            let conn = pool.get().unwrap();
            let user_data = Article{
                id: -1,
                owner: id,
                title: data.title,
                description: data.description,
            };
            repo::post_article(conn, user_data)
        }).await
        .map_err(|err| {
            session.set("register_failure", &err.to_string()).unwrap();
            return HttpResponse::Found().header("location", "/register").finish();
        })
        .map(|_| {
            session.set("register_failure", "").unwrap();
            return HttpResponse::Found().header("location", "/login").finish();
        });
        return HttpResponse::Found().header("location", "/login").finish();
    } else {
        return HttpResponse::Unauthorized().body("Unauthorized access");
    }
}
