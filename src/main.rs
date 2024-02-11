mod copilot;
mod gh;
mod headers;
mod prompts;
mod utils;
mod urls;

use rustyline::DefaultEditor;

#[tokio::main]
async fn main() {
    let auth_manager = gh::AuthenticationManager::new();
    let auth = auth_manager.cache_auth().await.unwrap();

    let client = reqwest::Client::new();

    let mut copilot_m = copilot::CopilotManager::new(&auth, &client);

    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let input = rl.readline("You: ").unwrap();

        if input == "exit" {
            break;
        }

        copilot_m.ask(&input, true).await;
    }
}
