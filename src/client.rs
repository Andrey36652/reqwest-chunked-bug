use tokio::time::Duration;
use tracing::info;

pub async fn run(request: &str, n_runs: usize) {
    let client = reqwest::Client::builder()
        .connection_verbose(true)
        .timeout(Duration::from_secs(60))
        .pool_idle_timeout(Some(std::time::Duration::from_secs(300)))
        .pool_max_idle_per_host(5)
        // .no_gzip() // uncomment this line to see expected behavior
        .build()
        .expect("reqwest client init error");

    for _ in 0..n_runs {
        info!("---------- start request ----------");
        let response = client
            .get(request)
            .send()
            .await
            .expect("HTTP request error");
        let text = response.text().await.expect("body read error");
        info!("result: {}", text);
        info!("---------- end request ----------");
    }
}
