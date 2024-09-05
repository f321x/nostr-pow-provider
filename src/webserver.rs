use super::*;

/// testing endpoint
async fn test_api() -> &'static str {
    "Hello, World!"
}

pub async fn api_server() -> Result<()> {
    let app = Router::new().route("/test", get(test_api));
    // .layer(Extension(coordinator));
    // add other routes here

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "9999".to_string())
        .parse()?;
    info!("Coordinator is listening on port {}", port);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let tcp = TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcp, app).await?;

    Ok(())
}
