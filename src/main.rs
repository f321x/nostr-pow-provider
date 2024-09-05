mod cashu_wallet;
mod webserver;

use anyhow::{anyhow, Context, Result};
use axum::{http::*, routing::*, Extension, Json, Router};
use cashu_wallet::CashuWallet;
use cdk::{nuts::*, wallet::*};
use cdk_sqlite::WalletSqliteDatabase;
use dotenvy::dotenv;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use rand::Rng;
use serde::Serialize;
use std::{env, net::SocketAddr, path::Path, sync::Arc};
use tokio::net::TcpListener;
use webserver::api_server;

pub struct Provider {
    pub base_hashprice: u64,
    pub mint_url: String,
    pub wallet: Arc<CashuWallet>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    dotenv().ok();

    let wallet = Arc::new(CashuWallet::new_from_env().await?);
    let provider = Arc::new(Provider {
        base_hashprice: env::var("BASE_QUOTE_SAT_POW_20")?.parse()?,
        mint_url: env::var("CASHU_MINT_URL")?,
        wallet,
    });

    api_server(provider).await?;
    Ok(())
}
