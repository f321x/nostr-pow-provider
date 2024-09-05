use super::*;

pub struct CashuWallet {
    pub cdk_wallet: Wallet,
}

impl CashuWallet {
    pub fn new_from_env() -> Result<Self> {
        let secret: [u8; 32] = match env::var("CASHU_SECRET")
            .context("CASHU_SECRET not found")?
            .as_str()
        {
            "" => {
                warn!("CASHU_SECRET is empty, generating a new one");
                let seed = rand::thread_rng().gen::<[u8; 32]>();
                info!(
                    "Generated seed: {}. Back it up and set it in the .env",
                    hex::encode(&seed)
                );
                seed
            }
            secret_str => {
                let seed = hex::decode(secret_str).context("CASHU_SECRET is not a valid hex")?;
                if seed.len() != 32 {
                    return Err(anyhow!("CASHU_SECRET is not 32 bytes long"));
                };
                seed.try_into()
                    .expect("Seed vec couldn't be converted to [u8; 32]")
            }
        };
        let mint_url = env::var("MINT_URL").context("MINT_URL not set")?;
        let unit = CurrencyUnit::Sat;

        let localstore = WalletMemoryDatabase::default();
        let wallet = Wallet::new(mint_url, unit, Arc::new(localstore), &seed, None);
        Self {}
    }
}
