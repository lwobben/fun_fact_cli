use std::env;
use reqwest::{Client,header::HeaderMap, Url};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;
use crossterm::{
    event::{self, Event, KeyCode}, 
    terminal::{self},
};
use std::io::{Write, stdout, Stdout};

#[tokio::main]
async fn main() {
    // get openai key
    let openai_key = "OPENAI_KEY";
    let key = env::var(openai_key).unwrap();

    // request details
    let client = Client::new();
    let mut headers: HeaderMap = HeaderMap::new();
    let header_string = format!("Bearer {}", key).parse::<String>().unwrap();
    let header_value = HeaderValue::from_str(&header_string).unwrap();
    headers.insert(AUTHORIZATION, header_value);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let mut request_body = RequestBodyOpenAI {
        model: String::from("gpt-3.5-turbo"),
        messages: vec![Message{role:"system".to_owned(), content:"You are a helpful assistant".to_owned()}],
    };
    let url = Url::parse(&format!("{}", "https://api.openai.com/v1/chat/completions")).unwrap();

    // terminal UI
    let mut stdout: Stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    let mut user_input: String = String::new();
    stdout.flush().unwrap();

    let mut count = 0u32;

    'chat: loop {
        
        if count == 0 {
            let first_input = String::from("Hey OpenAI, can you tell a random, but interesting, fun fact?");
            println!("Terminal: {}", first_input);

            // send OpenAI request and process message (to do: move to function)
            request_body.messages.push(Message { role: "user".to_owned(), content: first_input });           
            let request_body_string: String = serde_json::to_string(&request_body).unwrap();
            let res = client
                                        .post(url.clone())
                                        .headers(headers.clone())
                                        .body(request_body_string)
                                        .send().await.unwrap();
            
            let body = res.text().await.unwrap();
            let v: Value = serde_json::from_str(&body).unwrap();
            let assistant_response = v["choices"][0]["message"]["content"].as_str().unwrap().to_owned();
            request_body.messages.push(Message { role: "assistant".to_owned(), content: assistant_response.clone() });

            let mut message = String::from("\n\rOpenAI assistant: ");
            message.push_str( &assistant_response);

            println!("{}", message.as_str());

            print!("\n\r- Press 'esc' to exit or type a follow-up question: ");
            stdout.flush().unwrap();
            user_input = String::from("");
        }

        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Char(c) => {
                    print!("{}", &c.to_string());
                    stdout.flush().unwrap();
                    user_input.push(c);
                },
                KeyCode::Enter => {  
                    // send OpenAI request and process message (to do: move to function)  
                    request_body.messages.push(Message { role: "user".to_owned(), content: user_input.clone() });           
                    let request_body_string: String = serde_json::to_string(&request_body).unwrap();
                    let res = client
                                                .post(url.clone())
                                                .headers(headers.clone())
                                                .body(request_body_string)
                                                .send().await.unwrap();
                    
                    let body = res.text().await.unwrap();
                    let v: Value = serde_json::from_str(&body).unwrap();
                    let assistant_response = v["choices"][0]["message"]["content"].as_str().unwrap().to_owned();
                    request_body.messages.push(Message { role: "assistant".to_owned(), content: assistant_response.clone() });

                    let mut message = String::from("\n\n\rOpenAI Assistant: ");
                    message.push_str( &assistant_response);

                    println!("{}", message.as_str());

                    print!("\n\r- Press 'esc' to exit or type a follow-up question: ");
                    stdout.flush().unwrap();
                    user_input = String::from("");
                },
                KeyCode::Backspace => {
                    print!("\x08 \x08");
                    stdout.flush().unwrap();
                    user_input.pop();
                },
                KeyCode::Delete => {
                    print!("\rYes ");
                },
                KeyCode::Esc => {
                    break 'chat;
                }, 
                _ => {},   
            }
        }
        count += 1;
    }

    terminal::disable_raw_mode().unwrap();
}


// define custom structs
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct RequestBodyOpenAI {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}
