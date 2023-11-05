use lineup::startup::run;
use std::io::Result;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:80")?;
    run(listener)?.await
}
