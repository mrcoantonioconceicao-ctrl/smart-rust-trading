use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::str::FromStr;

#[derive(Debug)]
pub struct SolanaClientManager {
    client: RpcClient,
}

impl SolanaClientManager {
    /// Inicializa o cliente RPC da Solana (Ex: Cluster Devnet ou Mainnet)
    pub fn new(cluster_url: &str) -> Self {
        let client = RpcClient::new(cluster_url.to_string());
        Self { client }
    }

    /// Consulta o saldo de SOL de uma carteira pública on-chain
    pub fn get_balance(&self, pubkey_str: &str) -> Result<f64, String> {
        let pubkey = Pubkey::from_str(pubkey_str)
            .map_err(|e| format!("Erro ao parsear chave pública da Solana: {}", e))?;
        
        let lamports = self.client
            .get_balance(&pubkey)
            .map_err(|e| format!("Erro ao consultar saldo RPC na Solana: {}", e))?;
        
        // Conversão de Lamports para SOL (1 SOL = 1_000_000_000 Lamports)
        let sol_balance = lamports as f64 / 1_000_000_000.0;
        Ok(sol_balance)
    }

    /// Gera uma nova carteira temporária on-chain para simulações ou testes
    pub fn generate_new_wallet(&self) -> (String, String) {
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey().to_string();
        let secret_bytes = bs58::encode(keypair.to_bytes()).into_string();
        (pubkey, secret_bytes)
    }
}
