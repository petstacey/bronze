use crate::ai_functions::aifunc_backend_engineer::{
    print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    print_rest_api_endpoints,
};
use crate::helpers::cli::{confirm_safe_code, PrintCommand};
use crate::helpers::general::ai_task_request;
use crate::helpers::general::{
    check_status_code, read_code_template_content, read_executable_code_content, save_api_schema,
    save_backend_code, WEBSERVER_PATH,
};
use crate::models::base_agent::base_agent::{AgentState, BaseAgent};
use crate::models::base_agent::base_traits::BaseTraits;
use crate::models::team::team_traits::{RouteObject, SolutionSpecification, SpecialFunctions};
use async_trait::async_trait;
use reqwest::Client;
use std::fs;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time;

#[derive(Debug)]
pub struct BackendEngineer {
    attributes: BaseAgent,
    bug_count: u8,
    bugs: Option<String>,
}

impl BackendEngineer {
    pub fn new() -> Self {
        let attributes: BaseAgent = BaseAgent {
            objective: "Develops backend code for the webserver and json database".to_string(),
            position: "Backend Developer".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };
        Self {
            attributes,
            bug_count: 0,
            bugs: None,
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

    async fn call_improve_backend_code(&self, specification: &mut SolutionSpecification) {
        let msg_context: String = format!(
            "CODE_TEMPLATE: {:?} \nPROJECT DESCRIPTION: {:?}",
            specification.backend_code, specification
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improved_webserver_code),
            print_improved_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);
        specification.backend_code = Some(ai_response);
    }

    async fn call_fix_bugs(&self, specification: &mut SolutionSpecification) {
        let msg_context: String = format!(
            "BROKEN CODE: {:?} \nERROR_BUGS: {:?}\n
            THIS FUNCTION ONLY OUTPUTS CODE, NOTHING ELSE",
            specification.backend_code, self.bugs
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_fixed_code),
            print_fixed_code,
        )
        .await;

        save_backend_code(&ai_response);
        specification.backend_code = Some(ai_response);
    }

    async fn call_extract_rest_api_schema(&self) -> String {
        let backend_code = read_executable_code_content();

        let msg_context: String = format!("CODE_INPUT: {}", backend_code);

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
        .await;

        ai_response
    }
}

#[async_trait]
impl SpecialFunctions for BackendEngineer {
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
                    self.attributes.update_state(AgentState::Working);
                    continue;
                }
                AgentState::Working => {
                    if self.bug_count == 0 {
                        self.call_improve_backend_code(specification).await;
                    } else {
                        self.call_fix_bugs(specification).await;
                    }
                    self.attributes.update_state(AgentState::UnitTesting);
                    continue;
                }
                AgentState::UnitTesting => {
                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend code: unit testing",
                    );
                    let safe_code: bool = confirm_safe_code();
                    if !safe_code {
                        panic!("Terminating execution. AI needs some alignment...");
                    }

                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend code: Building deployable binary...",
                    );

                    let build_backend_server: std::process::Output = Command::new("cargo")
                        .arg("build")
                        .current_dir(WEBSERVER_PATH)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                        .expect("Backend code: Failed to build server");

                    if build_backend_server.status.success() {
                        self.bug_count = 0;
                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(),
                            "Backend code: Test server build successfully complete...",
                        );
                    } else {
                        let error_arr: Vec<u8> = build_backend_server.stderr;
                        let error_str: String = String::from_utf8(error_arr).unwrap();

                        self.bug_count += 1;
                        self.bugs = Some(error_str);

                        if self.bug_count > 2 {
                            PrintCommand::Issue.print_agent_message(
                                self.attributes.position.as_str(),
                                "Backend code: too many bugs to continue",
                            );
                            panic!("Error: Bug limit exceeded");
                        }

                        self.attributes.update_state(AgentState::Working);
                        continue;
                    }

                    let api_endpoints_str: String = self.call_extract_rest_api_schema().await;

                    let api_endpoints: Vec<RouteObject> =
                        serde_json::from_str(&api_endpoints_str.as_str())
                            .expect("Failed to decode API endpoints");

                    let check_endpoints: Vec<RouteObject> = api_endpoints
                        .iter()
                        .filter(|&route_object| {
                            route_object.method == "get" && route_object.is_route_dynamic == "false"
                        })
                        .cloned()
                        .collect();

                    specification.api_schema = Some(check_endpoints.clone());

                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend code: Starting web server...",
                    );

                    let mut run_backend_server: std::process::Child = Command::new("cargo")
                        .arg("run")
                        .current_dir(WEBSERVER_PATH)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("Failed to run backend server");

                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend code: Launching tests on server in 5 seconds...",
                    );

                    let sleep: Duration = Duration::from_secs(5);
                    time::sleep(sleep).await;

                    for endpoint in check_endpoints {
                        let testing_msg: String = format!("Testing endpoint '{}'...", endpoint.route);

                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(),
                            testing_msg.as_str(),
                        );

                        let client = Client::builder().timeout(Duration::from_secs(5)).build().unwrap();

                        let url: String = format!("http://127.0.0.1:8080{}", endpoint.route);
                        match check_status_code(&client, &url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    let err_msg: String = format!("WARNING: Failed to call backend url endpoint '{}'. Expected 200, got {}", endpoint.route, status_code);
                                    PrintCommand::Issue.print_agent_message(
                                        self.attributes.position.as_str(),
                                        err_msg.as_str(),
                                    );
                                }
                            }
                            Err(e) => {
                                run_backend_server.kill().expect("Failed to kill backend webserver");

                                let error_msg: String = format!("Error checking backend '{}'", e);
                                PrintCommand::Issue.print_agent_message(
                                    self.attributes.position.as_str(),
                                    error_msg.as_str(),
                                );
                            }
                        }
                    }

                    save_api_schema(&api_endpoints_str);

                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Server unit testing complete",
                    );

                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Webserver shutdown",
                    );

                    run_backend_server.kill().expect("Failed to kill backend webserver on completion");
                    
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
        let mut agent = BackendEngineer::new();

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
            .expect("Failed to execute backend engineer");
        dbg!(specification);
    }
}
