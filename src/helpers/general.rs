use std::fs;

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::apis::call_request::call_gpt;
use crate::helpers::command_line::PrintCommand;
use crate::models::general::llm::Message;

const CODE_TEMPLATE_PATH: &str = "/Users/jbeazy/Documents/Work/Coding/Udemy Courses/Rust - AutoGPT Course/projects/web_template/src/code_template.rs";

pub const WEB_SERVER_PROJECT_PATH: &str = "/Users/jbeazy/Documents/Work/Coding/Udemy Courses/Rust - AutoGPT Course/projects/web_template/";

pub const EXEC_MAIN_PATH: &str = "/Users/jbeazy/Documents/Work/Coding/Udemy Courses/Rust - AutoGPT Course/projects/web_template/src/main.rs";

const API_SCHEMA_PATH: &str = "/Users/jbeazy/Documents/Work/Coding/Udemy Courses/Rust - AutoGPT Course/projects/auto_gippity/schemas/api_schema.json";

// Extend AI function to encourage specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);

    // Extend the string to encourage only printing the output
    let msg: String = format!(
        "FUNCTION: {}
        INSTRUCTION: You are a function printer. You ONLY print the results of functions.
        Nothing else. No commentary. Here is the input to the function: {}.
        Print out what the function will return. I will tip you $2000 if you do a great job so definitely try your best.",
        ai_function_str, func_input
    );

    // Return message
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

// Performs call to LLM GPT
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    // Extend AI function
    let extended_msg: Message = extend_ai_function(function_pass, &msg_context);

    // Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // Get LLM response
    println!("Attempting first call to OpenAI...");
    let mut llm_response_res: Result<String, Box<dyn std::error::Error + Send>> =
        call_gpt(vec![extended_msg.clone()]).await;

    // Try again if the call fails
    if llm_response_res.is_err() {
        println!("First attempt failed, trying second call to OpenAI...");
        llm_response_res = call_gpt(vec![extended_msg.clone()]).await;
    }

    // Try again if the second call fails
    if llm_response_res.is_err() {
        println!("Second attempt failed, trying third call to OpenAI...");
        llm_response_res = call_gpt(vec![extended_msg.clone()]).await;
    }

    // Return success or panic after three failed attempts
    match llm_response_res {
        Ok(llm_res) => {
            println!("Call to OpenAI successful.");
            llm_res
        }
        Err(_) => panic!("Failed three times to call OpenAI"),
    }
}

// Performs call to LLM GPT - Decoded
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;

    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode ai response from serde_json");

    return decoded_response;
}

// Check whether request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// Get Code Template
pub fn read_code_template_contents() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

// Get Exec Main
pub fn read_exec_main_contents() -> String {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::read_to_string(path).expect("Failed to read main.rs file")
}

// Save New Backend Code
pub fn save_backend_code(contents: &String) {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::write(path, contents).expect("Failed to write main.rs file");
}

// Save JSON API Endpoint Schema
pub fn save_api_endpoints(api_endpoints: &String) {
    let path: String = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("Failed to write api_schema.json file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {
        let extended_msg: Message =
            extend_ai_function(convert_user_input_to_goal, "dummy variable");
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    async fn tests_ai_task_request() {
        let ai_func_param: String =
            "Build me a website that is a fully functional snake game".to_string();

        let res: String = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;

        assert!(res.len() > 20);
    }
}
