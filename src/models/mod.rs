//! Domain models used throughout the Risk Assessment and Modeling Service.
//!
//! These structures represent the core entities in the system and are used
//! for data transfer between layers.

mod risk_model;
mod risk_assessment;
mod factor_analysis;
mod model_output;
mod applicant_data;
mod request_context;

pub use risk_model::{RiskModel, ModelStatus, FeatureDefinition, FeatureType, OutputDefinition};
pub use risk_assessment::{RiskAssessment, Factor, ImpactDirection};
pub use factor_analysis::{FactorAnalysis, Explanations, Visualization};
pub use model_output::{ModelOutput, RiskTier};
pub use applicant_data::ApplicantData;
pub use request_context::RequestContext;