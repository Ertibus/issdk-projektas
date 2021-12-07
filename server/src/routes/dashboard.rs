use actix_session::Session;
use actix_identity::Identity;
use actix_web::HttpResponse;
use actix_web::http::{StatusCode};
use actix_web::{error, web, get, Result};
use crate::Pool;
use crate::repo;

pub async fn dashboard(
    id: Identity,
    db: web::Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let pool = db.clone();

    if let Some(id) = id.identity() {
        if let Some(_) = session.get::<bool>("is_admin")? {
            return Ok(HttpResponse::Found().header("location", "/dashboard/users").finish());
        } else {
            let res = web::block(move || {
                let conn = pool.get().unwrap();
                repo::check_permissions(conn, id)
            }).await
            .map_err(|err| {
                return Ok(HttpResponse::InternalServerError().body(err.to_string()));
            })
            .map(|is_admin| {
                session.set("is_admin", &is_admin)?;
                return Ok(HttpResponse::Found().header("location", "/dashboard/users").finish());
            });

            match res {Ok(res) => res, Err(res) => res,}
        }

    } else {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized access"));
    }
}

pub async fn dashboard_users(
    id: Identity,
    tmpl: web::Data<tera::Tera>,
    db: web::Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let pool = db.clone();

    if let Some(_id) = id.identity() {
        if let Some(is_admin) = session.get::<bool>("is_admin")? {
            if !is_admin {return Ok(HttpResponse::Unauthorized().body("Unauthorized access"))};

            let res = web::block(move || {
                let conn = pool.get().unwrap();
                repo::get_users(conn)
            }).await?;


            let mut ctx = tera::Context::new();
            ctx.insert("is_loggedin", &true);
            ctx.insert("is_admin", &is_admin);
            ctx.insert("users", &res);

            let render = tmpl.render("dashboard_users.html", &ctx)
            .map_err(|err| error::ErrorInternalServerError(format!("Template error: {:?}", err.to_string()))).expect("Test");

            return Ok(HttpResponse::build(StatusCode::OK)
                .content_type("text/html; charset=utf-8")
                .body(render));
        } else {
            return Ok(HttpResponse::Found().header("location", "/dashboard").finish());
        }
    } else {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized access"));
    }
}


pub async fn dashboard_user_del(
    id: Identity,
    db: web::Data<Pool>,
    session: Session,
    web::Path((uid,)): web::Path<(i32,)>,
) -> Result<HttpResponse> {
    let pool = db.clone();

    if let Some(_id) = id.identity() {
        if let Some(is_admin) = session.get::<bool>("is_admin")? {
            if !is_admin {return Ok(HttpResponse::Unauthorized().body("Unauthorized access"))};

            web::block(move || {
                let conn = pool.get().unwrap();
                repo::del_user(conn, uid)
            }).await?;

            return Ok(HttpResponse::Found().header("location", "/dashboard/users").finish());
        } else {
            return Ok(HttpResponse::Found().header("location", "/dashboard").finish());
        }
    } else {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized access"));
    }
}


pub async fn dashboard_user_promote(
    id: Identity,
    db: web::Data<Pool>,
    session: Session,
    web::Path((uid,)): web::Path<(i32,)>,
) -> Result<HttpResponse> {
    let pool = db.clone();

    if let Some(_id) = id.identity() {
        if let Some(is_admin) = session.get::<bool>("is_admin")? {
            if !is_admin {return Ok(HttpResponse::Unauthorized().body("Unauthorized access"))};

            web::block(move || {
                let conn = pool.get().unwrap();
                repo::promote_user(conn, uid)
            }).await?;

            return Ok(HttpResponse::Found().header("location", "/dashboard/users").finish());
        } else {
            return Ok(HttpResponse::Found().header("location", "/dashboard").finish());
        }
    } else {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized access"));
    }
}


pub async fn dashboard_user_demote(
    id: Identity,
    db: web::Data<Pool>,
    session: Session,
    web::Path((uid,)): web::Path<(i32,)>,
) -> Result<HttpResponse> {
    let pool = db.clone();

    if let Some(_id) = id.identity() {
        if let Some(is_admin) = session.get::<bool>("is_admin")? {
            if !is_admin {return Ok(HttpResponse::Unauthorized().body("Unauthorized access"))};

            web::block(move || {
                let conn = pool.get().unwrap();
                repo::demote_user(conn, uid)
            }).await?;

            return Ok(HttpResponse::Found().header("location", "/dashboard/users").finish());
        } else {
            return Ok(HttpResponse::Found().header("location", "/dashboard").finish());
        }
    } else {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized access"));
    }
}
