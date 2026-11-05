use sec_storage::SecureStorageManager;
use sec_domain::{TradeOrder, SecurityGuard};
use sec_binance::BinanceClient;
use std::path::Path;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone)]
struct SubAgent {
    id: usize,
    persona: String,
    status: String,
}

#[tokio::main]
async fn main() {
    println!("🤖 Smart Rust Trading — Agente Nolan & Agente Scout Multi-Ativo Iniciado!");
    println!("🛡️ Blindagem: DDD, SOA, LGPD & Criptografia AES-256-GCM ativa.\n");

    let vault_password = "NolanSecureMasterPassword2026!";
    let wallet_path = "sec_wallet.enc";
    let reserve_path = "sec_reserve.enc";
    let orders_path = "sec_orders.enc";

    let storage = SecureStorageManager::new(vault_password);

    if !Path::new(wallet_path).exists() {
        let initial_state = sec_storage::WalletState {
            owner: "Nolan-Autonomous-Agent".to_string(),
            balance_usd: 50.00,
            created_at: chrono::Utc::now().timestamp(),
        };
        let _ = storage.save_wallet(&initial_state, wallet_path);
    }

    if !Path::new(reserve_path).exists() {
        let reserve_state = sec_storage::WalletState {
            owner: "Nolan-Reserve-Vault".to_string(),
            balance_usd: 0.00,
            created_at: chrono::Utc::now().timestamp(),
        };
        let _ = storage.save_wallet(&reserve_state, reserve_path);
    }

    let binance = BinanceClient::new("testnet_api_key_placeholder", "testnet_secret_key_placeholder", true);
    
    // Cluster com as personas fundamentais
    let mut cluster: Vec<SubAgent> = vec![
        SubAgent { id: 1, persona: "Scout (Varredura Multi-Ativo: BTC, ETH, SOL, BNB)".to_string(), status: "Ativo".to_string() },
        SubAgent { id: 2, persona: "Risk Guardian (Validação de Risco)".to_string(), status: "Ativo".to_string() },
        SubAgent { id: 3, persona: "Vault (Gestão de Cofre Cifrado)".to_string(), status: "Ativo".to_string() },
        SubAgent { id: 4, persona: "Analista Tático (Decisão de Mercado)".to_string(), status: "Ativo".to_string() },
    ];

    let max_subagents = 10;
    let tracked_symbols = vec!["SOLUSDT", "BTCUSDT", "ETHUSDT", "BNBUSDT"];
    let mut cycle = 1;
    let mut last_prices: std::collections::HashMap<String, f64> = std::collections::HashMap::new();

    println!("👥 [SOA] Cluster inicial carregado. Scout configurado para monitorizar {} ativos em simultâneo.\n", tracked_symbols.len());

    loop {
        let mut wallet = storage.load_wallet(wallet_path).unwrap_or(sec_storage::WalletState {
            owner: "Nolan-Autonomous-Agent".to_string(),
            balance_usd: 50.00,
            created_at: chrono::Utc::now().timestamp(),
        });

        let reserve = storage.load_wallet(reserve_path).unwrap_or(sec_storage::WalletState {
            owner: "Nolan-Reserve-Vault".to_string(),
            balance_usd: 0.00,
            created_at: chrono::Utc::now().timestamp(),
        });

        println!("--------------------------------------------------------------------------");
        println!("🔄 [CICLO #{}] [Cluster: {} Sub-agentes] 🔭 Scout a varrer mercados...", cycle, cluster.len());

        // Varredura Multi-Ativo pelo Agente Scout
        let mut best_opportunity: Option<(String, f64, String)> = None;
        let mut max_variance: f64 = 0.0;

        for symbol in &tracked_symbols {
            match binance.get_ticker_price(symbol).await {
                Ok(price) => {
                    let prev_price = *last_prices.get(*symbol).unwrap_or(&price);
                    let variance = ((price - prev_price) / prev_price).abs();
                    
                    let trend = if price > prev_price {
                        "📈 ALTA"
                    } else if price < prev_price {
                        "📉 BAIXA"
                    } else {
                        "⚖️ ESTÁVEL"
                    };

                    println!("  ↳ [{}] Preço: ${:.2} USD | Tendência: {}", symbol, price, trend);

                    if variance >= max_variance {
                        max_variance = variance;
                        best_opportunity = Some((symbol.to_string(), price, trend.to_string()));
                    }

                    last_prices.insert(symbol.to_string(), price);
                }
                Err(_) => {
                    println!("  ↳ [{}] ⚠️ Falha ao obter cotação na Binance.", symbol);
                }
            }
        }

        if let Some((target_symbol, target_price, target_trend)) = best_opportunity {
            println!("🎯 [SCOUT] Melhor oportunidade detetada: {} a ${:.2} USD ({})", target_symbol, target_price, target_trend);
            println!("💰 Cofre Principal: ${:.2} USD  |  🏦 Cofre Reserva: ${:.2} USD", wallet.balance_usd, reserve.balance_usd);
            println!("--------------------------------------------------------------------------");

            let payload = format!("Multi-asset feed verified for {}. Executing strategy.", target_symbol);

            match SecurityGuard::validate_market_input(&target_symbol, target_price, &payload) {
                Ok(_) => {
                    wallet.balance_usd += 2.00; 
                    
                    if wallet.balance_usd > 60.00 {
                        let surplus = wallet.balance_usd - 60.00;
                        wallet.balance_usd = 60.00;

                        let mut res_mut = reserve;
                        res_mut.balance_usd += surplus;
                        let _ = storage.save_wallet(&res_mut, reserve_path);
                        println!("💰 [EXCEDENTE] Transferidos ${:.2} USD de lucro excedente para o cofre de reserva!", surplus);

                        if res_mut.balance_usd >= 15.00 && cluster.len() < max_subagents {
                            res_mut.balance_usd -= 15.00;
                            let _ = storage.save_wallet(&res_mut, reserve_path);
                            
                            let new_id = cluster.len() + 1;
                            let new_persona = format!("Analista Especialista em {} #{}", target_symbol, new_id);
                            cluster.push(SubAgent {
                                id: new_id,
                                persona: new_persona.clone(),
                                status: "Ativo".to_string(),
                            });

                            println!("\n🚀 [AUTO-DEPLOY] Sub-agente #{} ({}) instanciado no cluster SOA!", new_id, new_persona);
                            println!("📊 Total de sub-agentes ativos: {}/{}\n", cluster.len(), max_subagents);
                        }
                    }

                    let _ = storage.save_wallet(&wallet, wallet_path);

                    let order = TradeOrder {
                        order_id: format!("NOLAN-MULTI-{}-{}", target_symbol, chrono::Utc::now().timestamp()),
                        symbol: target_symbol,
                        amount: 500_000,
                        is_buy: true,
                        timestamp: chrono::Utc::now().timestamp(),
                    };
                    let _ = storage.append_encrypted_order(&order, orders_path);
                }
                Err(e) => println!("[SEC-GUARD-WARN] Ameaça bloqueada pelo Risk Guardian: {}", e),
            }
        }

        cycle += 1;
        sleep(Duration::from_secs(5)).await;
    }
}
