use std::env;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Deserialize)]
struct MessageResponse {
    content: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().skip(1).collect();
    println!("{:?}", args.get(0));

    let api_key = env::var("api_key_ai").unwrap();
    let client = Client::new();


    let request_body = ChatRequest {
         model: "gpt-4.1-nano", // or "gpt-3.5-turbo"
        messages: vec![Message {
            role: "developer",
            content: "Your goal is to improve developer productivity. I want you to help the user with terminal commands. \
            When the user asks how to do some thing give them a terminal command on how to do it and then give them a brief explaination on what the command does.\
            If a question is not about software development then tell the user you are only designed to know about software.\
            "
        },
        Message {
            role: "user",
            content: args.get(0).unwrap(),
        }],
    };

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await?;

    let chat_response: ChatResponse = res.json().await?;

    for choice in chat_response.choices {
        println!("ChatGPT says: {}", choice.message.content);
    }

    Ok(())

}
