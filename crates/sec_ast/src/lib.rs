use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ComparisonOp {
    GreaterThan,
    LessThan,
    Equals,
    GreaterOrEqual,
    LessOrEqual,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TradeAction {
    Buy,
    Sell,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum AstNode {
    /// Condição de limiar de preço (ex: Preço SOL > 90.0)
    PriceThreshold {
        asset: String,
        op: ComparisonOp,
        value: f64,
    },
    /// Ação de execução direta (ex: Comprar 0.2 SOL)
    ExecutionAction {
        action: TradeAction,
        amount: f64,
        asset: String,
    },
    /// Regras de gestão de risco e proteção (Stop Loss / Take Profit)
    RiskCheck {
        stop_loss_pct: f64,
        take_profit_pct: f64,
    },
    /// Condição lógica composta (ex: Condição A AND Condição B)
    Composite {
        op: LogicalOp,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
}

pub struct AstEvaluator;

impl AstEvaluator {
    /// Avalia o nó AST de forma estritamente determinística face ao contexto de mercado atual
    pub fn evaluate(node: &AstNode, current_price: f64) -> Result<bool, String> {
        match node {
            AstNode::PriceThreshold { asset, op, value } => {
                let matched = match op {
                    ComparisonOp::GreaterThan => current_price > *value,
                    ComparisonOp::LessThan => current_price < *value,
                    ComparisonOp::Equals => (current_price - *value).abs() < f64::EPSILON,
                    ComparisonOp::GreaterOrEqual => current_price >= *value,
                    ComparisonOp::LessOrEqual => current_price <= *value,
                };
                println!("[AST EVAL] Asset: {}, Current Price: {}, Threshold: {}, Result: {}", asset, current_price, value, matched);
                Ok(matched)
            }
            AstNode::ExecutionAction { action, amount, asset } => {
                println!("[AST EVAL] Ação validada com sucesso: {:?} de {} {}", action, amount, asset);
                Ok(true)
            }
            AstNode::RiskCheck { stop_loss_pct, take_profit_pct } => {
                println!("[AST EVAL] Regras de Risco carregadas -> Stop Loss: {}%, Take Profit: {}%", stop_loss_pct, take_profit_pct);
                Ok(true)
            }
            AstNode::Composite { op, left, right } => {
                let left_res = Self::evaluate(left, current_price)?;
                let right_res = Self::evaluate(right, current_price)?;
                let final_res = match op {
                    LogicalOp::And => left_res && right_res,
                    LogicalOp::Or => left_res || right_res,
                };
                Ok(final_res)
            }
        }
    }
}
