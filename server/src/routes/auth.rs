use serde_json::Value::Bool;
use serde_json::Value;
use actix_web::client::Client;
use crate::repo::register_user;
use crate::models::SlimUser;
use actix_session::Session;
use crate::repo::get_user;
use crate::Pool;
use crate::models::{LoginForm, RegisterForm};
use actix_web::{error, web, HttpResponse, Result};
use actix_web::http::StatusCode;
use actix_identity::Identity;

pub async fn login_form(
  id: Identity,
  tmpl: web::Data<tera::Tera>,
  session: Session,
) -> Result<HttpResponse> {
  if let Some(_id) = id.identity() {return Ok(HttpResponse::Found().header("location", "/").finish());}

  let mut ctx = tera::Context::new();
  ctx.insert("is_logedin", &false);

  if let Some(fail) = session.get::<String>("login_failure")? {
    ctx.insert("failed", &fail);
  } else {
    ctx.insert("failed", "");
  }

  let render = tmpl.render("login.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template error")).expect("Test");

  Ok(HttpResponse::build(StatusCode::OK)
      .content_type("text/html; charset=utf-8")
      .body(render))
}

pub async fn login(
  id: Identity,
  params: web::Form<LoginForm>,
  db: web::Data<Pool>,
  session: Session,
) -> HttpResponse {
    let pool = db.clone();
    let data = params.clone();

    let res = web::block(move || {
        let conn = pool.get().unwrap();
        get_user(conn, data.username)
    }).await
      .map_err(|err| {
        session.set("login_failure", &err.to_string()).unwrap();
        HttpResponse::Found().header("location", "/login").finish()
      })
      .map(|user| {
        if user.password != data.password {
          session.set("login_failure", &"Bad password").unwrap();
          return HttpResponse::Found().header("location", "/login").finish();
        }
          id.remember(user.username.to_owned());
          session.set("login_failure", "").unwrap();
          HttpResponse::Found().header("location", "/").finish()
      });

    match res {Ok(res) => res, Err(res) => res,}
}

pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Found().header("location", "/").finish()
}

pub async fn register_form(
  id: Identity,
  tmpl: web::Data<tera::Tera>,
  session: Session,
) -> Result<HttpResponse> {
  if let Some(_id) = id.identity() {return Ok(HttpResponse::Found().header("location", "/").finish());}

  let mut ctx = tera::Context::new();
  ctx.insert("is_logedin", &false);

  if let Some(fail) = session.get::<String>("register_failure")? {
    ctx.insert("failed", &fail);
  } else {
    ctx.insert("failed", "");
  }

  let render = tmpl.render("register.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template error")).expect("Test");

  Ok(HttpResponse::build(StatusCode::OK)
      .content_type("text/html; charset=utf-8")
      .body(render))
}

pub async fn register(
  params: web::Form<RegisterForm>,
  db: web::Data<Pool>,
  session: Session,
) -> HttpResponse {
    let pool = db.clone();
    let data = params.clone();

    let client = Client::default();
    let resp = client.get(format!("https://mailcheck.p.rapidapi.com/?domain={}", data.email))
        .header("x-rapidapi-host", "mailcheck.p.rapidapi.com")
        .header("x-rapidapi-key", "4f21a0e8e2msh739ef3c426b9383p107fb9jsn513436059c77")
        .send()
        .await
        .unwrap()
        .body()
        .await
        .unwrap();

    let json: Result<Value, _> = serde_json::from_slice(&resp);
    if let Bool(block) = json.unwrap().get("block").unwrap() {
        if *block {
          session.set("register_failure", "Invalid email").unwrap();
          return HttpResponse::Found().header("location", "/register").finish();
        }
    }
    if data.password != data.password_confirm {
        session.set("register_failure", "Password do not match").unwrap();
        return HttpResponse::Found().header("location", "/register").finish();
    }

    let _res = web::block(move || {
        let conn = pool.get().unwrap();
        let user_data = SlimUser{
          username: data.username,
          password: data.password,
        };

        register_user(conn, user_data)
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
}
