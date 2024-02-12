use std::io::Write;

use crate::{gh, headers::{CopilotCompletionHeaders, Headers}, prompts, utils};
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct ContentFilterResult {
    filtered: bool,
    severity: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentFilterOffsets {
    check_offset: i32,
    start_offset: i32,
    end_offset: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Delta {
    content: Option<String>,
    role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    index: i32,
    content_filter_offsets: ContentFilterOffsets,
    content_filter_results: Option<ContentFilterResults>,
    delta: Delta,
    finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentFilterResults {
    hate: ContentFilterResult,
    self_harm: ContentFilterResult,
    sexual: ContentFilterResult,
    violence: ContentFilterResult,
}

#[derive(Serialize, Deserialize, Debug)]
struct GhCopilotResponse {
    choices: Vec<Choice>,
    created: i64,
    id: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct Message {
    content: String,
    role: String,
}

#[derive(Debug)]
pub struct Completion {
    pub content: String,
    pub finish_reason: String
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

    pub async fn ask(&mut self, prompt: &String, log: bool) -> Completion {
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
        let mut finish_reason = String::new();

        'outerloop: while let Some(chunk) = response.next().await {
            let body = chunk.unwrap();
            let body_str = String::from_utf8_lossy(&body);

            buffer.push_str(&body_str);
            // the data may be split into multiple chunks, BUT it's always dilimited by \n\ndata:

            let lines = buffer.split("\n\ndata: ").map(|s| s.to_string()).map(|s| s.replacen("data:", "", 1)).collect::<Vec<String>>();


            let mut processed_buffer = String::new();
            for line in lines {
                utils::append_to_file("resp.txt", &format!("{}\n", line));
                if line.is_empty() {
                    continue;
                }

                let parsed = serde_json::from_str::<GhCopilotResponse>(&line);


                match parsed {
                    Ok(parsed) => {
                        if parsed.choices.len() > 0 {
                            let choice = &parsed.choices[0];
                            if let Some(freason) = &choice.finish_reason {
                                finish_reason = freason.clone().to_string();
                                break 'outerloop;
                            }
                            let delta = &choice.delta;
                            if let Some(content) = &delta.content {
                                print!("{}", content);
                                std::io::stdout().flush().unwrap();
                                message.push_str(content);
                            }
                        }
                    }
                    Err(_) => {
                        utils::append_to_file("debug.txt", &format!("{}\n", line));
                        processed_buffer.push_str(&line);
                    }
                }

                buffer = processed_buffer.clone();
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

        Completion {
            content: message.clone(),
            finish_reason
        }
    }
}
