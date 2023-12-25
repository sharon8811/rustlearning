use std::env;
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use rusttest::configuration::{get_configuration, DatabaseSettings};
use rusttest::telemetry::{get_subscriber, init_subscriber};
use rusttest::startup::{Application, get_connection_pool};
use wiremock::MockServer;


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

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
    pub email_server: MockServer,
}

pub struct ConfirmationLinks {
    pub html: reqwest::Url,
    pub plain_text: reqwest::Url
}

impl TestApp {
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub fn get_confirmation_links(&self, email_request: &wiremock::Request) -> ConfirmationLinks {
        let body: serde_json::Value = serde_json::from_slice(
            &email_request.body
        ).unwrap();

        // Extract the link from one of the request fields.
        let get_link = |s: &str| {
            let links: Vec<_> = linkify::LinkFinder::new()
                .links(s)
                .filter(|l| *l.kind() == linkify::LinkKind::Url)
                .collect();
            assert_eq!(links.len(), 1);
            let raw_link = links[0].as_str().to_owned();
            let mut confirmation_link = reqwest::Url::parse(&raw_link).unwrap();
            // Let's make sure we don't call random APIs on the web
            assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");
            confirmation_link.set_port(Some(self.port)).unwrap();
            confirmation_link
        };

        let html = get_link(&body["HtmlBody"].as_str().unwrap());
        let plain_text = get_link(&body["TextBody"].as_str().unwrap());
        ConfirmationLinks {
            html,
            plain_text
        }
    }

    pub async fn post_newsletters(
        &self,
        body: serde_json::Value
    ) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/newsletters", &self.address))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);
    let email_server = MockServer::start().await;

    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // Port 0 is special-cased at the OS level: trying to bind port 0 will trigger
        // an OS scan for an available port which will then be bound to the application.
        // Use random OS port
        c.application.port = 0;
        c.email_client.base_url = email_server.uri();
        c
    };

    // Create and migrate the database
    configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");

    // Get the port before spawning the application
    let address = format!("http://127.0.0.1:{}", application.port());
    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        port: application_port,
        db_pool: get_connection_pool(&configuration.database),
        email_server,
    }

    // let listener = TcpListener::bind("127.0.0.1:0")
    //     .expect("Failed to bind random port");
    // let port = listener.local_addr().unwrap().port();
    // let address = format!("http://127.0.0.1:{}", port);
    //
    // let mut configuration = get_configuration()
    //     .expect("Failed to read configuration.");
    // configuration.database.database_name = Uuid::new_v4().to_string();
    // let connection_pool = configure_database(&configuration.database).await;
    // let sender_email = configuration.email_client.sender().expect("Invalid sender address");
    // let timeout = configuration.email_client.timeout();
    // let email_client = EmailClient::new(
    //     configuration.email_client.base_url,
    //     sender_email,
    //     configuration.email_client.authorization_token,
    //     timeout
    // );
    //
    // let server =
    //     rusttest::startup::run(listener, connection_pool.clone(), email_client).expect("Failed to bind address");
    // let _ = tokio::spawn(server);
    // TestApp {
    //     address,
    //     db_pool: connection_pool,
    // }
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