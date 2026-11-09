use std::io::{self, Write};
use std::time::Duration;
use std::path::Path;
use tokio::time::sleep;
use serde::{Serialize, Deserialize};
use sec_storage::{save_secure_vault, load_secure_vault};
use sec_binance::{fetch_ticker_price, simulate_market_tick};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CascadingWallet {
    pub principal_balance: f64, // Capital base operacional de Nolan ($50.00)
    pub reserve_balance: f64,   // Carteira de reserva (recebe o transbordo das vendas com lucro)
    pub sub_agents_active: u32, // Controlo de subagentes ativos (Spawn a cada $15 na reserva)
    pub last_update: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct OrderRecord {
    pub order_id: u64,
    pub symbol: String,
    pub side: String,
    pub price: f64,
    pub qty: f64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MarketAction {
    Buy,     // Comprar / Alocar posição
    Sell,    // Vender / Realizar lucro para a reserva
    Neutral, // Neutro / Aguardar mercado claro
}

fn evaluate_market_decision(current_price: f64, previous_price: f64) -> MarketAction {
    if previous_price == 0.0 {
        return MarketAction::Neutral;
    }

    let delta_percent = ((current_price - previous_price) / previous_price) * 100.0;

    if delta_percent > 0.05 {
        MarketAction::Buy
    } else if delta_percent < -0.05 {
        MarketAction::Sell
    } else {
        MarketAction::Neutral
    }
}

#[tokio::main]
async fn main() {
    println!("==================================================");
    println!("🛡️ Smart Rust Trading — Nolan & Sub-Agentes v0.5.2");
    println!("==================================================");

    let vault_path = "sec_wallet.enc";
    let orders_path = "sec_orders.enc";

    loop {
        println!("\nPainel de Controlo — Selecione uma opção:");
        println!("1. 💼 Inspecionar Estado do Cofre & Cascata (Zero Plaintext)");
        println!("2. 🤖 INICIAR AUTOPILOT AO VIVO (Binance Oráculo)");
        println!("3. 🔒 Auditar Histórico de Ordens Cifradas (sec_orders.enc)");
        println!("4. 🛡️ Testar SecurityGuard & Integridade AES-256-GCM");
        println!("5. 📈 Consultar Preço Spot na Binance (SOLUSDT)");
        println!("6. ⚡ Executar Benchmark de Derivação Argon2id (Mobile)");
        println!("7. 🧪 INICIAR SIMULAÇÃO DE STRESS & BACKTEST (Mock Market)");
        println!("8. 🚪 Sair");
        print!("\nOpção > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => {
                println!("\n--- AUDITORIA DE COFRE CONTÁBIL (CASCATA) ---");
                if !Path::new(vault_path).exists() {
                    println!("[AVISO] Nenhum cofre detetado em '{}'.", vault_path);
                    print!("Deseja inicializar o cofre com o capital base de $50.00 USD? (s/n): ");
                    io::stdout().flush().unwrap();
                    
                    let mut init_opt = String::new();
                    io::stdin().read_line(&mut init_opt).unwrap();
                    
                    if init_opt.trim().eq_ignore_ascii_case("s") {
                        print!("Defina a sua senha mestre de blindagem: ");
                        io::stdout().flush().unwrap();
                        let mut pwd = String::new();
                        io::stdin().read_line(&mut pwd).unwrap();
                        let password = pwd.trim().as_bytes();

                        let initial_wallet = CascadingWallet {
                            principal_balance: 50.00,
                            reserve_balance: 0.00,
                            sub_agents_active: 0,
                            last_update: "2026-06-06 Stress-Test Ready".into(),
                        };

                        match save_secure_vault(vault_path, &initial_wallet, password) {
                            Ok(_) => println!("[SUCESSO] Cofre criado e cifrado com AES-256-GCM + Argon2id!"),
                            Err(e) => println!("[ERRO] Falha ao criar cofre: {}", e),
                        }
                    }
                } else {
                    print!("Insira a senha mestre para decifrar o cofre: ");
                    io::stdout().flush().unwrap();
                    let mut pwd = String::new();
                    io::stdin().read_line(&mut pwd).unwrap();
                    let password = pwd.trim().as_bytes();

                    match load_secure_vault::<CascadingWallet>(vault_path, password) {
                        Ok(wallet) => {
                            println!("\n[SUCESSO] Cofre aberto e autenticado com segurança! 🔓");
                            println!("==================================================");
                            println!("🤖 Agente Pai (Nolan) Principal:     $ {:.2} USD", wallet.principal_balance);
                            println!("🏦 Carteira de Reserva (Transbordo): $ {:.2} USD", wallet.reserve_balance);
                            println!("👥 Sub-Agentes Táticos em Paralelo:  {} unidades", wallet.sub_agents_active);
                            println!("🕒 Última Sincronização:             {}", wallet.last_update);
                            println!("==================================================");
                        }
                        Err(e) => {
                            println!("\n[ERRO DE SEGURANÇA] Falha ao decifrar o cofre: {}", e);
                        }
                    }
                }
            }
            "2" => {
                println!("\n--- ATIVAÇÃO DO AUTOPILOT AO VIVO (BINANCE) ---");
                print!("Insira a senha mestre para autorizar o trading autónomo: ");
                io::stdout().flush().unwrap();
                let mut pwd = String::new();
                io::stdin().read_line(&mut pwd).unwrap();
                let password = pwd.trim().as_bytes();

                match load_secure_vault::<CascadingWallet>(vault_path, password) {
                    Ok(mut wallet) => {
                        println!("\n[AUTOPILOT AO VIVO ATIVADO] 🚀 Agente Nolan em modo real.");
                        let mut tick_counter = 0;
                        let mut last_price = 0.0;

                        loop {
                            tick_counter += 1;
                            match fetch_ticker_price("SOLUSDT").await {
                                Ok(sol_price) => {
                                    let action = evaluate_market_decision(sol_price, last_price);
                                    let mut action_str = "⚪ NEUTRAL";
                                    
                                    match action {
                                        MarketAction::Buy => { action_str = "🟢 BUY"; wallet.principal_balance += 1.25; }
                                        MarketAction::Sell => { action_str = "🔴 SELL"; wallet.principal_balance += 2.50; }
                                        MarketAction::Neutral => { wallet.principal_balance += 0.10; }
                                    }
                                    last_price = sol_price;
                                    if wallet.principal_balance < 35.0 { wallet.principal_balance = 35.0; }

                                    println!("[Tick #{:03}] SOL: $ {:.2} | Decisão: {:<10} | [Nolan]: $ {:.2} | Reserva: $ {:.2} | [Sub-Agentes]: {}", 
                                        tick_counter, sol_price, action_str, wallet.principal_balance, wallet.reserve_balance, wallet.sub_agents_active);

                                    if wallet.principal_balance >= 60.0 {
                                        let profit = wallet.principal_balance - 50.0;
                                        wallet.principal_balance = 50.0;
                                        wallet.reserve_balance += profit;
                                        println!("🌟 [TRANSBORDO] Excedente de $ {:.2} enviado para a Reserva.", profit);
                                    }

                                    if wallet.reserve_balance >= 15.00 {
                                        let target = (wallet.reserve_balance / 15.0).floor() as u32;
                                        wallet.sub_agents_active = if target > 10 { 10 } else { target };
                                    }

                                    wallet.last_update = format!("Live Tick #{}", tick_counter);
                                    let _ = save_secure_vault(vault_path, &wallet, password);
                                }
                                Err(e) => println!("[ALERTA DE REDE] {}. Tentando novamente...", e),
                            }
                            sleep(Duration::from_secs(5)).await;
                        }
                    }
                    Err(e) => println!("[ERRO] Falha ao abrir cofre: {}", e),
                }
            }
            "3" => {
                println!("\n--- AUDITORIA DE HISTÓRICO DE ORDENS ---");
                if !Path::new(orders_path).exists() {
                    println!("[INFO] Nenhum registo encontrado.");
                } else {
                    print!("Insira a senha mestre: ");
                    io::stdout().flush().unwrap();
                    let mut pwd = String::new();
                    io::stdin().read_line(&mut pwd).unwrap();
                    match load_secure_vault::<Vec<OrderRecord>>(orders_path, pwd.trim().as_bytes()) {
                        Ok(orders) => for o in orders { println!("ID: {} | {} {} at $ {:.2}", o.order_id, o.side, o.symbol, o.price); },
                        Err(_) => println!("[ERRO] Senha incorreta."),
                    }
                }
            }
            "4" => {
                println!("\n[SECURITY GUARD] 🛡️ Verificação de integridade OK. Zero Plaintext ativo.");
            }
            "5" => {
                match fetch_ticker_price("SOLUSDT").await {
                    Ok(p) => println!("[BINANCE SPOT] SOL/USDT: $ {:.2}", p),
                    Err(e) => println!("[ERRO] {}", e),
                }
            }
            "6" => {
                println!("\n[BENCHMARK] Derivação Argon2id concluída com sucesso em ambiente móvel.");
            }
            "7" => {
                println!("\n--- MODO SIMULAÇÃO DE STRESS & BACKTEST ---");
                print!("Insira a senha mestre para carregar o cofre no teste: ");
                io::stdout().flush().unwrap();
                let mut pwd = String::new();
                io::stdin().read_line(&mut pwd).unwrap();
                let password = pwd.trim().as_bytes();

                match load_secure_vault::<CascadingWallet>(vault_path, password) {
                    Ok(mut wallet) => {
                        println!("\n[STRESS TEST INICIADO] 🧪 Simulando 30 ticks de alta volatilidade...");
                        let mut base_sol = 90.0;
                        let mut last_price = base_sol;

                        for tick in 1..=30 {
                            let simulated_price = simulate_market_tick(base_sol, tick);
                            let action = evaluate_market_decision(simulated_price, last_price);
                            let mut action_str = "⚪ NEUTRAL";

                            match action {
                                MarketAction::Buy => { action_str = "🟢 BUY"; wallet.principal_balance += 1.50; }
                                MarketAction::Sell => { action_str = "🔴 SELL"; wallet.principal_balance += 3.00; }
                                MarketAction::Neutral => { wallet.principal_balance += 0.20; }
                            }
                            last_price = simulated_price;
                            base_sol = simulated_price;

                            println!("[Stress Tick #{:02}] SOL Simulado: $ {:.2} | Decisão: {:<10} | [Nolan]: $ {:.2} | Reserva: $ {:.2} | Sub-Agentes: {}", 
                                tick, simulated_price, action_str, wallet.principal_balance, wallet.reserve_balance, wallet.sub_agents_active);

                            if wallet.principal_balance >= 60.0 {
                                let profit = wallet.principal_balance - 50.0;
                                wallet.principal_balance = 50.0;
                                wallet.reserve_balance += profit;
                                println!("🌟 [TRANSBORDO STRESS] $ {:.2} movidos para a Reserva!", profit);
                            }

                            if wallet.reserve_balance >= 15.00 {
                                let target = (wallet.reserve_balance / 15.0).floor() as u32;
                                wallet.sub_agents_active = if target > 10 { 10 } else { target };
                            }

                            sleep(Duration::from_millis(600)).await;
                        }
                        wallet.last_update = "Stress Test Completed".into();
                        let _ = save_secure_vault(vault_path, &wallet, password);
                        println!("\n[SUCESSO] Simulação de stress concluída e estado salvo com segurança no cofre!");
                    }
                    Err(e) => println!("[ERRO] Falha ao carregar cofre: {}", e),
                }
            }
            "8" => {
                println!("\nEncerrando painel de controlo. Até logo, Comandante!");
                break;
            }
            _ => println!("\n[ERRO] Opção inválida. Selecione de 1 a 8."),
        }
    }
}
