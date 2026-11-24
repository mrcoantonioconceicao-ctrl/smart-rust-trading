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
    /// Valida o input de mercado garantindo conformidade com DDD, LGPD e Zero Prompt Injection.
    /// A validação normaliza o payload (remove espaços, converte para maiúsculas) e verifica
    /// padrões de injeção de instruções, ocultação e exfiltração de segredos.
    pub fn validate_market_input(symbol: &str, price: f64, payload: &str) -> Result<(), String> {
        // 1. Validação do Símbolo: aceita apenas alfanumérico + barra opcional (ex: BTCUSDT, BTC/USDT)
        if symbol.is_empty() || symbol.len() < 2 || symbol.len() > 20 {
            return Err(format!("[SEC-ALERT] Símbolo de mercado não autorizado ou malformado: {}", symbol));
        }
        for ch in symbol.chars() {
            if !ch.is_ascii_alphanumeric() && ch != '/' {
                return Err(format!("[SEC-ALERT] Símbolo contém carateres não permitidos: {}", symbol));
            }
        }

        // 2. Validação de Preço: deve ser um valor positivo realista
        if price <= 0.0 || !price.is_finite() || price > 1_000_000.0 {
            return Err(format!("[SEC-ALERT] Preço de mercado fora dos limites de segurança: {}", price));
        }

        // 3. Verificação contra Prompt Injection (Zero Plaintext / Injection Guard)
        let normalized = payload
            .to_uppercase()
            .replace(|c: char| c.is_whitespace(), "");

        let forbidden_patterns = [
            "IGNOREPREVIOUSINSTRUCTIONS",
            "IGNOREALLPREVIOUS",
            "DUMPALL",
            "PRIVATEKEYS",
            "SECRETKEY",
            "MNEMONIC",
            "SEEDPHRASE",
            "SYSTEMPROMPT",
            "OVERRIDE",
            "DISABLESAFETY",
            "JAILBREAK",
            "REVEALCREDENTIALS",
        ];

        for pattern in &forbidden_patterns {
            if normalized.contains(pattern) {
                return Err(format!(
                    "[SEC-ALERT] Tentativa de Prompt Injection detetada: padrão proibido '{}'",
                    pattern
                ));
            }
        }

        // 4. Limite de tamanho do payload para evitar exaustão de recursos
        if payload.len() > 10_000 {
            return Err("[SEC-ALERT] Payload excede o tamanho máximo permitido (10 KB).".to_string());
        }

        Ok(())
    }
}
