use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

pub struct SolanaClientManager {
    client: RpcClient,
    rpc_url: String,
}

impl SolanaClientManager {
    /// Inicializa o gestor de conexão RPC com a Solana
    pub fn new(rpc_url: &str) -> Self {
        let client = RpcClient::new(rpc_url.to_string());
        Self {
            client,
            rpc_url: rpc_url.to_string(),
        }
    }

    /// Obtém o saldo atual de uma conta em Lamports/SOL
    pub fn get_balance(&self, pubkey: &Pubkey) -> Result<u64, Box<dyn std::error::Error>> {
        let balance = self.client.get_balance(pubkey)?;
        Ok(balance)
    }

    /// Retorna a URL RPC ativa
    pub fn get_rpc_url(&self) -> &str {
        &self.rpc_url
    }
}
