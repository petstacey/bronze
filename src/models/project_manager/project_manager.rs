use crate::ai_functions::aifunc_project_manager::print_user_input_as_scope;
use crate::helpers::general::ai_task_request;
use crate::models::base_agent::base_agent::{AgentState, BaseAgent};
use crate::models::general::llm::Message;
use crate::models::team::solution_architect::SolutionArchitect;
use crate::models::team::backend_engineer::BackendEngineer;
use crate::models::team::team_traits::{SolutionSpecification, SpecialFunctions};

#[derive(Debug)]
pub struct ProjectManager {
    attributes: BaseAgent,
    specification: SolutionSpecification,
    team: Vec<Box<dyn SpecialFunctions>>,
}

impl ProjectManager {
    pub async fn new(usr_req: String) -> Result<Self, Box<dyn std::error::Error>> {
        let position = "Project Manager".to_string();

        let attributes = BaseAgent {
            objective:
                "Manage team members that are designing and building the website for the user"
                    .to_string(),
            position: position.clone(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        let project_description: String = ai_task_request(
            usr_req,
            &position,
            get_function_string!(print_user_input_as_scope),
            print_user_input_as_scope,
        )
        .await;

        let team: Vec<Box<dyn SpecialFunctions>> = vec![];

        let specification: SolutionSpecification = SolutionSpecification {
            project_description,
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_schema: None,
        };

        Ok(Self {
            attributes,
            specification,
            team,
        })
    }

    fn add_team_member(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.team.push(agent);
    }

    fn create_team_members(&mut self) {
        self.add_team_member(Box::new(SolutionArchitect::new()));
        self.add_team_member(Box::new(BackendEngineer::new()));
        // Add other team members
    }

    pub async fn deliver_project(&mut self) {
        self.create_team_members();
        for member in &mut self.team {
            let result: Result<(), Box<dyn std::error::Error>> =
                member.execute_task(&mut self.specification).await;

            // let info = member.get_attributes_from_agent();
            // dbg!(info);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_project_manager() {
        let usr_req: String = String::from("need a full stack app that fetches and tracks my fitness progress. Needs to include timezone info from the web");
        let mut pm: ProjectManager = ProjectManager::new(usr_req)
            .await
            .expect("Error creating project manager");

        pm.deliver_project().await;

        dbg!(pm.specification);
    }
}
