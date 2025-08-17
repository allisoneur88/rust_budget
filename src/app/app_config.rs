use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub user_path: PathBuf,
    pub budget_path: PathBuf,
    pub super_transaction_path: PathBuf,
    pub account_path: PathBuf,
    pub super_category_path: PathBuf,
    pub category_path: PathBuf,
    pub transaction_path: PathBuf,
    pub payee_path: PathBuf,
    pub assignment_path: PathBuf,
    pub currency_path: PathBuf,
}

impl AppConfig {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&contents)?;
        Ok(config)
    }
}
