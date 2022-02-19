#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate serde;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use actix_web::cookie::SameSite;
use actix_web::cookie::time::Duration;

use crate::database::Config;

pub mod database;
pub mod routes;
pub mod user;

async fn error_404() -> HttpResponse {
    HttpResponse::NotFound().body("404 Not Found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = Config::load().unwrap_or_else(|e| {
        panic!("Failed to load config: {:?}", e);
    });
    let host_address = cfg.get_host_address();
    let cookie_key = cfg.cookie_key.clone();
    let db = web::Data::new(cfg.create_db());
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .app_data(web::JsonConfig::default().limit(1024 * 8))
            .wrap(middleware::NormalizePath::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new((&cookie_key).as_ref())
                    .name("safe-storage")
                    .secure(false)
                    .max_age(Duration::days(1))
                    .same_site(SameSite::Lax),
            ))
            .service(services![
                routes::auth::login,
                routes::user_info::user_info
            ])
            .default_service(
                web::to(error_404)
            )
    })
        .bind(host_address)?
        .run()
        .await
}

pub type SResult<T> = Result<T, Box<dyn std::error::Error + 'static>>;