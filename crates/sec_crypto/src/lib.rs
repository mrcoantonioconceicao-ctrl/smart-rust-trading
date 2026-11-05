use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};
use aes_gcm::aead::rand_core::RngCore;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

pub struct SecureVaultCipher;

impl SecureVaultCipher {
    pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(
            password.as_bytes(),
            salt,
            100_000,
            &mut key,
        );
        key
    }

    pub fn encrypt(plaintext: &[u8], password: &str) -> Result<Vec<u8>, &'static str> {
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);

        let key_bytes = Self::derive_key(password, &salt);
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher.encrypt(nonce, plaintext)
            .map_err(|_| "[SEC-CRYPTO-ERR] Falha crítica na encriptação AES-GCM.")?;

        let mut encrypted_payload = Vec::with_capacity(16 + 12 + ciphertext.len());
        encrypted_payload.extend_from_slice(&salt);
        encrypted_payload.extend_from_slice(&nonce_bytes);
        encrypted_payload.extend_from_slice(&ciphertext);

        Ok(encrypted_payload)
    }

    pub fn decrypt(encrypted_payload: &[u8], password: &str) -> Result<Vec<u8>, &'static str> {
        if encrypted_payload.len() < 28 {
            return Err("[SEC-CRYPTO-ERR] Payload cifrado corrompido ou inválido.");
        }

        let salt = &encrypted_payload[0..16];
        let nonce_bytes = &encrypted_payload[16..28];
        let ciphertext = &encrypted_payload[28..];

        let key_bytes = Self::derive_key(password, salt);
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|_| "[SEC-CRYPTO-ERR] Falha na desencriptação. Palavra-passe incorreta ou dados adulterados.")?;

        Ok(plaintext)
    }
}
