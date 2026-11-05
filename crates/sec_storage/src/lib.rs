use sec_crypto::SecureVaultCipher;
use sec_domain::TradeOrder;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalletState {
    pub owner: String,
    pub balance_usd: f64, // Inicializado estritamente com $50.00 USD
    pub created_at: i64,
}

pub struct SecureStorageManager {
    password: String,
}

impl SecureStorageManager {
    pub fn new(password: &str) -> Self {
        Self {
            password: password.to_string(),
        }
    }

    /// Guarda o estado da carteira de forma totalmente cifrada (Zero Plaintext)
    pub fn save_wallet(&self, state: &WalletState, path: &str) -> Result<(), &'static str> {
        let plaintext = serde_json::to_vec(state)
            .map_err(|_| "[SEC-STORAGE-ERR] Falha ao serializar estado da carteira.")?;

        let encrypted = SecureVaultCipher::encrypt(&plaintext, &self.password)?;
        
        fs::write(path, encrypted)
            .map_err(|_| "[SEC-STORAGE-ERR] Falha ao escrever cofre cifrado no disco.")?;

        println!("[SEC-STORAGE] Carteira guardada com blindagem AES-256-GCM em: {}", path);
        Ok(())
    }

    /// Carrega e descifra o estado da carteira com autocura de identidade
    pub fn load_wallet(&self, path: &str) -> Result<WalletState, &'static str> {
        if !Path::new(path).exists() {
            // Autocura: Se o cofre não existir, inicializa com o capital base de $50.00 USD
            let initial_state = WalletState {
                owner: "Nolan-Autonomous-Agent".to_string(),
                balance_usd: 50.00,
                created_at: chrono::Utc::now().timestamp(),
            };
            self.save_wallet(&initial_state, path)?;
            return Ok(initial_state);
        }

        let encrypted = fs::read(path)
            .map_err(|_| "[SEC-STORAGE-ERR] Falha ao ler cofre cifrado do disco.")?;

        let plaintext = SecureVaultCipher::decrypt(&encrypted, &self.password)?;

        let state: WalletState = serde_json::from_slice(&plaintext)
            .map_err(|_| "[SEC-STORAGE-ERR] Falha ao desserializar estado da carteira.")?;

        Ok(state)
    }

    /// Grava uma nova ordem no histórico cifrado (sec_orders.enc)
    pub fn append_encrypted_order(&self, order: &TradeOrder, path: &str) -> Result<(), &'static str> {
        let mut orders: Vec<TradeOrder> = if Path::new(path).exists() {
            match fs::read(path) {
                Ok(encrypted) => {
                    match SecureVaultCipher::decrypt(&encrypted, &self.password) {
                        Ok(plaintext) => serde_json::from_slice(&plaintext).unwrap_or_default(),
                        Err(_) => vec![],
                    }
                }
                Err(_) => vec![],
            }
        } else {
            vec![]
        };

        orders.push(order.clone());

        let plaintext = serde_json::to_vec(&orders)
            .map_err(|_| "[SEC-STORAGE-ERR] Falha ao serializar histórico de ordens.")?;

        let encrypted = SecureVaultCipher::encrypt(&plaintext, &self.password)?;

        fs::write(path, encrypted)
            .map_err(|_| "[SEC-STORAGE-ERR] Falha ao escrever histórico cifrado de ordens.")?;

        Ok(())
    }
}
