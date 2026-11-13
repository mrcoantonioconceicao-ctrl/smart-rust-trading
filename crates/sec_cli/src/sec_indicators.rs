pub struct MarketMetrics {
    pub rsi: f64,
    pub ema: f64,
    pub macd: f64,
    pub atr: f64,
    pub vwap: f64,
}

impl MarketMetrics {
    // Calcula indicadores base com base no histórico de preços recebido por eventos
    pub fn calculate(prices: &[f64], volumes: &[f64]) -> Self {
        let current_price = prices.last().copied().unwrap_or(0.0);
        
        // Simulação / Cálculo base de EMA e RSI para o motor de decisão do Agente Nolan
        let ema = if prices.len() >= 2 {
            (prices[prices.len() - 1] * 0.2) + (prices[prices.len() - 2] * 0.8)
        } else {
            current_price
        };

        let rsi = 54.2; // Valor normalizado de teste/exemplo para o regime neutro/momentum
        let macd = 1.25; // Convergência/Divergência de momentum
        let atr = 45.50; // Volatilidade média de range
        
        // Cálculo VWAP básico se houver volumes
        let vwap = if !volumes.is_empty() {
            prices.iter().zip(volumes.iter()).map(|(p, v)| p * v).sum::<f64>() / volumes.iter().sum::<f64>()
        } else {
            current_price
        };

        Self {
            rsi,
            ema,
            macd,
            atr,
            vwap,
        }
    }
}
