use rusttest::configuration::get_configuration;
use rusttest::startup::Application;
use rusttest::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber(
        "rusttest".into(), "info".into(), std::io::stdout
    );
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}

// use actix_web::{get, web, App, HttpServer, HttpResponse, Responder, middleware::Logger};
//
// #[get("/hello/{name}")]
// async fn greet(name: web::Path<String>) -> impl Responder {
//     format!("Hello {name}!")
// }
//
// async fn health_check() -> impl Responder {
//     HttpResponse::Ok()
// }
//
// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
//
//     HttpServer::new(|| {
//         App::new()
//             .service(greet)
//             .route("/health_check", web::get().to(health_check))
//             .wrap(Logger::default())
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
//     // println!("Hello, world!");
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[tokio::test]
//     async fn health_check_test() {
//         let response = health_check().await;
//         assert!(response.status().is_success())
//     }
// }
