use super::*;

pub struct Provider {
    pub base_hashprice: u64,
    pub mint_url: String,
    pub wallet: Arc<CashuWallet>,
    pub hasher: Arc<Mutex<Hasher>>,
}

impl Provider {
    pub async fn handle_request(
        &self,
        pow_request: PoWRequest,
    ) -> Result<StatusCode, (StatusCode, String)> {
        match self.wallet.receive(&pow_request.ecash).await {
            Ok(amount) => {
                let required_price =
                    self.base_hashprice * (2u64.pow(pow_request.leading_zeros as u32 - 20));
                if amount < (required_price as f64 * 0.98) as u64 {
                    // some tolerance for shitty clients
                    return Err((
                        StatusCode::PAYMENT_REQUIRED,
                        format!(
                            "Insufficient funds: {} < {}. Your money is gone, try again.",
                            amount, required_price
                        ),
                    ));
                }
            }
            Err(e) => {
                warn!("Failed to receive token: {}", e);
                return Err((StatusCode::PAYMENT_REQUIRED, e));
            }
        }
        let mut hasher = self.hasher.clone();
        hasher.add_task(pow_request.event, pow_request.leading_zeros);

        Ok(StatusCode::PROCESSING)
    }
}
