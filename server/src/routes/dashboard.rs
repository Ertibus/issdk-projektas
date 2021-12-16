use crate::models::CreateArticleForm;
use crate::models::Article;
use actix_session::Session;
use actix_identity::Identity;
use actix_web::HttpResponse;
use actix_web::http::{StatusCode};
use actix_web::{error, web, Result};
use crate::Pool;
use crate::repo;

pub async fn dashboard(
    id: Identity,
    db: web::Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let pool = db.clone();

    if let Some(id) = id.identity() {
        let res = web::block(move || {
            let conn = pool.get().unwrap();
            repo::check_permissions(conn, id)
        }).await
        .map_err(|err| {
            return Ok(HttpResponse::InternalServerError().body(err.to_string()));
        })
        .map(|is_admin| {
            session.set("is_admin", &is_admin)?;
            return Ok(HttpResponse::Found().header("location", "/dashboard/options").finish());
        });

        session.set("article_focus", &-1)?;
        match res {Ok(res) => res, Err(res) => res,}
    } else {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized access"));
    }
}

pub async fn dashboard_options(
    id: Identity,
    tmpl: web::Data<tera::Tera>,
    db: web::Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let pool = db.clone();

    if let Some(_id) = id.identity() {
        if let Some(is_admin) = session.get::<bool>("is_admin")? {

            let res = web::block(move || {
                let conn = pool.get().unwrap();
                repo::get_users(conn)
            }).await?;


            let mut ctx = tera::Context::new();
            ctx.insert("is_loggedin", &true);
            ctx.insert("is_admin", &is_admin);
            ctx.insert("users", &res);

            let render = tmpl.render("dashboard_options.html", &ctx)
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
            ctx.insert("is_admin", &true);
            ctx.insert("users", &res);

            if let Some(fail) = session.get::<String>("register_failure")? {
                ctx.insert("failed", &fail);
            } else {
                ctx.insert("failed", "");
            }

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

pub async fn dashboard_articles(
    id: Identity,
    tmpl: web::Data<tera::Tera>,
    db: web::Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let pool = db.clone();

    if let Some(id) = id.identity() {
        if let Some(is_admin) = session.get::<bool>("is_admin")? {
            let res = web::block(move || {
                let conn = pool.get().unwrap();
                if is_admin {
                    repo::get_all_articles(conn)
                } else {
                    repo::get_articles(conn, id)
                }
            }).await?;


            let mut ctx = tera::Context::new();
            ctx.insert("is_loggedin", &true);
            ctx.insert("is_admin", &is_admin);
            ctx.insert("articles", &res);

            if let Some(aid) = session.get::<i32>("article_focus")? {
                if aid == -1 {
                    ctx.insert("focus", &Article{
                        id: -1,
                        owner: "noowner".to_string(),
                        title: "".to_string(),
                        description: "".to_string(),
                    });
                } else {
                    let pool = db.clone();
                    let res = web::block(move || {
                        let conn = pool.get().unwrap();
                        repo::get_article(conn, aid)
                    }).await?;

                    ctx.insert("focus", &res);
                }
            } else {
                ctx.insert("focus", &Article{
                    id: -1,
                    owner: "noowner".to_string(),
                    title: "".to_string(),
                    description: "".to_string(),
                });
            }

            if let Some(fail) = session.get::<String>("create_article_failure")? {
                ctx.insert("failed", &fail);
            } else {
                ctx.insert("failed", "");
            }

            let render = tmpl.render("dashboard_articles.html", &ctx)
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

pub async fn dashboard_article_focus(
    id: Identity,
    db: web::Data<Pool>,
    session: Session,
    web::Path((uid,)): web::Path<(i32,)>,
) -> Result<HttpResponse> {
    let pool = db.clone();

    if let Some(_id) = id.identity() {
        if let Some(_is_admin) = session.get::<bool>("is_admin")? {
            if uid != -1 {
                let res = web::block(move || {
                    let conn = pool.get().unwrap();
                    repo::get_article(conn, uid)
                }).await?;
                session.set("article_focus", &res.id)?;
            } else {
                session.set("article_focus", &-1)?;
            }

            return Ok(HttpResponse::Found().header("location", "/dashboard/articles").finish());
        } else {
            return Ok(HttpResponse::Found().header("location", "/dashboard").finish());
        }
    } else {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized access"));
    }
}

pub async fn dashboard_article_post(
    id: Identity,
    params: web::Form<CreateArticleForm>,
    db: web::Data<Pool>,
    session: Session,
    web::Path((uid,)): web::Path<(i32,)>,
) -> HttpResponse {
    let pool = db.clone();
    let data = params.clone();

    if let Some(id) = id.identity() {
        let _res = web::block(move || {
            let conn = pool.get().unwrap();
            let user_data = Article{
                id: uid,
                owner: id,
                title: data.title,
                description: data.description,
            };
            repo::post_article(conn, user_data)
        }).await
        .map_err(|err| {
            session.set("register_failure", &err.to_string()).unwrap();
            return HttpResponse::Found().header("location", "/dashboard/articles").finish();
        })
        .map(|_| {
            session.set("register_failure", "").unwrap();
            return HttpResponse::Found().header("location", "/dashboard/articles").finish();
        });
        return HttpResponse::Found().header("location", "/dashboard/articles").finish();
    } else {
        return HttpResponse::Unauthorized().body("Unauthorized access");
    }
}

pub async fn dashboard_article_del(
    id: Identity,
    db: web::Data<Pool>,
    session: Session,
    web::Path((uid,)): web::Path<(i32,)>,
) -> HttpResponse {
    let pool = db.clone();

    if let Some(_id) = id.identity() {
        let _res = web::block(move || {
            let conn = pool.get().unwrap();
            repo::del_article(conn, uid)
        }).await
        .map_err(|err| {
            session.set("register_failure", &err.to_string()).unwrap();
            return HttpResponse::Found().header("location", "/dashboard/articles").finish();
        })
        .map(|_| {
            session.set("register_failure", "").unwrap();
            return HttpResponse::Found().header("location", "/dashboard/articles").finish();
        });
        return HttpResponse::Found().header("location", "/dashboard/articles").finish();
    } else {
        return HttpResponse::Unauthorized().body("Unauthorized access");
    }
}
