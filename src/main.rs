mod server;
mod client;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tokio::join!(
        server::run(("127.0.0.1", 9955)),
        client::run("http://127.0.0.1:9955/", 3)
    );    
}
