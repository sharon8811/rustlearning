use once_cell::sync::Lazy;
use rusttest::configuration::{get_configuration, DatabaseSettings};
use rusttest::telemetry::{get_subscriber, init_subscriber};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::env;
use std::net::TcpListener;
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

#[derive(Clone)]
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect ro Postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    // Port 0 is special-cased at the OS level: trying to bind port 0 will trigger
    // an OS scan for an available port which will then be bound to the application.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server =
        rusttest::startup::run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

#[tokio::test]
async fn health_check_works() {
    // env::set_var("RUST_LOG", "error");
    let app = spawn_app().await;
    let address = app.address;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect(
            "Fai\
        led to execute request.",
        );

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    let mut handles: Vec<tokio::task::JoinHandle<Result<(), std::io::Error>>> = vec![];
    for _ in 0..100 {
        let client = client.clone();
        let address = address.clone();
        handles.push(tokio::spawn(async move {
            let _ = client
                .get(format!("{}/health_check", &address))
                .send()
                .await;
            Ok(())
        }));
    }

    futures::future::join_all(handles).await;
    // env::remove_var("RUST_LOG");
}

#[tokio::test]
async fn subscribe_returns_a_200_for_a_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=testttttt%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to make a request");

    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "testttttt@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            description
        )
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let app_address = app.address;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=testttt%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to make a request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The api did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
