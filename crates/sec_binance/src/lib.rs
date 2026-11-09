use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct BinanceTicker {
    #[allow(dead_code)]
    symbol: String,
    price: String,
}

/// Consulta o preço spot real na API pública da Binance com tratamento de resiliência
pub async fn fetch_ticker_price(symbol: &str) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);
    
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
