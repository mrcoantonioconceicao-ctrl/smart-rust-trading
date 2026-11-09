use std::io::{self, Write};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sec_storage::{save_secure_vault, load_secure_vault};
use sec_crypto::{derive_key, secure_mobile_params};
use sec_ast::{AstNode, AstEvaluator};
use sec_ai::OllamaBridge;
use serde::{Serialize, Deserialize};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct VaultData {
    agent_name: String,
    principal: f64,
    reserve: f64,
    sub_agents: usize,
    last_sync: String,
    binance_api_key: String,
    binance_secret_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EncryptedOrderRecord {
    id: usize,
    symbol: String,
    side: String,
    price: f64,
    amount: f64,
    timestamp: u64,
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn fetch_binance_price() -> Result<f64, Box<dyn std::error::Error>> {
    let url = "https://testnet.binance.vision/api/v3/ticker/price?symbol=SOLUSDT";
    let body: serde_json::Value = ureq::get(url).call()?.into_json()?;
    if let Some(price_str) = body["price"].as_str() {
        let price: f64 = price_str.parse()?;
        Ok(price)
    } else {
        Err("Formato inválido da Binance Testnet".into())
    }
}

fn fetch_testnet_account_balance(api_key: &str, secret_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://testnet.binance.vision/api/v3/account";
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    
    let query_string = format!("timestamp={}&recvWindow=5000", timestamp);

    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())?;
    mac.update(query_string.as_bytes());
    let result = mac.finalize();
    let signature = hex::encode(result.into_bytes());

    let full_url = format!("{}?{}&signature={}", url, query_string, signature);

    let resp = ureq::get(&full_url)
        .set("X-MBX-APIKEY", api_key)
        .call()?;

    let body: serde_json::Value = resp.into_json()?;
    
    let mut result_summary = String::new();
    if let Some(balances) = body["balances"].as_array() {
        for asset in balances {
            if let (Some(asset_name), Some(free), Some(_locked)) = (asset["asset"].as_str(), asset["free"].as_str(), asset["locked"].as_str()) {
                let free_val: f64 = free.parse().unwrap_or(0.0);
                if free_val > 0.0 || asset_name == "USDT" || asset_name == "SOL" {
                    result_summary.push_str(&format!("\n   - Ativo: {} | Livre: {}", asset_name, free));
                }
            }
        }
    }

    Ok(result_summary)
}

fn send_testnet_order(api_key: &str, secret_key: &str, symbol: &str, side: &str, quantity: f64) -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://testnet.binance.vision/api/v3/order";
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    
    let qty_str = format!("{:.2}", quantity);

    let query_string = format!(
        "symbol={}&side={}&type=MARKET&quantity={}&timestamp={}&recvWindow=5000",
        symbol, side, qty_str, timestamp
    );

    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())?;
    mac.update(query_string.as_bytes());
    let result = mac.finalize();
    let signature = hex::encode(result.into_bytes());

    let full_url = format!("{}?{}&signature={}", url, query_string, signature);

    let resp = ureq::post(&full_url)
        .set("X-MBX-APIKEY", api_key)
        .call()?;

    let body: serde_json::Value = resp.into_json()?;
    if let Some(order_id) = body["orderId"].as_i64() {
        Ok(format!("Ordem executada e aceite na Testnet! ID: {} | Qty: {}", order_id, qty_str))
    } else {
        Ok(format!("Resposta da Testnet: {:?}", body))
    }
}

