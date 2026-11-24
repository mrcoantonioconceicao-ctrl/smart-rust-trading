use std::env;
use std::path::Path;
use std::process::exit;
use tokio::time::{sleep, Duration};
use sec_storage::save_secure_vault;
use sec_binance::fetch_ticker_price;
use serde::{Serialize, Deserialize};

const DEFAULT_VAULT_PATH: &str = "sec_wallet.enc";
const MIN_PASSWORD_LEN: usize = 8;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CascadingWallet {
    pub principal_balance: f64,
    pub reserve_balance: f64,
    pub sub_agents_active: u32,
    pub last_update: String,
}

/// Lê a senha mestre da variável de ambiente SEC_WALLET_PASSWORD.
/// Nunca armazene senhas no código-fonte.
fn load_master_password() -> Vec<u8> {
    match env::var("SEC_WALLET_PASSWORD") {
        Ok(pw) if !pw.is_empty() => {
            if pw.len() < MIN_PASSWORD_LEN {
                eprintln!("[SEC-ERROR] SEC_WALLET_PASSWORD demasiado curta (mínimo {} carateres).", MIN_PASSWORD_LEN);
                exit(1);
            }
            pw.into_bytes()
        }
        _ => {
            eprintln!("[SEC-ERROR] Variável de ambiente SEC_WALLET_PASSWORD não definida.");
            eprintln!("[HINT] Defina a variável SEC_WALLET_PASSWORD com uma senha forte (mín. 8 carateres).");
            exit(1);
        }
    }
}


#[tokio::main]
async fn main() {
    println!("==================================================");
    println!("🛡️ Smart Rust Trading — Agent Subsystem v0.5.2");
    println!("==================================================");

    let vault_path = env::var("SEC_VAULT_PATH").unwrap_or_else(|_| DEFAULT_VAULT_PATH.to_string());
    let password = load_master_password();

    if !Path::new(&vault_path).exists() {
        let initial_wallet = CascadingWallet {
            principal_balance: 50.00,
            reserve_balance: 0.00,
            sub_agents_active: 0,
            last_update: "Agent Initialized".into(),
        };
        if let Err(e) = save_secure_vault(&vault_path, &initial_wallet, &password) {
            eprintln!("[SEC-ERROR] Falha ao criar cofre cifrado: {}", e);
            exit(1);
        }
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
