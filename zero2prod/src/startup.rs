use crate::configuration::{DatabaseSettings, Settings};
use crate::email_client::EmailClient;
use crate::routes::{confirm, health_check, home, publish_newsletter, subscribe};
use actix_web::dev::Server;
use actix_web::web::{get, post, Data};
use actix_web::{web, App, HttpServer};
use aws_config::timeout::TimeoutConfig;
use aws_types::region::Region;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub struct Application {
    pub port: u16,
    pub server: Server,
}

pub struct ApplicationBaseUrl(pub String);

impl Application {
    pub async fn build(configuration: &Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        // setup tcp listener.
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)
            .unwrap_or_else(|_| panic!("Failed to bind port: {}", configuration.application.port));

        // setup email client.
        let timeout = configuration.email_client.timeout();
        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address");

        let aws_conf = aws_config::from_env()
            .region(Region::new(configuration.email_client.region.to_owned()))
            .timeout_config(
                TimeoutConfig::builder()
                    .operation_timeout(configuration.email_client.timeout())
                    .build(),
            )
            .endpoint_url(configuration.email_client.endpoint_url.to_owned())
            .load()
            .await;

        let email_client = EmailClient::new(sender_email, timeout, aws_conf);

        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            email_client,
            &configuration.application.base_url,
        )?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn run(
    listener: TcpListener,
    connection_pool: PgPool,
    email_client: EmailClient,
    base_url: &String,
) -> Result<Server, std::io::Error> {
    let connection_pool = Data::new(connection_pool);
    let email_client = Data::new(email_client);
    let base_url = Data::new(ApplicationBaseUrl(base_url.to_owned()));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", get().to(home))
            .route("/health_check", get().to(health_check))
            .route("/subscriptions", post().to(subscribe))
            .route("/subscriptions/confirm", get().to(confirm))
            .route("/newsletters", post().to(publish_newsletter))
            .app_data(connection_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
