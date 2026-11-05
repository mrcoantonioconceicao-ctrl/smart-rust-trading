use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use argon2::{
    Algorithm, Argon2, Params, Version,
};
use rand::{rngs::OsRng, RngCore};
use zeroize::Zeroize;

/// Parâmetros recomendados para o Argon2id ajustados para ambiente Mobile (Termux/Android)
/// - memory_cost (m): 19456 (19 MB) -> Seguro contra força bruta, mas leve para o telemóvel.
/// - time_cost (t): 2 -> Duas iterações para garantir atraso ideal (~0.5s a 1s).
/// - parallelism (p): 1 -> Otimizado para núcleos móveis.
pub fn default_mobile_params() -> Params {
    Params::new(
        19456, // 19 MB de custo de memória
        2,     // Iterações
        1,     // Grau de paralelismo
        Some(32), // Tamanho da chave gerada (32 bytes para AES-256)
    ).expect("Falha ao criar parâmetros Argon2")
}

/// Deriva uma chave criptográfica segura a partir da pass do utilizador e um Salt,
/// utilizando Argon2id (Memory-Hard).
pub fn derive_key(password: &[u8], salt: &[u8], params: Params) -> Result<[u8; 32], String> {
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    
    let mut derived_key = [0u8; 32];
    argon2
        .hash_password_into(password, salt, &mut derived_key)
        .map_err(|e| format!("Erro na derivação Argon2id: {}", e))?;

    Ok(derived_key)
}

/// Cifra os dados usando AES-256-GCM com chave derivada via Argon2id.
pub fn encrypt_vault(data: &[u8], password: &[u8]) -> Result<Vec<u8>, String> {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let params = default_mobile_params();
    let mut key_bytes = derive_key(password, &salt, params)?;
    
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes); // Corrigido para &mut nonce_bytes
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|e| format!("Erro ao cifrar cofre: {}", e))?;

    key_bytes.zeroize();

    let mut result = Vec::with_capacity(16 + 12 + ciphertext.len());
    result.extend_from_slice(&salt);
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// Decifra o cofre validando a integridade autenticada.
pub fn decrypt_vault(encrypted_data: &[u8], password: &[u8]) -> Result<Vec<u8>, String> {
    if encrypted_data.len() < 28 {
        return Err("Dados cifrados inválidos ou corrompidos".into());
    }

    let salt = &encrypted_data[0..16];
    let nonce_bytes = &encrypted_data[16..28];
    let ciphertext = &encrypted_data[28..];

    let params = default_mobile_params();
    let mut key_bytes = derive_key(password, salt, params)?;

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "Falha ao decifrar: Senha incorreta ou cofre corrompido".to_string())?;

    key_bytes.zeroize();

    Ok(plaintext)
}
