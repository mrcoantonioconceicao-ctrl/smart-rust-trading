use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, Params,
};
use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes256Gcm, Key, Nonce,
};
use rand::RngCore;
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;
use zeroize::Zeroize;

pub const CURRENT_VERSION: u8 = 0x01;
pub const VERSION_LEN: usize = 1;
pub const SALT_LEN: usize = 16;
pub const NONCE_LEN: usize = 12;
pub const MIN_ENCRYPTED_LEN: usize = VERSION_LEN + SALT_LEN + NONCE_LEN + 16;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Erro na derivação de chave Argon2id")]
    Argon2Error,
    #[error("Erro de cifragem/decifragem AES-GCM")]
    AesError,
    #[error("Dados cifrados corrompidos, truncados ou versão incompatível")]
    InvalidFormat,
    #[error("Falha ao serializar/deserializar dados JSON")]
    SerializationError(#[from] serde_json::Error),
    #[error("Parâmetros Argon2 inválidos")]
    InvalidParams,
}

pub fn secure_mobile_params() -> Result<Params, CryptoError> {
    Params::new(19456, 2, 1, Some(32)).map_err(|_| CryptoError::InvalidParams)
}

pub fn derive_key(password: &[u8], salt: &[u8]) -> Result<[u8; 32], CryptoError> {
    let params = secure_mobile_params()?;
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    
    let salt_str = SaltString::encode_b64(salt)
        .map_err(|_| CryptoError::Argon2Error)?;
    
    let password_hash = argon2.hash_password(password, &salt_str)
        .map_err(|_| CryptoError::Argon2Error)?;

    let hash_bytes = password_hash.hash.ok_or(CryptoError::Argon2Error)?;
    let bytes = hash_bytes.as_bytes();
    
    if bytes.len() < 32 {
        return Err(CryptoError::Argon2Error);
    }

    let mut key = [0u8; 32];
    key.copy_from_slice(&bytes[..32]);
    Ok(key)
}

pub fn encrypt_payload<T: Serialize>(data: &T, password: &[u8], aad: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let serialized = serde_json::to_vec(data)?;

    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);

    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);

    let mut key_bytes = derive_key(password, &salt)?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let payload = Payload {
        msg: &serialized,
        aad,
    };

    let ciphertext = cipher.encrypt(nonce, payload)
        .map_err(|_| CryptoError::AesError)?;

    key_bytes.zeroize();

    let mut output = Vec::with_capacity(VERSION_LEN + SALT_LEN + NONCE_LEN + ciphertext.len());
    output.push(CURRENT_VERSION);
    output.extend_from_slice(&salt);
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);

    Ok(output)
}

pub fn decrypt_payload<T: DeserializeOwned>(encrypted_data: &[u8], password: &[u8], aad: &[u8]) -> Result<T, CryptoError> {
    if encrypted_data.len() < MIN_ENCRYPTED_LEN {
        return Err(CryptoError::InvalidFormat);
    }

    let version = encrypted_data[0];
    if version != CURRENT_VERSION {
        return Err(CryptoError::InvalidFormat);
    }

    let salt = &encrypted_data[VERSION_LEN..VERSION_LEN + SALT_LEN];
    let nonce_bytes = &encrypted_data[VERSION_LEN + SALT_LEN..VERSION_LEN + SALT_LEN + NONCE_LEN];
    let ciphertext = &encrypted_data[VERSION_LEN + SALT_LEN + NONCE_LEN..];

    let mut key_bytes = derive_key(password, salt)?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    let payload = Payload {
        msg: ciphertext,
        aad,
    };

    let decrypted = cipher.decrypt(nonce, payload)
        .map_err(|_| CryptoError::AesError)?;

    key_bytes.zeroize();

    let obj: T = serde_json::from_slice(&decrypted)?;
    Ok(obj)
}
