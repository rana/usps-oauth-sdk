use std::env;
use usps_oauth_sdk::apis::configuration::Configuration;
use usps_oauth_sdk::apis::default_api::post_token;
use usps_oauth_sdk::models::InlineResponse2001::ProviderAccessTokenResponse;

async fn fetch_token() -> Result<(), Box<dyn std::error::Error>> {
    let client_key = &env::var("USPS_CLIENT_KEY")?;
    let client_secret = &env::var("USPS_CLIENT_SECRET")?;

    let cfg = Configuration::new();
    match post_token(
        &cfg,
        Some("client_credentials"),
        None,
        Some(client_key),
        Some(client_secret),
        None,
        None,
        None,
    )
    .await
    {
        Ok(inline_res) => {
            if let ProviderAccessTokenResponse(res) = inline_res {
                eprintln!("res:{res:?}");
            }
            Ok(())
        }
        Err(err) => {
            eprintln!("err:{err:?}");
            Err(Box::new(err))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_token_success() {
        let result = fetch_token().await;
        assert!(result.is_ok(), "fetch_token should succeed");
    }
}
