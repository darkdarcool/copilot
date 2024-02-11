use std::io::Write;

use crate::{gh, headers::{CopilotCompletionHeaders, Headers}, prompts};
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct ContentFilterOffsets {
    check_offset: u64,
    start_offset: u64,
    end_offset: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentFilterResults {
    hate: FilterResult,
    self_harm: FilterResult,
    sexual: FilterResult,
    violence: FilterResult,
}

#[derive(Serialize, Deserialize, Debug)]
struct FilterResult {
    filtered: bool,
    severity: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Delta {
    content: Option<String>,
    role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    index: u64,
    content_filter_offsets: ContentFilterOffsets,
    content_filter_results: ContentFilterResults,
    delta: Delta,
    #[serde(rename = "finish_reason")]
    finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GhCopilotResponse {
    choices: Vec<Choice>,
    created: u64,
    id: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct Message {
    content: String,
    role: String,
}

pub struct CopilotManager<'a> {
    vscode_sid: String,
    device_id: String,
    auth: &'a gh::GithubAuth,
    client: &'a Client,
    history: Vec<Message>,
}

impl<'a> CopilotManager<'a> {
    pub fn new(auth: &'a gh::GithubAuth, client: &'a Client) -> CopilotManager<'a> {
        let vscode_sid = crate::utils::generate_vscode_session_id();
        let device_id = crate::utils::random_hex_string(6);

        CopilotManager {
            vscode_sid,
            device_id,
            auth,
            client,
            history: Vec::new(),
        }
    }

    fn construct_message_history(
        &self,
        system_prompt: &str,
        current_history: &Vec<Message>,
    ) -> Vec<Message> {
        let system_message = Message {
            content: system_prompt.to_string(),
            role: "system".to_string(),
        };

        // return system message and the current history
        vec![system_message]
            .into_iter()
            .chain(current_history.iter().cloned())
            .collect()
    }

    pub async fn ask(&mut self, prompt: &String, log: bool) -> String {
        let url = "https://api.githubcopilot.com/chat/completions";
        let headers = CopilotCompletionHeaders {
            token: &self.auth.copilot_auth.token,
            vscode_sid: &self.vscode_sid,
            device_id: &self.device_id,
        }.to_headers();

        let mut history =
            self.construct_message_history(prompts::COPILOT_INSTRUCTIONS, &self.history);

        // add current user prompt to history
        history.push(Message {
            content: prompt.to_string(),
            role: "user".to_string(),
        });

        // no chat history for this
        let data = json!({
            "intent": true,
            "model": "gpt-4",
            "n": 1,
            "stream": true,
            "temperature": 0.1,
            "top_p": 1,
            "messages": history
        });

        // we need to stream the response
        let mut response = self
            .client
            .post(url)
            .headers(headers)
            .json(&data)
            .send()
            .await
            .unwrap()
            .bytes_stream();

        let mut message = String::new();
        let mut buffer = String::new();

        'outerloop: while let Some(chunk) = response.next().await {
            let body = chunk.unwrap();
            let body_str = String::from_utf8_lossy(&body)
                .into_owned()
                .replace("\n", "");

            let lines: Vec<String> = body_str
                .split("data:")
                .map(|s| s.trim())
                .map(|s| s.to_string())
                .collect();

            for line in lines {
                if line == "" {
                    continue;
                }

                buffer.push_str(&line.trim());

                let json = serde_json::from_str::<GhCopilotResponse>(&buffer);

                match json {
                    Ok(json) => {
                        if json.choices.len() > 0 {
                            if let Some(content) = &json.choices[0].delta.content {
                                if log {
                                    print!("{}", content);
                                    std::io::stdout().flush().unwrap();
                                }
                                message.push_str(content);
                            } else if let Some(_finish) = &json.choices[0].finish_reason {
                                // println!("Finish reason: {}", finish);
                            } else {
                                // utils::append_to_file("debugr.txt", &format!("{:#?}\n", json));
                            }
                        }

                        buffer.clear();
                    }
                    Err(_e) => {
                        if line == "[DONE]" {
                            break 'outerloop;
                        }
                        // utils::append_to_file("debug.txt", &format!("{}\n", e));
                        continue;
                    }
                }
            }
        }

        if log {
            print!("\n");
            std::io::stdout().flush().unwrap();
        }

        // add the response to the history
        history.push(Message {
            content: message.clone(),
            role: "system".to_string(),
        });

        self.history = history;

        message
    }
}
