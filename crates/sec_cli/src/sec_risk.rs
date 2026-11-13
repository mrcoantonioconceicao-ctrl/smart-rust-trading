pub struct RiskMetrics {
    pub max_drawdown: f64,
    pub current_exposure_btc: f64,
    pub current_exposure_eth: f64,
    pub api_latency_ms: u64,
    pub risk_status: &'static str,
}

impl RiskMetrics {
    pub fn evaluate(counter: usize) -> Self {
        // Simulação baseada no ciclo atual para telemetria em tempo real
        let drawdown = if counter % 7 == 0 { -0.15 } else { -0.02 };
        let latency = 45 + (counter as u64 * 3) % 120;
        let status = if latency > 120 { "⚠️ LATÊNCIA ELEVADA" } else { "🟢 RISCO CONTROLADO" };

        Self {
            max_drawdown: drawdown,
            current_exposure_btc: 55.0, // 55% do capital alocado em BTC
            current_exposure_eth: 45.0, // 45% alocado em ETH
            api_latency_ms: latency,
            risk_status: status,
        }
    }
}
