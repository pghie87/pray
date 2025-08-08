//! Model output structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Risk tier enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskTier {
    /// Very Low Risk
    VeryLow,
    
    /// Low Risk
    Low,
    
    /// Moderate Risk
    Moderate,
    
    /// High Risk
    High,
    
    /// Very High Risk
    VeryHigh,
}

impl RiskTier {
    /// Converts a risk score to a tier
    pub fn from_score(score: f64, max_score: f64) -> Self {
        let normalized = score / max_score;
        
        match normalized {
            n if n < 0.2 => RiskTier::VeryLow,
            n if n < 0.4 => RiskTier::Low,
            n if n < 0.6 => RiskTier::Moderate,
            n if n < 0.8 => RiskTier::High,
            _ => RiskTier::VeryHigh,
        }
    }
    
    /// Returns the string representation of the tier
    pub fn as_str(&self) -> &'static str {
        match self {
            RiskTier::VeryLow => "Very Low",
            RiskTier::Low => "Low",
            RiskTier::Moderate => "Moderate",
            RiskTier::High => "High",
            RiskTier::VeryHigh => "Very High",
        }
    }
}

/// Model execution output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelOutput {
    /// Risk score
    pub score: f64,
    
    /// Risk tier
    pub tier: String,
    
    /// Confidence level (0.0-1.0)
    pub confidence: f64,
    
    /// Raw model outputs
    pub raw_outputs: HashMap<String, serde_json::Value>,
    
    /// Execution time in milliseconds
    pub execution_time: f64,
    
    /// Warning messages
    pub warnings: Vec<String>,
}

impl ModelOutput {
    /// Creates a new model output
    pub fn new(
        score: f64,
        confidence: f64,
        raw_outputs: HashMap<String, serde_json::Value>,
    ) -> Self {
        let tier = RiskTier::from_score(score, 1000.0).as_str().to_string();
        
        Self {
            score,
            tier,
            confidence,
            raw_outputs,
            execution_time: 0.0,
            warnings: Vec::new(),
        }
    }
    
    /// Adds a warning message
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
    
    /// Sets the execution time
    pub fn set_execution_time(&mut self, time_ms: f64) {
        self.execution_time = time_ms;
    }
    
    /// Returns true if the model output has warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}