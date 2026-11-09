use sec_ast::AstNode;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    format: String, // Força a Ollama a retornar JSON estruturado
}

#[derive(Deserialize, Debug)]
struct OllamaResponse {
    response: String,
}

pub struct OllamaBridge {
    endpoint: String,
    model: String,
}

impl OllamaBridge {
    pub fn new(model: &str) -> Self {
        Self {
            endpoint: "http://localhost:11434/api/generate".to_string(),
            model: model.to_string(),
        }
    }

    /// Traduz linguagem natural em um nó AST determinístico usando a Ollama local
    pub fn translate_intent_to_ast(&self, user_prompt: &str) -> Result<AstNode, Box<dyn std::error::Error>> {
        let system_instructions = format!(
            "És um tradutor estrito de intenções financeiras para nós AST em JSON. \
            O utilizador vai dar uma ordem em linguagem natural. \
            Deves devolver EXATAMENTE um JSON válido correspondente a uma das seguintes estruturas Rust enum AstNode: \
            1. PriceThreshold {{ \"symbol\": string, \"operator\": string (\">\", \"<\", \"==\"), \"target_price\": f64 }} \
            2. RiskCheck {{ \"max_loss_allowed\": f64, \"take_profit_target\": f64 }} \
            3. ExecutionAction {{ \"symbol\": string, \"side\": string (\"BUY\", \"SELL\"), \"quantity\": f64 }} \
            Não adiciones nenhum texto explicativo, apenas o JSON puro. \
            Instrução do utilizador: {}",
            user_prompt
        );

        let req_body = OllamaRequest {
            model: self.model.clone(),
            prompt: system_instructions,
            stream: false,
            format: "json".to_string(),
        };

        let resp = ureq::post(&self.endpoint)
            .send_json(&req_body)?
            .into_json::<OllamaResponse>()?;

        let ast_node: AstNode = serde_json::from_str(&resp.response)?;
        Ok(ast_node)
    }
}
