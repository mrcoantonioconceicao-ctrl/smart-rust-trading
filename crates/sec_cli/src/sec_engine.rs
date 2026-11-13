use crate::sec_stream::BinanceStreamClient;
use crate::sec_indicators::MarketMetrics;

pub struct NolanEngine {
    symbol: String,
}

impl NolanEngine {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
        }
    }

    pub fn run_live_loop(&mut self) {
        println!("[sec_engine] A iniciar motor orientado a eventos para {}...", self.symbol);
        
        let client = BinanceStreamClient::new(&self.symbol);

        let mut prices: Vec<f64> = Vec::new();
        let mut volumes: Vec<f64> = Vec::new();

        client.connect_and_listen(|price, volume| {
            prices.push(price);
            volumes.push(volume);

            // Calcula os indicadores quantitativos em tempo real com base no fluxo de eventos
            let metrics = MarketMetrics::calculate(&prices, &volumes);
            
            println!(
                "📡 [Event] Preço: ${:.2} | Vol: {:.2} | EMA: ${:.2} | RSI: {:.1} | MACD: {:.2} | ATR: {:.2} | VWAP: ${:.2}", 
                price, volume, metrics.ema, metrics.rsi, metrics.macd, metrics.atr, metrics.vwap
            );
        });
    }
}
