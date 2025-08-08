//! Factor analysis and model explainability models.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::risk_assessment::Factor;

/// Factor analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactorAnalysis {
    /// Unique identifier for this analysis
    pub analysis_id: String,
    
    /// Associated assessment ID
    pub assessment_id: String,
    
    /// Individual factors and their impacts
    pub factors: Vec<Factor>,
    
    /// Baseline values for comparison
    pub baseline: HashMap<String, serde_json::Value>,
    
    /// Feature impact values
    pub impact_values: HashMap<String, f64>,
    
    /// Analysis of categorical features
    pub categorical_factors: HashMap<String, CategoricalFactorAnalysis>,
    
    /// Analysis of continuous features
    pub continuous_factors: HashMap<String, ContinuousFactorAnalysis>,
}

/// Analysis of a categorical factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoricalFactorAnalysis {
    /// Factor name
    pub name: String,
    
    /// Current value
    pub value: String,
    
    /// Impact of current value
    pub impact: f64,
    
    /// Impact of each possible value
    pub value_impacts: HashMap<String, f64>,
}

/// Analysis of a continuous factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuousFactorAnalysis {
    /// Factor name
    pub name: String,
    
    /// Current value
    pub value: f64,
    
    /// Impact of current value
    pub impact: f64,
    
    /// Sensitivity range (how score changes across value range)
    pub sensitivity: Vec<(f64, f64)>,
}

/// Human-readable explanations of factor analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Explanations {
    /// Overall explanation of the risk assessment
    pub overall_explanation: String,
    
    /// Explanations for individual factors
    pub factor_explanations: Vec<FactorExplanation>,
    
    /// Suggested actions to improve score
    pub suggested_actions: Vec<SuggestedAction>,
}

/// Explanation for an individual factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactorExplanation {
    /// Factor name
    pub factor_name: String,
    
    /// Human-readable explanation
    pub explanation: String,
    
    /// Importance level (0.0-1.0)
    pub importance: f64,
}

/// Suggested action to improve score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedAction {
    /// Action description
    pub description: String,
    
    /// Estimated impact on score
    pub estimated_impact: f64,
    
    /// Difficulty to implement (1-5)
    pub difficulty: u8,
    
    /// Related factors
    pub related_factors: Vec<String>,
}

/// Visualization data for factor analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visualization {
    /// Chart data for factor impacts
    pub factor_impact_chart: ChartData,
    
    /// Chart data for score distribution
    pub score_distribution_chart: ChartData,
    
    /// Chart data for continuous factor sensitivity
    pub sensitivity_charts: HashMap<String, ChartData>,
}

/// Generic chart data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartData {
    /// Chart type
    pub chart_type: String,
    
    /// Chart title
    pub title: String,
    
    /// X-axis label
    pub x_label: String,
    
    /// Y-axis label
    pub y_label: String,
    
    /// Data series
    pub series: Vec<DataSeries>,
}

/// Data series for charts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSeries {
    /// Series name
    pub name: String,
    
    /// Series data points
    pub data: Vec<DataPoint>,
}

/// Data point for charts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    /// X value
    pub x: serde_json::Value,
    
    /// Y value
    pub y: f64,
}

impl FactorAnalysis {
    /// Creates a new factor analysis with a generated UUID
    pub fn new(assessment_id: String) -> Self {
        Self {
            analysis_id: uuid::Uuid::new_v4().to_string(),
            assessment_id,
            factors: Vec::new(),
            baseline: HashMap::new(),
            impact_values: HashMap::new(),
            categorical_factors: HashMap::new(),
            continuous_factors: HashMap::new(),
        }
    }
    
    /// Returns the top N factors by absolute impact
    pub fn top_factors(&self, n: usize) -> Vec<&Factor> {
        let mut factors = self.factors.iter().collect::<Vec<_>>();
        factors.sort_by(|a, b| b.impact.abs().partial_cmp(&a.impact.abs()).unwrap());
        factors.truncate(n);
        factors
    }
    
    /// Calculate overall importance of each feature category
    pub fn category_importance(&self) -> HashMap<String, f64> {
        let mut importance = HashMap::new();
        
        for factor in &self.factors {
            let current = importance.entry(factor.category.clone()).or_insert(0.0);
            *current += factor.impact.abs();
        }
        
        // Normalize
        let total: f64 = importance.values().sum();
        if total > 0.0 {
            for value in importance.values_mut() {
                *value /= total;
            }
        }
        
        importance
    }
}