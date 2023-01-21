#[macro_use]
extern crate diesel;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod models;
mod schema;
mod err;
mod verifyHandler;
mod emailService;
mod utils;
mod registerHandler;
mod authHandler;



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "simple-auth-server=debug,actix_web=info,actix_server=info",
    );
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age(time::Duration::days(1))
                    .secure(false), // this can only be true if you have https
            ))
            // limit the maximum amount of data that server will accept
            .app_data(web::JsonConfig::default().limit(4096))
            // everything under '/api/' route
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/veri")
                            .route(web::post().to(verifyHandler::post_verification)),
                    )
                    // .service(
                    //     web::resource("/invitation")
                    //     .route(web::post().to(invitation_routes::post_verification)),
                    // )
                    .service(
                        web::resource("/register/{invitation_id}")
                            .route(web::post().to(registerHandler::register_user)),
                    )
                    .service(
                        web::resource("/auth")
                            .route(web::post().to(authHandler::login))
                            .route(web::delete().to(authHandler::logout))
                            .route(web::get().to(authHandler::get_me)),
                    )
                    .service(
                        web::resource("/question/{question_id}")
                            .route(web::post().to(OJHandler::show_question)),
                    ),
    })
    .workers(4)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
