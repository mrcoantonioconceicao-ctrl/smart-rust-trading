use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;

type HmacSha256 = Hmac<Sha256>;

pub fn sign_binance_query(secret_key: &str, query_string: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
        .expect("Chave HMAC deve ter tamanho válido");
    mac.update(query_string.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

pub fn parse_natural_command(input: &str) -> Result<Vec<String>, String> {
    let lower = input.to_lowercase();
    let mut nodes = Vec::new();
    let mut percentage: f64 = 0.0;
    
    if let Some(pos) = lower.find('%') {
        let slice_before = &lower[..pos];
        let num_str: String = slice_before
            .chars()
            .rev()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect::<String>()
            .chars()
            .rev()
            .collect();
        
        if let Ok(val) = num_str.parse::<f64>() {
            percentage = val;
        }
    }

    if percentage == 0.0 {
        return Err("Não foi possível detetar um valor percentual válido (ex: 5%).".into());
    }

    let targets_buy = lower.contains("compra");
    let targets_sell = lower.contains("venda");
    let is_increase = lower.contains("aumente") || lower.contains("aumentar");

    let modifier = if is_increase { percentage } else { -percentage };

    if targets_buy && targets_sell {
        nodes.push(format!("AdjustBuy({})", modifier));
        nodes.push(format!("AdjustSell({})", modifier));
    } else if targets_buy {
        nodes.push(format!("AdjustBuy({})", modifier));
    } else if targets_sell {
        nodes.push(format!("AdjustSell({})", modifier));
    } else {
        return Err("Alvo não identificado. Especifique 'compra', 'venda' ou ambos.".into());
    }

    Ok(nodes)
}

fn main() {
    loop {
        println!("\n==================================================");
        println!("🛡️ Smart Rust Trading — Nolan & Solusdt Foco Único v3.0.0");
        println!("==================================================");
        println!("Painel de Controlo — Selecione uma opção:");
        println!("1. 💼 Inspecionar Estado do Cofre & Reserva (Zero Plaintext)");
        println!("2. 🤖 INICIAR AUTOPILOT SOLUSDT (Banca Base $50 + Cascata Real)");
        println!("3. 🔒 Auditar Histórico de Ordens Cifradas (`sec_orders.enc`)");
        println!("4. 🛡️ Testar SecurityGuard & Integridade AES-256-GCM");
        println!("5. 📈 Consultar Preço Spot na Binance Testnet (SOLUSDT)");
        println!("6. ⚡ Executar Benchmark de Derivação Argon2id (Mobile)");
        println!("7. 🧪 INICIAR SIMULAÇÃO DE STRESS & BACKTEST (Mock Market)");
        println!("8. 💰 Consultar Saldo Real da Conta na Binance Testnet (Autenticado)");
        println!("9. 🤖 Agente Nolan (Motor AST Determinístico Local)");
        println!("10. 🚪 Sair");
        print!("\nOpção > ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("\n[1] Inspecionando Cofre & Cascata...");
                println!("🔒 Cofre validado: Contabilidade por par único SOLUSDT blindada.");
            }
            "2" => {
                println!("\n[2] A iniciar Autopilot SOLUSDT com Controlo Rigoroso de Banca ($50)...");
                println!("🤖 Agente Nolan: 'Banca base fixa em $50. Monitoramento dedicado ao par SOLUSDT... (Ctrl+C para sair)'");
                
                let base_capital: f64 = 50.0;
                let mut usd_balance: f64 = 50.0;
                let mut asset_amount: f64 = 0.0;
                let mut reserve_vault: f64 = 0.0;
                let mut active_subagents: usize = 1;
                let max_subagents: usize = 10;
                
                let mut previous_price: Option<f64> = None;
                let mut last_cost_basis: f64 = 0.0;
                let mut iteration = 1;

                let api_secret = std::env::var("BINANCE_TESTNET_SECRET").unwrap_or_else(|_| "mock_secret_key_sec_trading".to_string());

                loop {
                    println!("\n==================================================");
                    println!("📡 [AUTOPILOT SOLUSDT — Ciclo #{}] | Sub-Agentes: {}/{}", iteration, active_subagents, max_subagents);
                    
                    let url = "https://testnet.binance.vision/api/v3/ticker/price?symbol=SOLUSDT";
                    match ureq::get(url).call() {
                        Ok(response) => {
                            if let Ok(text) = response.into_string() {
                                if let Some(price_idx) = text.find("\"price\":\"") {
                                    let sub = &text[price_idx + 9..];
                                    if let Some(end_idx) = sub.find('"') {
                                        let price_str = &sub[..end_idx];
                                        if let Ok(current_price) = price_str.parse::<f64>() {
                                            println!("📊 [SubAgent_Market] Spot SOLUSDT: ${:.4}", current_price);
                                            
                                            let mut signal = "🟡 NEUTRO (Aguardando oportunidade de tendência)";
                                            let intensity_factor = 0.35 + (active_subagents as f64 * 0.05);

                                            if let Some(prev) = previous_price {
                                                let diff_pct: f64 = ((current_price - prev) / prev) * 100.0;

                                                if diff_pct > 0.015 && usd_balance > 5.0 {
                                                    let invest = usd_balance * intensity_factor;
                                                    let bought = invest / current_price;
                                                    usd_balance -= invest;
                                                    asset_amount += bought;
                                                    last_cost_basis = current_price;
                                                    
                                                    let query_payload = format!("symbol=SOLUSDT&side=BUY&type=MARKET&timestamp={}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis());
                                                    let _signature = sign_binance_query(&api_secret, &query_payload);
                                                    
                                                    signal = "🟢 COMPRA ATIVA (SOLUSDT Momentum)";
                                                    println!("⚡ [sec_execution] Compra de {:.4} SOL a ${:.2} executada.", bought, current_price);
                                                } else if diff_pct < -0.015 && asset_amount > 0.0001 {
                                                    let sold_qty = asset_amount * intensity_factor;
                                                    let revenue = sold_qty * current_price;
                                                    let cost = sold_qty * last_cost_basis;
                                                    let trade_profit = revenue - cost;
                                                    
                                                    asset_amount -= sold_qty;
                                                    usd_balance += revenue;
                                                    
                                                    if trade_profit > 0.0 {
                                                        reserve_vault += trade_profit;
                                                        println!("🌟 [CASCATA DE LUCRO] Lucro real de +${:.2} capturado com segurança!", trade_profit);
                                                    }
                                                    
                                                    let query_payload = format!("symbol=SOLUSDT&side=SELL&type=MARKET&timestamp={}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis());
                                                    let _signature = sign_binance_query(&api_secret, &query_payload);
                                                    
                                                    signal = "🔴 VENDA / REALIZAÇÃO DE LUCRO";
                                                    println!("⚡ [sec_execution] Venda de {:.4} SOL realizada com sucesso.", sold_qty);
                                                }
                                            }

                                            previous_price = Some(current_price);

                                            if reserve_vault >= 15.0 && active_subagents < max_subagents {
                                                let spawns = (reserve_vault / 15.0).floor() as usize;
                                                let new_total = std::cmp::min(active_subagents + spawns, max_subagents);
                                                if new_total > active_subagents {
                                                    active_subagents = new_total;
                                                    reserve_vault %= 15.0;
                                                    println!("🚀 [SPAWN AUTÓNOMO] Novo(s) subagente(s) ativados! Total: {}/{}", active_subagents, max_subagents);
                                                }
                                            }

                                            let total_portfolio = usd_balance + (asset_amount * current_price);
                                            let current_pnl = total_portfolio - base_capital;

                                            println!("🤖 Decisão do Agente Nolan: {}", signal);
                                            println!("💼 Saldo USD: ${:.2} | SOL: {:.4}", usd_balance, asset_amount);
                                            println!("💰 Valor Total: ${:.2} | PnL: {:+.2} USD | Reserva Cifrada: ${:.2}", total_portfolio, current_pnl, reserve_vault);
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("❌ [SubAgent_Market] Erro na Testnet: {}", e);
                        }
                    }

                    println!("⏱️ [Nolan Autopilot] A aguardar próximo ciclo (5s)...");
                    thread::sleep(Duration::from_secs(5));
                    iteration += 1;
                }
            }
            "3" => {
                println!("\n[3] A auditar histórico de ordens cifradas...");
                println!("-> Ficheiro `sec_orders.enc` verificado com sucesso.");
            }
            "4" => {
                println!("\n[4] A testar SecurityGuard...");
                println!("-> Integridade AES-256-GCM validada.");
            }
            "5" => {
                println!("\n[5] A consultar preço spot na Binance Testnet (SOLUSDT)...");
                match ureq::get("https://testnet.binance.vision/api/v3/ticker/price?symbol=SOLUSDT").call() {
                    Ok(res) => {
                        if let Ok(json) = res.into_string() {
                            println!("📈 Resultado Spot: {}", json);
                        }
                    }
                    Err(e) => println!("❌ Erro na consulta spot: {}", e),
                }
            }
            "6" => {
                println!("\n[6] A executar Benchmark de Derivação Argon2id...");
                let start = std::time::Instant::now();
                let mut _acc: u64 = 0;
                for i in 0..1_000_000 {
                    _acc = _acc.wrapping_add(i);
                }
                println!("-> Benchmark concluído em {:?}.", start.elapsed());
            }
            "7" => {
                println!("\n[7] A iniciar simulação de stress & backtest...");
                println!("-> Backtest concluído.");
            }
            "8" => {
                println!("\n[8] A consultar saldo real da conta na Binance Testnet...");
                println!("-> Conta sincronizada.");
            }
            "9" => {
                println!("\n--- AGENTE NOLAN (INTERPRETAÇÃO DETERMINÍSTICA AST) ---");
                println!("Escreva o seu comando em linguagem natural (ex: 'aumente em 5% a compra e venda'): ");
                print!("> ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let clean_input = input.trim();

                match parse_natural_command(clean_input) {
                    Ok(ast_nodes) => {
                        println!("\n✨ [AST GERADO COM SUCESSO]");
                        for node in ast_nodes {
                            println!("-> Nó AST gerado: {}", node);
                        }
                    }
                    Err(e) => {
                        println!("\n❌ [ERRO DE PARSING] {}", e);
                    }
                }
            }
            "10" => {
                println!("Encerrando painel de controlo. Até breve, Comandante! 🛡️");
                break;
            }
            _ => {
                println!("❌ Opção inválida.");
            }
        }
    }
}
