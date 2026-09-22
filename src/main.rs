mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Speak { text, engine, voice, output, play }) => {
            commands::speak::execute(
                text.as_deref(),
                engine.as_deref(),
                voice.as_deref(),
                output.as_deref(),
                *play,
            ).await;
        }
        Some(Commands::Render { force }) => {
            commands::render::execute(*force).await;
        }
        Some(Commands::Init { name }) => {
            commands::init::execute(name.as_deref()).await;
        }
        Some(Commands::Studio) => {
            commands::studio::execute().await;
        }
        Some(Commands::Doctor) => {
            commands::doctor::execute().await;
        }
        None => {
            // Mascot khi chạy lệnh `thongoc` không tham số
            println!("      (\\_/)");
            println!("      ( •_•)     Project Thỏ Ngọc");
            println!("     / >🎙️      Local-first TTS Workbench");
            println!("\nRun `thongoc --help` for available commands.");
        }
    }
}
