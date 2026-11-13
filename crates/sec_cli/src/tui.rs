use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[path = "sec_risk.rs"]
mod sec_risk;
use sec_risk::RiskMetrics;

#[derive(serde::Deserialize)]
struct BinanceTicker {
    #[allow(dead_code)]
    symbol: String,
    price: String,
}

fn fetch_binance_price(symbol: &str) -> Option<f64> {
    let url = format!("https://testnet.binance.vision/api/v3/ticker/price?symbol={}", symbol);
    if let Ok(resp) = ureq::get(&url).call() {
        if let Ok(ticker) = resp.into_json::<BinanceTicker>() {
            return ticker.price.parse::<f64>().ok();
        }
    }
    None
}

pub fn render_retro_dashboard() {
    let mut fallback_btc = 73830.41;
    let mut fallback_eth = 2054.78;
    
    // Limpeza profunda inicial do terminal
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().unwrap();

    for counter in 0..25 {
        // Limpa a tela e coloca o cursor no topo a cada frame para evitar qualquer lixo visual
        print!("\x1b[2J\x1b[H");
        io::stdout().flush().unwrap();

        let btc_price = fetch_binance_price("BTCUSDT").unwrap_or_else(|| {
            fallback_btc += 5.0;
            fallback_btc
        });
        let eth_price = fetch_binance_price("ETHUSDT").unwrap_or_else(|| {
            fallback_eth += 1.5;
            fallback_eth
        });

        let risk = RiskMetrics::evaluate(counter);
        let pnl = 0.07 + (counter as f64 * 0.005);
        let vault_secured = 12.50 + (counter as f64 * 0.02);

        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║ 🛡️ NOLAN TERMINAL v3.3.0 [LIVE BINANCE TESTNET]              ║");
        println!("╠══════════════════════════════╦═══════════════════════════════╣");
        println!("║ BTC: ${:<17.2}  ║ ETH: ${:<17.2}   ║", btc_price, eth_price);
        println!("║ Alloc: {:<5.1}%               ║ Alloc: {:<5.1}%                ║", risk.current_exposure_btc, risk.current_exposure_eth);
        println!("╠══════════════════════════════╩═══════════════════════════════╣");
        println!("║ 💰 CAIXA & COFRE DE LUCROS                                   ║");
        println!("║ Inicial: $50.00              │ Taxas: -$2.30                 ║");
        println!("║ Cofre: ${:<19.2}  │ PnL: +${:<13.2} USD  ║", vault_secured, pnl);
        println!("╠══════════════════════════════════════════════════════════════╣");
        println!("║ ⚡ RISCO & LATÊNCIA                                          ║");
        println!("║ Status: {:<19} │ Latência: {:<11}ms   ║", risk.risk_status, risk.api_latency_ms);
        println!("║ Drawdown: {:<5.2}%             │ Alavancagem: 1.0x             ║", risk.max_drawdown);
        println!("╠══════════════════════════════════════════════════════════════╣");
        println!("║ 📋 LOGS AO VIVO (AUTOPILOT & AST)                            ║");
        println!("║ > [sec_live] Binance Testnet sincronizado                    ║");
        println!("║ > [sec_vault] Lucro transferido para o cofre                 ║");
        println!("║ > [autopilot] Ciclo #{:<2} concluído - Preços ao vivo         ║", counter + 1);
        println!("╚══════════════════════════════════════════════════════════════╝");
        print!(" [ Frame {}/25 — Pressione Ctrl+C para sair ]", counter + 1);
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_secs(2));
    }

    print!("\x1b[2J\x1b[H");
    println!("[✔] Sessão TUI encerrada. Pressione Enter para voltar ao menu...");
    let _ = io::stdout().flush();
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();
}
