mod routes;
mod repo;
mod models;

use actix_session::CookieSession;
use tera::Tera;
use rand::Rng;
use r2d2_sqlite::SqliteConnectionManager;
use crate::repo::Pool;
use actix_files::Files;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    // Initiates error logger
    env_logger::init();

    // Databas
    let manager = SqliteConnectionManager::file(concat!(env!("CARGO_MANIFEST_DIR"), "/data.sqlite"));
    let pool = Pool::new(manager).unwrap();

    // Authorisation
    let cookie_secret_key = rand::thread_rng().gen::<[u8; 32]>();

    println!("Listening on: 127.0.0.1:8080");
    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .data(pool.clone())
            .data(tera)
            // Authorisation
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&cookie_secret_key)
                    .name("auth")
                    .path("/")
                    .domain("127.0.0.1")
                    .max_age(86400)
                    .secure(false),
            ))
            .wrap(
                CookieSession::signed(&[0; 32])
                    .secure(false)
            )
            // Error logging
            .wrap(Logger::default())
            // Services
            .service(routes::index)
            .service(
                web::scope("/login")
                    .service(web::resource("")
                        .route(web::get().to(routes::auth::login_form))
                        .route(web::post().to(routes::auth::login))
            ))
            .service(
                web::scope("/register")
                    .service(web::resource("")
                        .route(web::get().to(routes::auth::register_form))
                        .route(web::post().to(routes::auth::register))
            ))
            .service(
                web::resource("/logout")
                    .route(web::get().to(routes::auth::logout)
            ))
            .service(
                web::scope("/dashboard")
                    .service(web::resource("")
                        .route(web::get().to(routes::dashboard::dashboard)))
                    .service(web::scope("/users")
                        .service(web::resource("")
                            .route(web::get().to(routes::dashboard::dashboard_users)))
                        .service(web::resource("/delete/{uid}")
                            .route(web::get().to(routes::dashboard::dashboard_user_del)))
                        .service(web::resource("/promote/{uid}")
                            .route(web::get().to(routes::dashboard::dashboard_user_promote)))
                        .service(web::resource("/demote/{uid}")
                            .route(web::get().to(routes::dashboard::dashboard_user_demote)))
            ))
            .service(Files::new("/", "./server/static/"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

