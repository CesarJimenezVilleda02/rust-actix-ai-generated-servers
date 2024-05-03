use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::{header, Client};
use std::env;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

// Call LLM API
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    // Extract api key info
    let api_key: String = env::var("OPEN_AI_KEY").expect("Open Ai Key not found in env variables");
    let api_org: String = env::var("OPEN_AI_ORG").expect("Open Ai ORG not found in env variables");

    // Confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create header
    let mut headers = HeaderMap::new();

    // Create api header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create org header
    headers.insert(
        "OpenAi-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create client
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Create request
    let chat_completion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages: messages,
        temperature: 0.1,
    };

    // Troubleshooting
    // let res_raw = client
    //     .post(url)
    //     .json(&chat_completion)
    //     .send()
    //     .await
    //     .unwrap();
    // dbg!(res_raw.text().await.unwrap());

    // Extract api response
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Send response
    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_gpt() {
        let messages = vec![Message {
            role: "user".to_string(),
            content: "Hi there this is a test, give me a short response.".to_string(),
        }];

        let response = call_gpt(messages).await;
        if let Ok(response) = response {
            dbg!(response);
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
