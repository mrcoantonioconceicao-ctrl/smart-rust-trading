use std::path::Path;
use tokio::time::{sleep, Duration};
use sec_storage::save_secure_vault;
use sec_binance::fetch_ticker_price;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CascadingWallet {
    pub principal_balance: f64,
    pub reserve_balance: f64,
    pub sub_agents_active: u32,
    pub last_update: String,
}

#[tokio::main]
async fn main() {
    println!("==================================================");
    println!("🛡️ Smart Rust Trading — Agent Subsystem v0.5.2");
    println!("==================================================");

    let vault_path = "sec_wallet.enc";
    let password = b"123456";

    if !Path::new(vault_path).exists() {
        let initial_wallet = CascadingWallet {
            principal_balance: 50.00,
            reserve_balance: 0.00,
            sub_agents_active: 0,
            last_update: "Agent Initialized".into(),
        };
        let _ = save_secure_vault(vault_path, &initial_wallet, password);
    }

    println!("[AGENT] Sub-sistema autónomo iniciado em background.");
    
    loop {
        match fetch_ticker_price("SOLUSDT").await {
            Ok(price) => {
                println!("[AGENT TICK] SOL Spot Price: $ {:.2}", price);
            }
            Err(e) => {
                println!("[AGENT WARNING] Erro ao consultar oráculo: {}", e);
            }
        }
        sleep(Duration::from_secs(10)).await;
    }
}
