use crate::models::general::llm::{ApiResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    let api_key: String =
        env::var("OPEN_AI_KEY").expect("Open AI Key not found in environment variables");
    let api_org: String =
        env::var("OPEN_AI_ORG").expect("Open AI Org not found in environment variables");

    let url: String = 
    env::var("OPEN_AI_URL").expect("Open AI Org not found in environment variables");

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    let chat_completion = ChatCompletion {
        model: "gpt-4".to_string(), // gpt-3.5-turbo, gpt-4
        messages,
        temperature: 0.1,
    };

    let response: ApiResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    Ok(response.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hi there. This is a test. Give me a short response".to_string(),
        };

        let messages = vec![message];

        let response: Result<String, Box<dyn std::error::Error + Send>> = call_gpt(messages).await;
        match response {
            Ok(res_str) => {
                dbg!(res_str);
                assert!(true)
            }
            Err(_) => assert!(false),
        }
    }
}
