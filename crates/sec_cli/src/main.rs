use sec_binance::scan_markets;

#[tokio::main]
async fn main() {
    println!("==================================================");
    println!(" 🛡️ SMART RUST TRADING — PAINEL DE CONTROLO CLI");
    println!("==================================================");
    println!("1. Inspecionar Estado do Cofre & Saldo ($50.00 USD)");
    println!("2. Executar Ciclo de Análise do Agente Nolan");
    println!("3. Auditar Histórico de Ordens Cifradas (AEAD)");
    println!("4. Testar Validação de Segurança (SecurityGuard)");
    println!("5. Consultar Preço Real na Binance (API Testnet)");
    println!("6. Efetuar Reset Contábil do Cofre ($50.00 USD)");
    println!("7. Sair do Painel de Controlo");
    println!("==================================================");
    
    // Teste automático do Agente Scout Multi-Ativo (Opção 2)
    println!("\n🤖 [AGENTE NOLAN] A iniciar ciclo de varredura multi-ativo...");
    match scan_markets().await {
        Ok(opportunities) => {
            println!("\n[NOLAN-SUCCESS] Oportunidades validadas na cesta de ativos:");
            for opp in opportunities {
                println!(" -> Par: {:<8} | Preço: ${:<9.2} | Status: {}", opp.pair, opp.price, opp.status);
            }
        }
        Err(e) => {
            eprintln!("[NOLAN-ERROR] Falha no ciclo de varredura: {}", e);
        }
    }
}
