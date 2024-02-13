mod copilot;
mod gh;
mod headers;
mod prompts;
mod urls;
mod utils;
mod term;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write};

use oxc_allocator;
use rustyline::DefaultEditor;

fn move_up_one_line() {
    print!("\x1b[1A");
    std::io::stdout().flush().unwrap();
}

#[tokio::main]
async fn main() {
    // enter alternate screen
    execute!(stdout(), EnterAlternateScreen).unwrap();

    let auth_manager = gh::AuthenticationManager::new();
    let auth = auth_manager.cache_auth().await.unwrap();

    let client = reqwest::Client::new();

    let allocator = oxc_allocator::Allocator::default();

    let mut copilot_m = copilot::CopilotManager::new(&auth, &client, &allocator, prompts::COPILOT_INSTRUCTIONS);

    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let input = rl.readline("You: ").unwrap();

        move_up_one_line();

        if input == "exit" {
            break;
        }

        let _msg = copilot_m.ask(&input, true).await;
        // reset the forground color
        print!("\033[0m");
        // syntax highlighting
        // let highlighted = term::highlight_text(&msg.content);
        // println!("{}", highlighted);

    }

    // leave alternate screen
    execute!(stdout(), LeaveAlternateScreen).unwrap();
}
