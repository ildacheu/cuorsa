use anyhow::Result;
use axum::async_trait;
use dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::info;

#[derive(Default, Clone)]
pub struct AuthToken {
    token: String,
    expires: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseWithToken {
    token_type: String,
    expires_in: u64,
    ext_expires_in: u64,
    access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipient {
    #[serde(rename = "emailAddress")]
    email_address: Address,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    subject: String,
    body: Body,
    #[serde(rename = "toRecipients")]
    to_recipients: [Recipient; 1],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    #[serde(rename = "contentType")]
    content_type: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmailPayload {
    message: Message,
    #[serde(rename = "saveToSentItems")]
    save_to_sent_items: String,
}

#[async_trait]
pub trait Email {
    async fn get_token(&self) -> Result<ResponseWithToken>;

    async fn renew_token(&mut self) -> Result<()>;

    async fn check_if_token_is_valid(&self) -> bool;

    async fn send_email(&mut self, address: &str, link: &str) -> Result<()>;
}

#[async_trait]
impl Email for AuthToken {
    async fn get_token(&self) -> Result<ResponseWithToken> {
        let client = reqwest::Client::new();
        let params = [
            (
                "client_id",
                dotenv::var("CLIENT_ID").expect("CLIENT_ID is not set in .env file"),
            ),
            ("scope", "https://graph.microsoft.com/.default".to_string()),
            (
                "client_secret",
                dotenv::var("SECRET").expect("SECRET is not set in .env file"),
            ),
            ("grant_type", "client_credentials".to_string()),
        ];
        let url = format!(
            "https://login.microsoftonline.com/{}{}",
            dotenv::var("TENANT_ID").expect("TENANT_ID is not set in .env file"),
            "/oauth2/v2.0/token"
        );
        let res = client
            .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?
            .json::<ResponseWithToken>()
            .await?;
        println!("{:#?}", res);
        Ok(res)
    }

    async fn renew_token(&mut self) -> Result<()> {
        let response = self.get_token().await?;
        self.token = response.access_token;
        self.expires = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::new(0, 0))
            .as_secs()
            + response.expires_in;
        Ok(())
    }

    async fn check_if_token_is_valid(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::new(0, 0))
            .as_secs();
        return now < self.expires;
    }

    async fn send_email(&mut self, address: &str, link: &str) -> Result<()> {
        if !self.check_if_token_is_valid().await {
            self.renew_token().await?;
        }
        let welcome = EmailPayload {
            message: Message {
                subject: "Engraziel per l'annunzia".to_string(),
                body: Body {
                    content_type: "HTML".to_string(),
                    content: format!("Smacca cheu per serrar giu l'annunzia {}", link),
                },
                to_recipients: [Recipient {
                    email_address: Address {
                        address: address.to_string(),
                    },
                }],
            },
            save_to_sent_items: "true".to_string(),
        };
        let client = reqwest::Client::new();
        let url = format!(
            "https://graph.microsoft.com/v1.0/users/{}{}",
            dotenv::var("USER_ID").expect("USER_ID is not set in .env file"),
            "/sendMail"
        );
        let res = client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(&welcome)
            .send()
            .await?;
        info!("{:?}", res);
        Ok(())
    }
}
