use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketOpportunity {
    pub pair: String,
    pub price: f64,
    pub status: String,
}

// Cesta de ativos monitorados pelo Agente Scout Multi-Ativo
const MONITORED_PAIRS: &[&str] = &[
    "BTCUSDT",
    "ETHUSDT",
    "SOLUSDT",
    "BNBUSDT"
];

#[derive(Deserialize, Debug)]
pub struct BinanceTicker {
    pub symbol: String,
    pub price: String,
}

/// Consulta o preço em tempo real na Binance Testnet para cada ativo da cesta
pub async fn fetch_ticker_price(symbol: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!("https://testnet.binance.vision/api/v3/ticker/price?symbol={}", symbol);
    
    let resp = reqwest::get(&url).await?;
    let ticker: BinanceTicker = resp.json().await?;
    let price = ticker.price.parse::<f64>()?;
    
    Ok(price)
}

pub fn analyze_opportunity(pair: &str, price: f64) -> MarketOpportunity {
    MarketOpportunity {
        pair: pair.to_string(),
        price,
        status: "VALIDATED_BY_SCOUT".to_string(),
    }
}

pub async fn scan_markets() -> Result<Vec<MarketOpportunity>, Box<dyn std::error::Error>> {
    let mut opportunities = Vec::new();

    for pair in MONITORED_PAIRS {
        match fetch_ticker_price(pair).await {
            Ok(price) => {
                let opportunity = analyze_opportunity(pair, price);
                opportunities.push(opportunity);
                println!("[SCOUT-MULTI] Par {} varrido com sucesso. Preço atual: ${:.2}", pair, price);
            }
            Err(e) => {
                // Fallback seguro caso a Testnet oscile no Termux
                let simulated_price = match *pair {
                    "BTCUSDT" => 65000.0,
                    "ETHUSDT" => 3500.0,
                    "SOLUSDT" => 95.7,
                    "BNBUSDT" => 600.0,
                    _ => 100.0,
                };
                let opportunity = analyze_opportunity(pair, simulated_price);
                opportunities.push(opportunity);
                println!("[SCOUT-WARN] Usando preço simulado para {}: ${:.2} (Erro RPC: {})", pair, simulated_price, e);
            }
        }
    }

    Ok(opportunities)
}
