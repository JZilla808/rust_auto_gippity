use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

// Call Large Language Model (i.e. GPT-4 Turbo)
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    // Extract API Key information
    let api_key: String =
        env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in environment variables");
    let api_org: String =
        env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in environment variables");

    // Confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create header
    let mut headers = HeaderMap::new();
    // Create api key header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );
    // Create OPEN AI Org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create client
    let client: Client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Create chat completion
    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-4-1106-preview".to_string(),
        messages,
        temperature: 0.1,
    };

    // // Troubleshooting
    // let res_raw = client
    //     .post(url)
    //     .json(&chat_completion)
    //     .send()
    //     .await
    //     .unwrap();

    // dbg!(res_raw.text().await.unwrap());

    // Extract API response
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message: Message = Message {
            role: "user".to_string(),
            content: "What is your knowlege cut off?".to_string(),
        };

        let messages: Vec<Message> = vec![message];

        let res: Result<String, Box<dyn std::error::Error + Send>> = call_gpt(messages).await;
        match res {
            Ok(res_str) => {
                dbg!(res_str);
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
}