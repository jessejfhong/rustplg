use lineup::startup::run;
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
}

impl TestApp {
    pub async fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to create a listener");
        let addr = listener.local_addr().unwrap();
        let port = addr.port();

        std::mem::drop(tokio::spawn(async { run(listener).unwrap().await }));

        Self {
            address: format!("http://localhost:{}", port),
        }
    }
}
