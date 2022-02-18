#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate serde;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{App, HttpResponse, HttpServer, middleware, web};

use crate::database::DataBase;

pub mod database;
pub mod routes;

async fn error_404() -> HttpResponse {
    HttpResponse::NotFound().body("404 Not Found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = DataBase::load().unwrap_or_else(|e| {
        panic!("Failed to load config: {:?}", e);
    });
    let host_address = db.get_host_address();
    let cookie_key = db.cookie_key.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .wrap(middleware::NormalizePath::default())
            .wrap(middleware::Compress::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new((&cookie_key).as_ref())
                    .name(cookie_key.clone())
                    .secure(false),
            ))
            .service(
                web::resource("/api")
            )
            .service(services![routes::auth::login])
            .default_service(
                web::to(error_404)
            )
    })
        .bind(host_address)?
        .run()
        .await
}

pub type SResult<T> = Result<T, Box<dyn std::error::Error + 'static>>;