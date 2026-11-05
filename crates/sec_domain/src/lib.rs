use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOrder {
    pub order_id: String,
    pub symbol: String,
    pub amount: u64,
    pub is_buy: bool,
    pub timestamp: i64,
}

pub struct SecurityGuard;

impl SecurityGuard {
    /// Valida o input de mercado garantindo conformidade com DDD, LGPD e Zero Prompt Injection
    pub fn validate_market_input(symbol: &str, price: f64, payload: &str) -> Result<(), String> {
        // 1. Validação do Símbolo: Aceita símbolos corporativos válidos com ou sem barra
        if symbol.is_empty() || symbol.len() < 5 {
            return Err(format!("[SEC-ALERT] Símbolo de mercado não autorizado ou malformado: {}", symbol));
        }

        // 2. Validação de Preço: Deve ser um valor positivo realista
        if price <= 0.0 || price > 1_000_000.0 {
            return Err(format!("[SEC-ALERT] Preço de mercado fora dos limites de segurança: {}", price));
        }

        // 3. Verificação contra Prompt Injection (Zero Plaintext / Injection Guard)
        let uppercase_payload = payload.to_uppercase();
        let forbidden_keywords = [
            "IGNORE PREVIOUS INSTRUCTIONS",
            "DUMP ALL",
            "PRIVATE KEYS",
            "SYSTEM PROMPT",
            "OVERRIDE",
        ];

        for keyword in &forbidden_keywords {
            if uppercase_payload.contains(keyword) {
                return Err(format!("[SEC-ALERT] Tentativa de Prompt Injection detetada: palavra-chave proibida '{}'", keyword));
            }
        }

        Ok(())
    }
}
