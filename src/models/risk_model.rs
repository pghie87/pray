//! Risk model definition and related structures.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::{Validate, ValidationError};

/// Current status of a risk model in its lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelStatus {
    /// Model is being developed
    Development,
    
    /// Model is being tested
    Testing,
    
    /// Model is active and in use
    Active,
    
    /// Model is active but being used as challenger
    Challenger,
    
    /// Model is deprecated
    Deprecated,
    
    /// Model is archived
    Archived,
}

/// Data type for model features
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeatureType {
    Numeric,
    Categorical,
    Boolean,
    DateTime,
    Text,
}

/// Definition of a feature used by the risk model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureDefinition {
    /// Feature name
    pub name: String,
    
    /// Feature data type
    pub data_type: FeatureType,
    
    /// Is this feature required?
    pub required: bool,
    
    /// Default value if missing
    pub default_value: Option<serde_json::Value>,
    
    /// Valid range for numeric features
    pub range: Option<(f64, f64)>,
    
    /// Valid values for categorical features
    pub valid_values: Option<Vec<String>>,
    
    /// Human-readable description
    pub description: String,
}

/// Definition of a model output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputDefinition {
    /// Output name
    pub name: String,
    
    /// Output data type
    pub data_type: FeatureType,
    
    /// Valid range for numeric outputs
    pub range: Option<(f64, f64)>,
    
    /// Valid values for categorical outputs
    pub valid_values: Option<Vec<String>>,
    
    /// Human-readable description
    pub description: String,
}

/// Risk model definition
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RiskModel {
    /// Unique identifier for the model
    #[validate(length(min = 1, max = 36))]
    pub model_id: String,
    
    /// Human-readable name
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    /// Model version (semver)
    #[validate(length(min = 1, max = 20))]
    pub version: String,
    
    /// Model type (e.g., "creditcard", "mortgage", "auto")
    #[validate(length(min = 1, max = 50))]
    pub model_type: String,
    
    /// Target customer segment
    #[validate(length(min = 1, max = 50))]
    pub target_segment: String,
    
    /// Model parameters (serialized)
    pub parameters: HashMap<String, serde_json::Value>,
    
    /// Model metadata
    pub metadata: HashMap<String, serde_json::Value>,
    
    /// Creation date
    pub created_date: DateTime<Utc>,
    
    /// Last modification date
    pub modified_date: DateTime<Utc>,
    
    /// Model status
    pub status: ModelStatus,
    
    /// Expected score range
    pub score_range: (f64, f64),
    
    /// Model feature definitions
    pub features: Vec<FeatureDefinition>,
    
    /// Model output definitions
    pub outputs: Vec<OutputDefinition>,
    
    /// Performance metrics from validation
    pub validation_metrics: HashMap<String, f64>,
    
    /// Model owner/creator
    #[validate(length(min = 1, max = 100))]
    pub owner: String,
}

impl RiskModel {
    /// Validates that the model has all required features
    pub fn validate_features(&self) -> Result<(), ValidationError> {
        if self.features.is_empty() {
            return Err(ValidationError::new("Model must have at least one feature"));
        }
        
        // Check for duplicate feature names
        let mut names = std::collections::HashSet::new();
        for feature in &self.features {
            if !names.insert(&feature.name) {
                return Err(ValidationError::new(&format!("Duplicate feature name: {}", feature.name)));
            }
        }
        
        Ok(())
    }
    
    /// Validates that the model has all required outputs
    pub fn validate_outputs(&self) -> Result<(), ValidationError> {
        if self.outputs.is_empty() {
            return Err(ValidationError::new("Model must have at least one output"));
        }
        
        // Check for duplicate output names
        let mut names = std::collections::HashSet::new();
        for output in &self.outputs {
            if !names.insert(&output.name) {
                return Err(ValidationError::new(&format!("Duplicate output name: {}", output.name)));
            }
        }
        
        // Check that the model has a risk score output
        if !self.outputs.iter().any(|o| o.name == "risk_score") {
            return Err(ValidationError::new("Model must have a 'risk_score' output"));
        }
        
        Ok(())
    }
    
    /// Validates the score range
    pub fn validate_score_range(&self) -> Result<(), ValidationError> {
        let (min, max) = self.score_range;
        
        if min >= max {
            return Err(ValidationError::new("Score minimum must be less than maximum"));
        }
        
        Ok(())
    }
}