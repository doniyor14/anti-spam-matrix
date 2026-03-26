use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub username: String,
    pub auth: Auth,

    pub spam_limit: u32,
    pub spam_regex_exprs: Vec<String>,

    pub proxy: Option<String>,
}

#[derive(Deserialize, Default, Serialize)]
#[serde(tag = "type")]
pub enum Auth {
    #[serde(rename = "password")]
    Password { password: String },
    #[serde(rename = "sso_login")]
    #[default]
    SSO,
}

impl Config {
    /// Validate configuration values and return an error for any invalid state.
    pub fn validate(&self) -> Result<()> {
        if self.spam_limit == 0 {
            bail!("`spam_limit` must be greater than 0");
        }
        if self.spam_regex_exprs.is_empty() {
            tracing::warn!("`spam_regex_exprs` is empty – the bot will never match spam");
        }
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: "@alice:example.org".into(),
            auth: Auth::Password {
                password: "hardpassword".into(),
            },
            spam_limit: 3,
            spam_regex_exprs: vec![],
            proxy: None,
        }
    }
}
