use sha3::Digest;
use once_cell::sync::Lazy;
use sqlx::Connection;
use sqlx::PgConnection;
use sqlx::{migrate::MigrateDatabase, PgPool, Postgres};
use uuid::Uuid;
use wiremock::MockServer;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_log_level = "info".into();
    let subscriber_name = "test".into();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_log_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_log_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
    pub email_server: MockServer,
    pub postgres_connection_str: String,
    pub test_user: TestUser,
}

pub struct ConfirmationLinks {
    pub html: reqwest::Url,
    pub text: reqwest::Url,
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    if !Postgres::database_exists(&config.connection_str_with_db())
        .await
        .unwrap_or(false)
    {
        Postgres::create_database(&config.connection_str_with_db())
            .await
            .expect("Failed to create Postgres database");
    }

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    // need to close connection in order for migration changes to be applied
    // closing an in memory connection will drop the database as well, so cannot
    // use in memory db for testing
    connection_pool
}

pub async fn cleanup(app: &TestApp) {
    app.db_pool.close().await;

    // the web server may be still using the db at this point
    // need to shutdown it first before droping the database
    //Postgres::drop_database(&app.postgres_connection_str).await.expect("Failed to drop database");
}

pub struct TestUser {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            username: Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string()
        }
    }

    async fn store(&self, pool: &PgPool) {
        let password_hash = sha3::Sha3_256::digest(
            self.password.as_bytes()
        );

        let password_hash = format!("{:x}", password_hash);

        sqlx::query!(
            r#"insert into users (user_id, username, password_hash)
               values ($1, $2, $3)
            "#,
            self.user_id,
            self.username,
            password_hash,
        )
        .execute(pool)
        .await
        .expect("Failed to store test user");
    }
}

impl TestApp {
    pub async fn new() -> Self {
        // The frist time `initialize` is invoked the code in `TRACING` is executed.
        // All other invocation will instead skip execution.
        Lazy::force(&TRACING);

        let email_server = MockServer::start().await;

        // Setup database connection pool
        let configuration = {
            let mut c = get_configuration().expect("Failed to read configuration");
            c.database.database_name = format!("data/{}", Uuid::new_v4());
            c.application.port = 0;
            c.email_client.endpoint_url = email_server.uri();
            c
        };

        let db_pool = configure_database(&configuration.database).await;

        // Spin up the server
        let app = Application::build(&configuration)
            .await
            .expect("Failed to build application");
        let address = format!("http://127.0.0.1:{}", app.port());
        let port = app.port();
        std::mem::drop(tokio::spawn(app.run_until_stopped()));

        let test_app = TestApp {
            port,
            address,
            db_pool,
            email_server,
            postgres_connection_str: configuration.database.connection_str_with_db(),
            test_user: TestUser::generate(),
        };

        test_app.test_user.store(&test_app.db_pool).await;

        test_app
    }

    pub async fn post_subscriptions(&self, body: &str) -> reqwest::Response {
        let url = format!("{}/subscriptions", self.address);
        reqwest::Client::new()
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body.to_string())
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_newsletters(&self, body: serde_json::Value) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/newsletters", &self.address))
            .basic_auth(&self.test_user.username, Some(&self.test_user.password))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub fn get_confirmation_links(&self, email_request: &wiremock::Request) -> ConfirmationLinks {
        let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

        let get_link = |s: &str| {
            let links: Vec<_> = linkify::LinkFinder::new()
                .links(s)
                .filter(|l| *l.kind() == linkify::LinkKind::Url)
                .collect();
            assert_eq!(links.len(), 1);
            let raw_link = links[0].as_str().to_owned();
            let mut confirmation_link = reqwest::Url::parse(&raw_link).unwrap();
            assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");
            confirmation_link.set_port(Some(self.port)).unwrap();
            confirmation_link
        };

        let html = get_link(
            body.pointer("/Content/Simple/Body/Html/Data")
                .unwrap()
                .as_str()
                .unwrap(),
        );
        let text = get_link(
            body.pointer("/Content/Simple/Body/Text/Data")
                .unwrap()
                .as_str()
                .unwrap(),
        );

        ConfirmationLinks { html, text }
    }
}
