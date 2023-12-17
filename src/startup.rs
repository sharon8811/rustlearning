use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
//use actix_web::middleware::Logger;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use crate::routes::*;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
        })
        .listen(listener)?
        .run();
    Ok(server)
    // println!("Hello, world!");
}