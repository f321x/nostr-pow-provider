mod cashu_wallet;
mod webserver;

use anyhow::{anyhow, Context, Result};
use axum::{routing::get, Router};
use cdk::{nuts::*, wallet::*};
use dotenvy::dotenv;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use rand::Rng;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use webserver::api_server;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    dotenv().ok();

    api_server().await?;
    Ok(())
}
