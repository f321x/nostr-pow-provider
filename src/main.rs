mod cashu_wallet;
mod hasher;
mod provider;
mod webserver;

use anyhow::{anyhow, Context, Result};
use axum::{http::*, routing::*, Extension, Json, Router};
use cashu_wallet::CashuWallet;
use cdk::{amount::*, nuts::*, wallet::*};
use cdk_sqlite::WalletSqliteDatabase;
use dotenvy::dotenv;
use hasher::Hasher;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use nostr_sdk::prelude::*;
use provider::*;
use rand::Rng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    net::SocketAddr,
    path::Path,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::net::TcpListener;
use webserver::*;

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
        hasher: Hasher::new(),
    });

    api_server(provider).await?;
    Ok(())
}
