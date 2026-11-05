use sec_crypto::{encrypt_vault, decrypt_vault};
use serde::{de::DeserializeOwned, Serialize};

/// Salva e cifrar dados estruturados no cofre local usando o novo motor Argon2id + AES-256-GCM
pub fn save_secure_vault<T: Serialize>(path: &str, data: &T, password: &[u8]) -> Result<(), String> {
    let json_bytes = serde_json::to_vec(data)
        .map_err(|e| format!("Erro ao serializar dados para JSON: {}", e))?;

    let encrypted_data = encrypt_vault(&json_bytes, password)?;

    std::fs::write(path, encrypted_data)
        .map_err(|e| format!("Erro ao gravar cofre no disco ({}): {}", path, e))?;

    Ok(())
}

/// Carrega e decifra dados estruturados do cofre local
pub fn load_secure_vault<T: DeserializeOwned>(path: &str, password: &[u8]) -> Result<T, String> {
    if !std::path::Path::new(path).exists() {
        return Err(format!("Ficheiro de cofre não encontrado: {}", path));
    }

    let encrypted_data = std::fs::read(path)
        .map_err(|e| format!("Erro ao ler cofre do disco ({}): {}", path, e))?;

    let plaintext = decrypt_vault(&encrypted_data, password)?;

    let data: T = serde_json::from_slice(&plaintext)
        .map_err(|e| format!("Erro ao desserializar JSON do cofre: {}", e))?;

    Ok(data)
}
