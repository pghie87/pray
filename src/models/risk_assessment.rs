//! Risk assessment result and related structures.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

/// Direction of factor impact on risk score
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactDirection {
    /// Factor increases risk
    Negative,
    
    /// Factor decreases risk
    Positive,
    
    /// Factor has neutral impact
    Neutral,
}

/// Factor influencing risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Factor {
    /// Factor name/identifier
    pub name: String,
    
    /// Factor value for this applicant
    pub value: serde_json::Value,
    
    /// Impact on risk score (-1.0 to 1.0, where negative is beneficial)
    pub impact: f64,
    
    /// Direction of impact (positive or negative)
    pub direction: ImpactDirection,
    
    /// Factor category
    pub category: String,
    
    /// Human-readable description
    pub description: String,
}

/// Risk assessment result
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RiskAssessment {
    /// Unique identifier for this assessment
    #[validate(length(min = 1, max = 36))]
    pub assessment_id: String,
    
    /// Applicant identifier
    #[validate(length(min = 1, max = 36))]
    pub applicant_id: String,
    
    /// Model used for assessment
    #[validate(length(min = 1, max = 36))]
    pub model_id: String,
    
    /// Risk score (numeric)
    #[validate(range(min = 0.0, max = 1000.0))]
    pub risk_score: f64,
    
    /// Risk tier (categorical)
    #[validate(length(min = 1, max = 20))]
    pub risk_tier: String,
    
    /// Confidence level in the assessment (0.0-1.0)
    #[validate(range(min = 0.0, max = 1.0))]
    pub confidence: f64,
    
    /// Key factors influencing the assessment
    pub key_factors: Vec<Factor>,
    
    /// When the assessment was performed
    pub assessment_date: DateTime<Utc>,
    
    /// When the assessment expires
    pub expires_date: DateTime<Utc>,
    
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl RiskAssessment {
    /// Creates a new risk assessment with a generated UUID
    pub fn new(
        applicant_id: String,
        model_id: String,
        risk_score: f64,
        risk_tier: String,
        confidence: f64,
        key_factors: Vec<Factor>,
        expires_days: i64,
    ) -> Self {
        let now = Utc::now();
        Self {
            assessment_id: uuid::Uuid::new_v4().to_string(),
            applicant_id,
            model_id,
            risk_score,
            risk_tier,
            confidence,
            key_factors,
            assessment_date: now,
            expires_date: now + chrono::Duration::days(expires_days),
            metadata: HashMap::new(),
        }
    }
    
    /// Returns true if the assessment has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_date
    }
    
    /// Returns the top N factors by absolute impact
    pub fn top_factors(&self, n: usize) -> Vec<&Factor> {
        let mut factors = self.key_factors.iter().collect::<Vec<_>>();
        factors.sort_by(|a, b| b.impact.abs().partial_cmp(&a.impact.abs()).unwrap());
        factors.truncate(n);
        factors
    }
    
    /// Returns positive factors (those reducing risk)
    pub fn positive_factors(&self) -> Vec<&Factor> {
        self.key_factors.iter()
            .filter(|f| f.direction == ImpactDirection::Positive)
            .collect()
    }
    
    /// Returns negative factors (those increasing risk)
    pub fn negative_factors(&self) -> Vec<&Factor> {
        self.key_factors.iter()
            .filter(|f| f.direction == ImpactDirection::Negative)
            .collect()
    }
}