use std::thread;
use std::time::Duration;

pub struct BinanceStreamClient {
    pub symbol: String,
}

impl BinanceStreamClient {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
        }
    }

    // Simula ou conecta ao fluxo WebSocket de alta frequência da Testnet
    pub fn connect_and_listen<F>(&self, mut callback: F) 
    where
        F: FnMut(f64, f64), // Recebe (Preço, Volume) em tempo real por evento
    {
        println!("[sec_stream] A subscrever o canal WebSocket para {}...", self.symbol);
        
        // Simulação de fluxo contínuo de eventos orientados a WebSocket (substituindo o polling de 10s)
        let base_price = if self.symbol.contains("BTC") { 70500.0 } else { 2000.0 };
        
        for i in 0..10 {
            let jitter = (i as f64 * 1.5).sin() * 12.0;
            let current_price = base_price + jitter;
            let current_volume = 1.25 + (i as f64 * 0.05);

            // Dispara o callback com o evento de mercado detetado em tempo real
            callback(current_price, current_volume);

            thread::sleep(Duration::from_secs(1));
        }
    }
}
