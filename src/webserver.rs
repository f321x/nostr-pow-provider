use super::*;

#[derive(Serialize)]
struct QuoteResponse {
    base_hashprice_sat_pow_20: u64,
    preferred_mint_url: String,
}

// request a new pow task
async fn request_pow(
    Extension(provider): Extension<Arc<Provider>>,
    headers: HeaderMap,
) -> Result<StatusCode, StatusCode> {
    let cashu_token = match headers.get("cashu-token") {
        Some(token) => token.to_str().map_err(|_| StatusCode::BAD_REQUEST)?,
        None => return Err(StatusCode::BAD_REQUEST),
    };
    Ok(StatusCode::OK)
}

// fetch the status of a pow task
async fn fetch_pow() -> Json<QuoteResponse> {
    //
}

// fetch pow quotes (hashprice)
async fn fetch_quote(Extension(provider): Extension<Arc<Provider>>) -> Json<QuoteResponse> {
    let provider = provider.as_ref();
    Json(QuoteResponse {
        base_hashprice_sat_pow_20: provider.base_hashprice,
        preferred_mint_url: provider.mint_url.clone(),
    })
}

pub async fn api_server(provider: Arc<Provider>) -> Result<()> {
    let app = Router::new()
        .route("/pow-quote", get(fetch_quote))
        .route("/request-work", post(request_pow))
        .route("/fetch-work", get(fetch_pow))
        .layer(Extension(provider));

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "9999".to_string())
        .parse()?;
    info!("Coordinator is listening on port {}", port);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let tcp = TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcp, app).await?;

    Ok(())
}
