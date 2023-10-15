use crate::ai_functions::aifunc_architect::{print_requirements, print_site_urls};
use crate::helpers::cli::PrintCommand;
use crate::helpers::general::{ai_task_request_decoded, check_status_code};
use crate::models::base_agent::base_agent::{AgentState, BaseAgent};
use crate::models::base_agent::base_traits::BaseTraits;
use crate::models::team::team_traits::{ProjectFeatures, SolutionSpecification, SpecialFunctions};

use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

#[derive(Debug)]
pub struct SolutionArchitect {
    attributes: BaseAgent,
}

impl SolutionArchitect {
    pub fn new() -> Self {
        let attributes = BaseAgent {
            objective: "Gathers requirements and design the solution for the website architecture"
                .to_string(),
            position: "Solution Architect".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };
        Self { attributes }
    }
    async fn call_project_features(
        &mut self,
        specification: &mut SolutionSpecification,
    ) -> ProjectFeatures {
        let msg_context: String = format!("{}", specification.project_description);

        let ai_response: ProjectFeatures = ai_task_request_decoded::<ProjectFeatures>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_requirements),
            print_requirements,
        )
        .await;

        specification.project_scope = Some(ai_response.clone());
        self.attributes.update_state(AgentState::Finished);
        ai_response
    }
    async fn call_external_urls(
        &mut self,
        specification: &mut SolutionSpecification,
        msg_context: String,
    ) {
        let ai_response: Vec<String> = ai_task_request_decoded::<Vec<String>>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_site_urls),
            print_site_urls,
        )
        .await;

        specification.external_urls = Some(ai_response);
        self.attributes.update_state(AgentState::UnitTesting);
    }
}

#[async_trait]
impl SpecialFunctions for SolutionArchitect {
    fn get_attributes_from_agent(&self) -> &BaseAgent {
        &self.attributes
    }
    async fn execute_task(
        &mut self,
        specification: &mut SolutionSpecification,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // WARN: CAREFUL OF INFINITE LOOP => INFINITE COST
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    let project_features = self.call_project_features(specification).await;
                    if project_features.is_external_urls_required {
                        self.call_external_urls(
                            specification,
                            specification.project_description.clone(),
                        )
                        .await;
                        self.attributes.update_state(AgentState::UnitTesting);
                    }
                }
                AgentState::UnitTesting => {
                    let mut exclude_urls: Vec<String> = vec![];

                    let client: Client = Client::builder()
                        .timeout(Duration::from_secs(5))
                        .build()
                        .unwrap();

                    let urls: &Vec<String> = specification
                        .external_urls
                        .as_ref()
                        .expect("No URL object in specification");

                    for url in urls {
                        let endpoint_str: String = format!("Testing URL endpoint: {}", url);
                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(),
                            endpoint_str.as_str(),
                        );

                        match check_status_code(&client, url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    exclude_urls.push(url.clone())
                                }
                            }
                            Err(e) => println!("Error checking {}: {}", url, e),
                        }
                    }

                    if exclude_urls.len() > 0 {
                        let new_urls: Vec<String> = specification
                            .external_urls
                            .as_ref()
                            .unwrap()
                            .iter()
                            .filter(|url| !exclude_urls.contains(&url))
                            .cloned()
                            .collect();
                        specification.external_urls = Some(new_urls);
                    }

                    self.attributes.update_state(AgentState::Finished);
                }
                _ => {
                    self.attributes.state = AgentState::Finished;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_solution_architect() {
        let mut agent = SolutionArchitect::new();
        let mut specification = SolutionSpecification {
            project_description: "Build a fullstack website with user login and logout that shows the latest Forex prices".to_string(),
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_schema: None,
        };
        agent
            .execute_task(&mut specification)
            .await
            .expect("Unable to execute Solution Architect");
        assert!(specification.project_scope != None);
        assert!(specification.external_urls.is_some());

        dbg!(specification);
    }
}
