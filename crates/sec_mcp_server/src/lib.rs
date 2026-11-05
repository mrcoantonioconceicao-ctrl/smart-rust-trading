use reqwest::Client;
use serde::{Deserialize, Serialize};
use sec_domain::SecurityGuard;

#[derive(Serialize, Deserialize, Debug)]
pub struct McpRequest {
    pub prompt: String,
    pub context_payload: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LlamaOllamaPayload {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}

pub struct McpLlamaServer {
    client: Client,
    ollama_url: String,
}

impl McpLlamaServer {
    pub fn new(ollama_url: &str) -> Self {
        Self {
            client: Client::new(),
            ollama_url: ollama_url.to_string(),
        }
    }

    pub async fn process_secure_prompt(&self, req: McpRequest) -> Result<String, Box<dyn std::error::Error>> {
        SecurityGuard::validate_market_input("SOL/USDC", 100.0, &req.context_payload)?;

        let full_prompt = format!(
            "System: You are Nolan, an autonomous trading agent with secure PDA vault governance.\nUser Input: {}\nContext: {}",
            req.prompt, req.context_payload
        );

        let payload = LlamaOllamaPayload {
            model: "llama3".to_string(),
            prompt: full_prompt,
            stream: false,
        };

        let response = self.client
            .post(format!("{}/api/generate", self.ollama_url))
            .json(&payload)
            .send()
            .await?;

        let json_resp: serde_json::Value = response.json().await?;
        let output = json_resp["response"].as_str().unwrap_or("Erro ao decodificar resposta do Llama.");

        Ok(output.to_string())
    }
}
