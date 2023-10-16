use crate::ai_functions::aifunc_backend_engineer::print_backend_webserver_code;
use crate::helpers::general::ai_task_request;
use crate::helpers::general::{ read_code_template_content, save_backend_code };
use crate::models::base_agent::base_agent::{AgentState, BaseAgent};
use crate::models::base_agent::base_traits::BaseTraits;
use crate::models::team::team_traits::{SolutionSpecification, SpecialFunctions};
use async_trait::async_trait;

#[derive(Debug)]
pub struct JuniorBackendEngineer {
    attributes: BaseAgent,
}

impl JuniorBackendEngineer {
    pub fn new() -> Self {
        let attributes: BaseAgent = BaseAgent {
            objective: "Develops initial backend code for the webserver".to_string(),
            position: "Junior Backend Developer".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };
        Self {
            attributes,
        }
    }

    async fn call_initial_backend_code(&self, specification: &mut SolutionSpecification) {
        let code_template_str: String = read_code_template_content();

        let msg_context: String = format!(
            "CODE_TEMPLATE: {} \nPROJECT DESCRIPTION: {}",
            code_template_str, specification.project_description
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);
        specification.backend_code = Some(ai_response);
    }
}

#[async_trait]
impl SpecialFunctions for JuniorBackendEngineer {
    fn get_attributes_from_agent(&self) -> &BaseAgent {
        &self.attributes
    }

    async fn execute_task(
        &mut self,
        specification: &mut SolutionSpecification,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Finished {
            match &self.attributes.state {
                AgentState::Discovery => {
                    self.call_initial_backend_code(specification).await;
                    self.attributes.update_state(AgentState::Finished);
                }
                _ => {}
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_writing_backend_code() {
        let mut agent = JuniorBackendEngineer::new();

        let specification_str: &str = r#"
            {
                "project_description": "build a website that fetches and tracks fitness progress with timezone information.",
                "project_scope": {
                    "is_crud_required": true,
                    "is_user_login_and_logout_required": true,
                    "is_external_urls_required": true
                },
                "external_urls": [
                    "http://worldtimeapi.org/api/timezone"
                ],
                "backend_code": null,
                "api_schema": null
            }"#;

        let mut specification: SolutionSpecification =
            serde_json::from_str(specification_str).unwrap();

        agent.attributes.update_state(AgentState::Discovery);

        agent
            .execute_task(&mut specification)
            .await
            .expect("Failed to execute junior backend engineer");
        dbg!(specification);
    }
}
