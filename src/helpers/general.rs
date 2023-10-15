use crate::apis::call_request::call_gpt;
use crate::helpers::cli::PrintCommand;
use crate::models::general::llm::Message;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fs;

const CODE_TEMPLATE_PATH: &str = "/home/peter/workspace/web-template/src/code_template.rs";
pub const EXEC_MAIN_PATH: &str = "/home/peter/workspace/web-template/src/main.rs";
pub const WEBSERVER_PATH: &str = "/home/peter/workspace/web-template/";
const API_SCHEMA_PATH: &str = "/home/peter/workspace/bronze/schemas/api_schema.json";

pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_func_str = ai_func(func_input);

    let message: String = format!(
        "FUNCTION: {}
        INSTRUCTION: You are a function printer. You ONLY print the results of functions.
        Nothing else. No commentary. Here is the input to the function: {}.
        Print out what the function will return.",
        ai_func_str, func_input
    );

    Message {
        role: "system".to_string(),
        content: message,
    }
}

pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    ai_func: for<'a> fn(&'a str) -> &'static str,
) -> String {
    let extended_msg: Message = extend_ai_function(ai_func, &msg_context);
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);
    let llm_res: Result<String, Box<dyn std::error::Error + Send>> =
        call_gpt(vec![extended_msg.clone()]).await;

    match llm_res {
        Ok(llm_response) => llm_response,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed twice to call OpenAPI"),
    }
}

pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    ai_func: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String =
        ai_task_request(msg_context, agent_position, agent_operation, ai_func).await;
    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode AI response from serde_json");
    decoded_response
}

pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

pub fn read_code_template_content() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

pub fn read_executable_code_content() -> String {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

pub fn save_backend_code(content: &String) {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::write(path, content).expect("Failed to write main.rs file");
}

pub fn save_api_schema(schema: &String) {
    let path: String = String::from(API_SCHEMA_PATH);
    fs::write(path, schema).expect("Failed to write api schema to file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_project_manager::print_user_input_as_scope;

    #[test]
    fn tests_extending_ai_function() {
        let extended_msg = extend_ai_function(print_user_input_as_scope, "dummy variable");
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    async fn tests_ai_task_request() {
        let ai_func_param: String =
            "Build me a webserver for making stock price api requests".to_string();

        let result = ai_task_request(
            ai_func_param,
            "Project Manager",
            "Defining user requirements",
            print_user_input_as_scope,
        )
        .await;

        assert!(result.len() > 20);
    }
}