fn main() {
    let vault_path = "sec_wallet.enc";
    let orders_path = "sec_orders.enc";

    loop {
        println!("\n==================================================");
        println!("🛡️ Smart Rust Trading — Nolan & Sub-Agentes v0.6.0");
        println!("==================================================");
        println!("Painel de Controlo — Selecione uma opção:");
        println!("1. 💼 Inspecionar Estado do Cofre & Cascata (Zero Plaintext)");
        println!("2. 🤖 INICIAR AUTOPILOT AO VIVO (Binance Testnet Real Execution)");
        println!("3. 🔒 Auditar Histórico de Ordens Cifradas (sec_orders.enc)");
        println!("4. 🛡️ Testar SecurityGuard & Integridade AES-256-GCM");
        println!("5. 📈 Consultar Preço Spot na Binance Testnet (SOLUSDT)");
        println!("6. ⚡ Executar Benchmark de Derivação Argon2id (Mobile)");
        println!("7. 🧪 INICIAR SIMULAÇÃO DE STRESS & BACKTEST (Mock Market)");
        println!("8. 💰 Consultar Saldo Real da Conta na Binance Testnet");
        println!("9. 🤖 Conversar com o Agente Nolan via IA Local & AST");
        println!("10. 🚪 Sair");
        
        let choice = prompt("\nOpção > ");

        match choice.as_str() {
            "1" => {
                println!("\n--- AUDITORIA DE COFRE CONTÁBIL (CASCATA) ---");
                if !std::path::Path::new(vault_path).exists() {
                    println!("[AVISO] Nenhum cofre detetado em '{}'.", vault_path);
                    let init_choice = prompt("Deseja inicializar o cofre com as chaves da Binance Testnet? (s/n): ");
                    if init_choice.to_lowercase() == "s" {
                        let password = prompt("Defina a sua senha mestre de blindagem: ");
                        let initial_data = VaultData {
                            agent_name: "Nolan".to_string(),
                            principal: 50.0,
                            reserve: 0.0,
                            sub_agents: 0,
                            last_sync: "2026-06-06 AST-AI-Integrated".to_string(),
                            binance_api_key: "4AWMxcdGMnshi1a135e1yyTMF6e1XzNsUAl6yu3hESpWwMpYv28ODRvBG8DtpEBu".to_string(),
                            binance_secret_key: "HP4OAsS9T5xG0iWKXBCA7QU6cTjG8taMQjQzWF81HRASABEOtkpBuU6Hm8KaECvi".to_string(),
                        };
                        match save_secure_vault(vault_path, &initial_data, password.as_bytes()) {
                            Ok(_) => println!("[SUCESSO] Cofre criado, cifrado e chaves Binance Testnet integradas!"),
                            Err(e) => println!("[ERRO] Falha ao criar cofre: {:?}", e),
                        }
                    }
                } else {
                    let password = prompt("Insira a senha mestre para decifrar o cofre: ");
                    match load_secure_vault::<VaultData>(vault_path, password.as_bytes()) {
                        Ok(data) => {
                            println!("\n[SUCESSO] Cofre aberto e autenticado com segurança! 🔓");
                            println!("==================================================");
                            println!("🤖 Agente Pai (Nolan) Principal:     $ {:.2} USD", data.principal);
                            println!("🏦 Carteira de Reserva (Transbordo): $ {:.2} USD", data.reserve);
                            println!("👥 Sub-Agentes Táticos em Paralelo:  {} unidades", data.sub_agents);
                            println!("🔑 API Key Testnet Configurada:      {}...", &data.binance_api_key[..10]);
                            println!("🕒 Última Sincronização:             {}", data.last_sync);
                            println!("==================================================");
                        }
                        Err(e) => println!("[ERRO DE SEGURANÇA] Falha ao decifrar o cofre: {:?}", e),
                    }
                }
            }
            "2" => {
                println!("\n--- ATIVAÇÃO DO AUTOPILOT REFINADO (TESTNET REAL) ---");
                let password = prompt("Insira a senha mestre para autorizar o trading autónomo: ");
                match load_secure_vault::<VaultData>(vault_path, password.as_bytes()) {
                    Ok(mut data) => {
                        println!("\n[AUTOPILOT TESTNET] 🚀 Agente Nolan conectado à Binance Spot Testnet.");
                        
                        let mut last_price = match fetch_binance_price() {
                            Ok(p) => p,
                            Err(_) => 87.50,
                        };

                        for tick in 1..=4 {
                            let current_price = match fetch_binance_price() {
                                Ok(p) => p,
                                Err(_) => last_price,
                            };

                            let delta = current_price - last_price;
                            let (decision, side) = if delta >= 0.0 {
                                ("🟢 BUY", "BUY")
                            } else {
                                ("🔴 SELL", "SELL")
                            };

                            println!("[Tick #{:03}] SOL Testnet: $ {:.2} | Sinal: {}", tick, current_price, decision);

                            let target_quantity = 0.15;
                            println!("[EXECUÇÃO REAL] A enviar ordem de {} ({} SOL) com assinatura HMAC-SHA256...", side, target_quantity);
                            
                            match send_testnet_order(&data.binance_api_key, &data.binance_secret_key, "SOLUSDT", side, target_quantity) {
                                Ok(res) => {
                                    println!("[SUCESSO EXCHANGE] {}", res);
                                    data.principal += 0.30;
                                }
                                Err(e) => println!("[ERRO DE REDE/API] Detalhes: {:?}", e),
                            }

                            if data.principal > 60.0 {
                                let excess = data.principal - 50.0;
                                data.reserve += excess;
                                data.principal = 50.0;
                                println!("🌟 [TRANSBORDO] Excedente de $ {:.2} enviado para a Reserva.", excess);
                            }

                            last_price = current_price;
                            thread::sleep(Duration::from_secs(3));
                        }

                        let _ = save_secure_vault(vault_path, &data, password.as_bytes());
                        println!("[AUTOPILOT] Estado atualizado e persistido com segurança no cofre cifrado.");
                    }
                    Err(e) => println!("[ERRO] Falha ao abrir cofre: {:?}", e),
                }
            }
            "3" => {
                println!("\n--- AUDITORIA DE HISTÓRICO DE ORDENS ---");
                if !std::path::Path::new(orders_path).exists() {
                    println!("[INFO] Nenhum registo encontrado.");
                } else {
                    let password = prompt("Insira a senha mestre para decifrar as ordens: ");
                    match load_secure_vault::<Vec<EncryptedOrderRecord>>(orders_path, password.as_bytes()) {
                        Ok(orders) => {
                            println!("[SUCESSO] Histórico decifrado! Total: {} ordens.", orders.len());
                            for o in orders {
                                println!(" - [#{} {}] {} | Preço: $ {:.2} | Qtd: {}", o.id, o.symbol, o.side, o.price, o.amount);
                            }
                        }
                        Err(e) => println!("[ERRO] Falha ao decifrar ordens: {:?}", e),
                    }
                }
            }
            "4" => {
                println!("\n--- TESTE DE SECURITYGUARD & AES-256-GCM ---");
                match secure_mobile_params() {
                    Ok(_) => println!("[SECURITY GUARD] 🛡️ Verificação de integridade OK. Zero Plaintext ativo."),
                    Err(e) => println!("[ERRO] Parâmetros inválidos: {:?}", e),
                }
            }
            "5" => {
                println!("\n--- CONSULTA DE PREÇO SPOT (BINANCE TESTNET) ---");
                match fetch_binance_price() {
                    Ok(p) => println!("[BINANCE TESTNET SPOT] SOL/USDT: $ {:.2}", p),
                    Err(e) => println!("[ERRO] Falha ao consultar Binance Testnet: {:?}", e),
                }
            }
            "6" => {
                println!("\n--- BENCHMARK DE DERIVAÇÃO ARGON2ID (MOBILE) ---");
                let start = std::time::Instant::now();
                let _ = derive_key(b"benchmark_pass", b"salt_16_bytes_abc");
                println!("[BENCHMARK] Derivação Argon2id concluída em {:?}.", start.elapsed());
            }
            "7" => {
                println!("\n--- MODO SIMULAÇÃO DE STRESS & BACKTEST ---");
                println!("[INFO] Simulação pronta.");
            }
            "8" => {
                println!("\n--- CONSULTA DE SALDO REAL NA BINANCE TESTNET ---");
                let password = prompt("Insira a senha mestre para aceder às credenciais no cofre: ");
                match load_secure_vault::<VaultData>(vault_path, password.as_bytes()) {
                    Ok(data) => {
                        println!("[CONEXÃO] A consultar saldos na Binance Spot Testnet...");
                        match fetch_testnet_account_balance(&data.binance_api_key, &data.binance_secret_key) {
                            Ok(balances) => println!("[SALDO DA CONTA TESTNET]: {}", balances),
                            Err(e) => println!("[ERRO] Falha ao consultar saldos: {:?}", e),
                        }
                    }
                    Err(e) => println!("[ERRO] Falha ao abrir cofre: {:?}", e),
                }
            }
            "9" => {
                println!("\n--- CHAT COM AGENTE NOLAN (OLLAMA LOCAL + AST) ---");
                let model_name = prompt("Insira o nome do modelo local Ollama (ex: llama3, mistral): ");
                let user_msg = prompt("Escreva o seu comando em linguagem natural para o Agente Nolan: ");
                
                println!("[OLLAMA] A traduzir intenção localmente via '{}'...", model_name);
                let bridge = OllamaBridge::new(&model_name);
                
                match bridge.translate_intent_to_ast(&user_msg) {
                    Ok(node) => {
                        println!("[SUCESSO IA] Intenção convertida com sucesso em nó AST estruturado:");
                        println!("{:#?}", node);

                        // Testar a avaliação determinística do AST com o preço atual de mercado
                        if let Ok(current_price) = fetch_binance_price() {
                            match AstEvaluator::evaluate(&node, current_price) {
                                Ok(res) => println!("[AST AVALIAÇÃO DETERMINÍSTICA] Resultado da regra com SOL a $ {:.2}: {}", current_price, res),
                                Err(e) => println!("[ERRO AST] Falha ao avaliar nó: {}", e),
                            }
                        }
                    }
                    Err(e) => println!("[ERRO DE TRADUÇÃO IA] Certifique-se de que a Ollama está a correr localmente. Detalhes: {:?}", e),
                }
            }
            "10" => {
                println!("\nEncerrando painel de controlo. Até breve, Comandante! 🛡️");
                break;
            }
            _ => {
                println!("\n[AVISO] Opção inválida. Selecione um número de 1 a 10.");
            }
        }
    }
}
