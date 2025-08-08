//! Applicant data structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Applicant data used for risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicantData {
    /// Unique applicant identifier
    pub applicant_id: String,
    
    /// Basic personal information
    pub personal_info: PersonalInfo,
    
    /// Financial information
    pub financial_info: FinancialInfo,
    
    /// Credit history
    pub credit_info: CreditInfo,
    
    /// Employment information
    pub employment_info: EmploymentInfo,
    
    /// Additional attributes that don't fit in the standard categories
    pub additional_attributes: HashMap<String, serde_json::Value>,
}

/// Basic personal information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalInfo {
    /// Full name
    pub name: String,
    
    /// Date of birth
    pub date_of_birth: String,
    
    /// Residential address
    pub address: Address,
    
    /// Contact information
    pub contact: ContactInfo,
    
    /// Years at current address
    pub years_at_address: f64,
    
    /// Number of dependents
    pub dependents: u8,
}

/// Address structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    /// Street address
    pub street: String,
    
    /// City
    pub city: String,
    
    /// State/province
    pub state: String,
    
    /// Postal code
    pub postal_code: String,
    
    /// Country
    pub country: String,
}

/// Contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    /// Email address
    pub email: String,
    
    /// Phone number
    pub phone: String,
}

/// Financial information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialInfo {
    /// Annual income
    pub annual_income: f64,
    
    /// Monthly housing payment
    pub monthly_housing_payment: f64,
    
    /// Total monthly debt payments
    pub monthly_debt_payments: f64,
    
    /// Total assets
    pub total_assets: f64,
    
    /// Liquid assets
    pub liquid_assets: f64,
    
    /// Monthly free cash flow
    pub monthly_free_cash_flow: f64,
    
    /// Debt to income ratio
    pub debt_to_income_ratio: f64,
}

/// Credit history information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditInfo {
    /// Credit score
    pub credit_score: u16,
    
    /// Number of open accounts
    pub open_accounts: u8,
    
    /// Number of delinquent accounts
    pub delinquent_accounts: u8,
    
    /// Number of inquiries in the last 6 months
    pub inquiries_last_6_months: u8,
    
    /// Oldest account age in months
    pub oldest_account_age_months: u16,
    
    /// Total credit limit
    pub total_credit_limit: f64,
    
    /// Total current balance
    pub total_current_balance: f64,
    
    /// Credit utilization percentage
    pub credit_utilization: f64,
    
    /// Public records
    pub public_records: u8,
    
    /// Collections
    pub collections: u8,
}

/// Employment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmploymentInfo {
    /// Employment status
    pub status: EmploymentStatus,
    
    /// Employer name
    pub employer: String,
    
    /// Job title
    pub title: String,
    
    /// Years at current employer
    pub years_at_employer: f64,
    
    /// Years in profession
    pub years_in_profession: f64,
    
    /// Industry
    pub industry: String,
}

/// Employment status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmploymentStatus {
    Employed,
    SelfEmployed,
    Retired,
    Unemployed,
    Student,
    Other,
}

impl ApplicantData {
    /// Convert to model input format
    pub fn to_input_format(&self) -> HashMap<String, serde_json::Value> {
        let mut input = HashMap::new();
        
        // Flatten the structure for model input
        // Personal info
        input.insert("name".to_string(), serde_json::to_value(&self.personal_info.name).unwrap());
        input.insert("age".to_string(), self.calculate_age());
        input.insert("address_years".to_string(), serde_json::to_value(&self.personal_info.years_at_address).unwrap());
        input.insert("dependents".to_string(), serde_json::to_value(&self.personal_info.dependents).unwrap());
        input.insert("postal_code".to_string(), serde_json::to_value(&self.personal_info.address.postal_code).unwrap());
        
        // Financial info
        input.insert("annual_income".to_string(), serde_json::to_value(&self.financial_info.annual_income).unwrap());
        input.insert("monthly_housing".to_string(), serde_json::to_value(&self.financial_info.monthly_housing_payment).unwrap());
        input.insert("monthly_debt".to_string(), serde_json::to_value(&self.financial_info.monthly_debt_payments).unwrap());
        input.insert("total_assets".to_string(), serde_json::to_value(&self.financial_info.total_assets).unwrap());
        input.insert("liquid_assets".to_string(), serde_json::to_value(&self.financial_info.liquid_assets).unwrap());
        input.insert("free_cash_flow".to_string(), serde_json::to_value(&self.financial_info.monthly_free_cash_flow).unwrap());
        input.insert("dti".to_string(), serde_json::to_value(&self.financial_info.debt_to_income_ratio).unwrap());
        
        // Credit info
        input.insert("credit_score".to_string(), serde_json::to_value(&self.credit_info.credit_score).unwrap());
        input.insert("open_accounts".to_string(), serde_json::to_value(&self.credit_info.open_accounts).unwrap());
        input.insert("delinquent_accounts".to_string(), serde_json::to_value(&self.credit_info.delinquent_accounts).unwrap());
        input.insert("inquiries".to_string(), serde_json::to_value(&self.credit_info.inquiries_last_6_months).unwrap());
        input.insert("account_age".to_string(), serde_json::to_value(&self.credit_info.oldest_account_age_months).unwrap());
        input.insert("credit_limit".to_string(), serde_json::to_value(&self.credit_info.total_credit_limit).unwrap());
        input.insert("credit_balance".to_string(), serde_json::to_value(&self.credit_info.total_current_balance).unwrap());
        input.insert("utilization".to_string(), serde_json::to_value(&self.credit_info.credit_utilization).unwrap());
        input.insert("public_records".to_string(), serde_json::to_value(&self.credit_info.public_records).unwrap());
        input.insert