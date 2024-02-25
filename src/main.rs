use std::env;
use reqwest::{Client,header::HeaderMap, Url};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;
use crossterm::{
    event::{self, Event, KeyCode}, 
    terminal::{self},
};
use std::io::{Write, stdout, Stdout};


const MODEL: &'static str = "gpt-3.5-turbo";
const SYSTEM_MESSAGE: &'static str = "You are a helpful and fun assistant";
const FIRST_INPUT: &'static str = "Hey OpenAI, can you tell a random, but interesting, fun fact?";


#[tokio::main]
async fn main() {
    let openai_key = env::var("OPENAI_KEY").expect("\nEnv var 'OPENAI_KEY' is not set, please set it with your OpenAI API openai_key!\n\n");

    let mut request_body = OpenAIRequestBody {
        model: String::from(MODEL),
        messages: vec![Message{role: "system".to_owned(), content: SYSTEM_MESSAGE.to_owned()}],
    };
    let api_url = Url::parse(&format!("{}", "https://api.openai.com/v1/chat/completions")).unwrap();

    let mut headers: HeaderMap = HeaderMap::new();
    let header_string = format!("Bearer {}", openai_key).parse::<String>().unwrap();
    let header = HeaderValue::from_str(&header_string).unwrap();
    headers.insert(AUTHORIZATION, header);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let client = Client::new();

    let mut stdout: Stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    let mut user_input: String = String::new();
    stdout.flush().unwrap();

    let mut count = 0u32;

    'chat: loop {
        
        if count == 0 {
            println!("Terminal: {}", FIRST_INPUT.to_string());

            request_body.messages.push(Message { role: "user".to_owned(), content: FIRST_INPUT.to_string() });           
            let request_body_string: String = serde_json::to_string(&request_body).unwrap();
            let res = client
                                        .post(api_url.clone())
                                        .headers(headers.clone())
                                        .body(request_body_string)
                                        .send().await.unwrap();
            
            let body = res.text().await.unwrap();
            let v: Value = serde_json::from_str(&body).unwrap();
            let assistant_response = v["choices"][0]["message"]["content"].as_str().unwrap().to_owned();
            request_body.messages.push(Message {role: "assistant".to_owned(), content: assistant_response.clone()});

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
                    request_body.messages.push(Message {role: "user".to_owned(), content: user_input.clone()});           
                    let request_body_string: String = serde_json::to_string(&request_body).unwrap();
                    let res = client
                                                .post(api_url.clone())
                                                .headers(headers.clone())
                                                .body(request_body_string)
                                                .send().await.unwrap();
                    
                    let body = res.text().await.unwrap();
                    let v: Value = serde_json::from_str(&body).unwrap();
                    let assistant_response = v["choices"][0]["message"]["content"].as_str().unwrap().to_owned();
                    request_body.messages.push(Message {role: "assistant".to_owned(), content: assistant_response.clone()});

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


use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct OpenAIRequestBody {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}
