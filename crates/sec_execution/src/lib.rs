use sec_domain::{TradeOrder, SecurityGuard};
use tokio::sync::mpsc;

pub struct ExecutionEngine {
    tx: mpsc::Sender<TradeOrder>,
}

impl ExecutionEngine {
    pub fn new(buffer_size: usize) -> (Self, mpsc::Receiver<TradeOrder>) {
        let (tx, rx) = mpsc::channel(buffer_size);
        (Self { tx }, rx)
    }

    pub async fn submit_order(&self, order: TradeOrder) -> Result<(), &'static str> {
        // Validação defensiva de segurança antes de enfileirar a ordem
        SecurityGuard::validate_market_input(&order.symbol, 100.0, "Secure Execution Check")?;
        
        self.tx.send(order).await.map_err(|_| "[SEC-ERROR] Falha ao enviar ordem pelo canal SOA.")?;
        Ok(inspect_order_dispatch())
    }
}

fn inspect_order_dispatch() {
    println!("[SOA-EXECUTION] Ordem validada e encaminhada com segurança para o motor on-chain.");
}
