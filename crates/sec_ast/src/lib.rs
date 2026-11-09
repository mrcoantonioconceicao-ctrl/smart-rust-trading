use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AstNode {
    /// Verifica se o preço atual de um ativo atinge um limiar
    PriceThreshold {
        symbol: String,
        operator: String, // ">", "<", "=="
        target_price: f64,
    },
    /// Regra de gestão de risco (ex: Limitar exposição máxima ou lucro)
    RiskCheck {
        max_loss_allowed: f64,
        take_profit_target: f64,
    },
    /// Ação final de execução determinística
    ExecutionAction {
        symbol: String,
        side: String, // "BUY" ou "SELL"
        quantity: f64,
    },
}

/// O motor de avaliação do AST (Avalia nós de forma estrita e determinística)
pub struct AstEvaluator;

impl AstEvaluator {
    pub fn evaluate(node: &AstNode, current_price: f64) -> Result<bool, String> {
        match node {
            AstNode::PriceThreshold { symbol: _, operator, target_price } => {
                match operator.as_str() {
                    ">" => Ok(current_price > *target_price),
                    "<" => Ok(current_price < *target_price),
                    "==" => Ok((current_price - target_price).abs() < f64::EPSILON),
                    _ => Err(format!("Operador lógico desconhecido no AST: {}", operator)),
                }
            }
            AstNode::RiskCheck { max_loss_allowed: _, take_profit_target: _ } => {
                Ok(true) 
            }
            AstNode::ExecutionAction { symbol: _, side: _, quantity: _ } => {
                Ok(true)
            }
        }
    }
}
