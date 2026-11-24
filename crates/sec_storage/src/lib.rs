use std::fs;
use std::path::{Component, Path, PathBuf};
use std::io::Write;
use serde::{Serialize, de::DeserializeOwned};
use sec_crypto::{encrypt_payload, decrypt_payload, CryptoError};
use thiserror::Error;
use tempfile::NamedTempFile;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

const MAX_VAULT_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10 MB

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Erro criptográfico: {0}")]
    Crypto(#[from] CryptoError),
    #[error("Erro de E/S em disco: {0}")]
    Io(#[from] std::io::Error),
    #[error("Cofre não encontrado no caminho especificado")]
    NotFound,
    #[error("Arquivo corrompido, vazio ou excede o limite de tamanho permitido")]
    InvalidFileSize,
    #[error("Caminho do ficheiro inválido ou inseguro: {0}")]
    InvalidPath(String),
}

/// Valida o caminho do ficheiro de cofre para impedir path traversal,
/// caminhos absolutos e componentes perigosos.
fn validate_vault_path(path: &str) -> Result<PathBuf, StorageError> {
    if path.is_empty() {
        return Err(StorageError::InvalidPath("caminho vazio".to_string()));
    }

    let path_obj = Path::new(path);

    // Rejeita caminhos absolutos
    if path_obj.is_absolute() {
        return Err(StorageError::InvalidPath(format!("caminho absoluto não permitido: {}", path)));
    }

    // Rejeita componentes perigosos ("..", raiz, prefixo)
    for component in path_obj.components() {
        match component {
            Component::ParentDir => {
                return Err(StorageError::InvalidPath(format!("path traversal detetado: {}", path)));
            }
            Component::CurDir => {}
            Component::Normal(_) => {}
            Component::RootDir | Component::Prefix(_) => {
                return Err(StorageError::InvalidPath(format!("componente de raiz/prefixo não permitido: {}", path)));
            }
        }
    }

    if path.len() > 4096 {
        return Err(StorageError::InvalidPath("caminho excede o limite máximo".to_string()));
    }

    Ok(path_obj.to_path_buf())
}

pub fn save_secure_vault<T: Serialize>(path: &str, data: &T, password: &[u8]) -> Result<(), StorageError> {
    let path_obj = validate_vault_path(path)?;

    let parent_dir = path_obj.parent().unwrap_or_else(|| Path::new("."));
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir)?;
    }

    let aad = path.as_bytes();
    let encrypted_bytes = encrypt_payload(data, password, aad)?;

    let mut temp_file = NamedTempFile::new_in(parent_dir)?;
    temp_file.write_all(&encrypted_bytes)?;
    temp_file.as_file().sync_all()?;

    #[cfg(unix)]
    {
        let mut perms = fs::metadata(temp_file.path())?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(temp_file.path(), perms)?;
    }

    temp_file.persist(&path_obj).map_err(|e| e.error)?;
    Ok(())
}

pub fn load_secure_vault<T: DeserializeOwned>(path: &str, password: &[u8]) -> Result<T, StorageError> {
    let path_obj = Path::new(path);
    if !path_obj.exists() {
        return Err(StorageError::NotFound);
    }

    let metadata = fs::metadata(path_obj)?;
    let file_size = metadata.len();

    if file_size == 0 || file_size > MAX_VAULT_FILE_SIZE {
        return Err(StorageError::InvalidFileSize);
    }

    let encrypted_bytes = fs::read(path_obj)?;
    let aad = path.as_bytes();
    
    let data: T = decrypt_payload(&encrypted_bytes, password, aad)?;
    Ok(data)
}
