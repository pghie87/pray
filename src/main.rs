//! Main entry point for the Risk Assessment and Modeling Service.
//!
//! This service is responsible for executing risk models, generating risk assessments,
//! and providing model explainability for the Credit Risk Intelligence Platform.

use actix_web::{App, HttpServer, middleware, web};
use log::{info, error};
use std::sync::Arc;

mod models;
mod services;
mod api;
mod config;
mod errors;
mod repositories;
mod utils;

use crate::services::risk_assessment::RiskAssessmentServiceImpl;
use crate::services::model_execution::ModelExecutionServiceImpl;
use crate::services::model_management::ModelManagementServiceImpl;
use crate::services::factor_analysis::FactorAnalysisServiceImpl;
use crate::services::data_integration::DataIntegrationServiceImpl;
use crate::repositories::risk_assessment::RiskAssessmentRepositoryImpl;
use crate::repositories::model::ModelRepositoryImpl;
use crate::api::routes;
use crate::config::AppConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("Starting Risk Assessment and Modeling Service");
    
    // Load configuration
    let config = match AppConfig::load_from_env() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()));
        }
    };
    
    // Initialize repositories
    let db_pool = match config.create_db_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to create database connection pool: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()));
        }
    };
    
    let risk_assessment_repo = Arc::new(RiskAssessmentRepositoryImpl::new(db_pool.clone()));
    let model_repo = Arc::new(ModelRepositoryImpl::new(db_pool.clone()));
    
    // Initialize services
    let data_integration_service = Arc::new(DataIntegrationServiceImpl::new(&config));
    let model_management_service = Arc::new(ModelManagementServiceImpl::new(model_repo.clone()));
    let factor_analysis_service = Arc::new(FactorAnalysisServiceImpl::new());
    
    let ml_engine = services::ml::create_ml_engine(&config);
    let metrics_service = Arc::new(services::metrics::MetricsServiceImpl::new(&config));
    
    let model_execution_service = Arc::new(ModelExecutionServiceImpl::new(
        model_repo.clone(),
        ml_engine,
        metrics_service.clone()
    ));
    
    let risk_assessment_service = Arc::new(RiskAssessmentServiceImpl::new(
        model_execution_service.clone(),
        model_management_service.clone(),
        factor_analysis_service.clone(),
        data_integration_service.clone(),
        risk_assessment_repo.clone()
    ));
    
    // Start HTTP server
    info!("Starting HTTP server at {}:{}", config.server.host, config.server.port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(risk_assessment_service.clone()))
            .app_data(web::Data::from(model_management_service.clone()))
            .app_data(web::Data::from(model_execution_service.clone()))
            .app_data(web::Data::from(factor_analysis_service.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(routes::configure)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}