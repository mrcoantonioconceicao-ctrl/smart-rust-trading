use reqwest;
use serde::Deserialize;

const BINANCE_BASE_URL: &str = "https://api.binance.com/api/v3/ticker/price";

#[derive(Deserialize, Debug)]
struct BinanceTicker {
    #[allow(dead_code)]
    symbol: String,
    price: String,
}

/// Símbolos de mercado válidos: alfanuméricos, opcionalmente com uma barra (ex: BTCUSDT, BTC/USDT).
/// Bloqueia injeção de parâmetros de consulta e path traversal na URL da API.
fn validate_symbol(symbol: &str) -> Result<(), String> {
    if symbol.is_empty() || symbol.len() > 20 {
        return Err(format!("Símbolo inválido: {}", symbol));
    }
    for ch in symbol.chars() {
        if !ch.is_ascii_alphanumeric() && ch != '/' {
            return Err(format!("Símbolo contém carateres não permitidos: {}", symbol));
        }
    }
    Ok(())
}

/// Consulta o preço spot real na API pública da Binance com tratamento de resiliência.
/// O símbolo é validado antes de ser interpolado na URL para evitar injeção de parâmetros.
pub async fn fetch_ticker_price(symbol: &str) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    validate_symbol(symbol)?;

    let url = format!("{}?symbol={}", BINANCE_BASE_URL, symbol);
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    let response = client.get(&url).send().await?;
    
    if !response.status().is_success() {
        return Err(format!("Erro na API da Binance: Status {}", response.status()).into());
    }

    let ticker: BinanceTicker = response.json().await?;
    let price: f64 = ticker.price.parse()?;
    
    Ok(price)
}

/// Oráculo de Simulação de Stress (Gera variações artificiais para testes da Matriz de Decisão)
pub fn simulate_market_tick(base_price: f64, step: usize) -> f64 {
    let oscillation = ((step as f64 * 0.5).sin()) * 1.5;
    let trend = if step % 10 < 5 { 0.2 } else { -0.2 };
    base_price + oscillation + trend
}
