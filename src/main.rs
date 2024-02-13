mod copilot;
mod gh;
mod headers;
mod prompts;
mod urls;
mod utils;

use oxc_allocator;
use rustyline::DefaultEditor;

#[tokio::main]
async fn main() {
    let auth_manager = gh::AuthenticationManager::new();
    let auth = auth_manager.cache_auth().await.unwrap();

    let client = reqwest::Client::new();

    let allocator = oxc_allocator::Allocator::default();

    let mut copilot_m = copilot::CopilotManager::new(&auth, &client, &allocator, prompts::COPILOT_INSTRUCTIONS);

    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let input = rl.readline("You: ").unwrap();

        if input == "exit" {
            break;
        }

        let msg = copilot_m.ask(&input, true).await;

        println!("===COPILOT===");
        println!("{:#?}", msg);
    }
}
