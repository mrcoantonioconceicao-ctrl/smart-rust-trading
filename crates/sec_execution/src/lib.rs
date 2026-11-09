// Crate sec_execution - Execução segura de ordens e validação tática

pub struct ExecutionManager;

impl ExecutionManager {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_and_execute(&self, _symbol: &str, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Montante de execução inválido ou zero".to_string());
        }
        Ok(())
    }
}
