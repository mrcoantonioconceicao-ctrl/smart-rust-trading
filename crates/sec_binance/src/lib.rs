use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct BinancePriceTicker {
    symbol: String,
    price: String,
}

/// Consulta o preço atual de um par diretamente na API REST pública da Binance
pub async fn fetch_ticker_price(symbol: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Erro da API Binance: Status {}", response.status()).into());
    }

    let ticker: BinancePriceTicker = response.json().await?;
    let price = ticker.price.parse::<f64>()?;

    Ok(price)
}

/// Alias para manter compatibilidade
pub async fn fetch_live_price(symbol: &str) -> Result<f64, String> {
    fetch_ticker_price(symbol).await.map_err(|e| e.to_string())
}

/// Executa a varredura multi-ativo do Agente Nolan no mercado real
pub async fn run_nolan_scout_cycle() -> Result<Vec<(String, f64)>, String> {
    let symbols = vec!["BTCUSDT", "ETHUSDT", "SOLUSDT", "BNBUSDT"];
    let mut results = Vec::new();

    for symbol in symbols {
        match fetch_live_price(symbol).await {
            Ok(price) => results.push((symbol.to_string(), price)),
            Err(e) => return Err(format!("Falha ao varrer {}: {}", symbol, e)),
        }
    }

    Ok(results)
}
