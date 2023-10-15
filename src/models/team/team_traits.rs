use crate::models::base_agent::base_agent::BaseAgent;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct ProjectFeatures {
    pub is_crud_required: bool,
    pub is_user_login_and_logout_required: bool,
    pub is_external_urls_required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RouteObject {
    pub is_route_dynamic: String,
    pub method: String,
    pub request_body: serde_json::Value,
    pub response: serde_json::Value,
    pub route: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SolutionSpecification {
    pub project_description: String,
    pub project_scope: Option<ProjectFeatures>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub api_schema: Option<Vec<RouteObject>>,
}

#[async_trait]
pub trait SpecialFunctions: Debug {
    fn get_attributes_from_agent(&self) -> &BaseAgent;
    async fn execute_task(
        &mut self,
        specification: &mut SolutionSpecification,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
