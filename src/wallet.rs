use crate::ZebedeeClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletData {
    pub unit: String,
    pub balance: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletRes {
    pub message: String,
    pub data: WalletData,
}

pub async fn get_wallet_details(client: ZebedeeClient) -> Result<WalletRes, anyhow::Error> {
    let url = String::from("https://api.zebedee.io/v0/wallet");
    let resp = client
        .reqw_cli
        .get(&url)
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .send()
        .await?;

    let status_code = resp.status();
    let status_success = resp.status().is_success();
    let resp_text = resp.text().await?;

    if !status_success {
        return Err(anyhow::anyhow!(
            "Error: status {}, message: {}, url: {}",
            status_code,
            resp_text.clone(),
            &url,
        ));
    }

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2: WalletRes = match resp_serialized {
        Ok(c) => c,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\nstatus code: {}\n url: {}",
                e,
                resp_text.clone(),
                status_code,
                &url,
            ))
        }
    };

    Ok(resp_seralized_2)
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[tokio::test]
    async fn test_wallet_details() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);
        let any_balance = 0..; //u64::MAX;
        let r = get_wallet_details(zebedee_client)
            .await
            .unwrap()
            .data
            .balance;
        let r2: u64 = r.parse().unwrap();
        assert!(any_balance.contains(&r2));
    }
}
