use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "thongoc")]
#[command(author, version, about = "Local-first TTS Workbench", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Quick speech generation from text, file, or stdin pipe
    Speak {
        /// Text string, or path to a .txt file (leave empty to read from pipe/stdin)
        text: Option<String>,

        /// TTS Engine to use (default: piper)
        #[arg(short, long)]
        engine: Option<String>,

        /// Voice to use
        #[arg(short, long)]
        voice: Option<String>,

        /// Output audio file path (default: narration.wav)
        #[arg(short, long)]
        output: Option<String>,

        /// Automatically play the audio file after generating
        #[arg(short, long)]
        play: bool,
    },
    /// Render all audio segments from script/segments.json
    Render {
        /// Force re-render of all segments even if already generated
        #[arg(short, long)]
        force: bool,
    },
    /// Open the TUI Workbench
    Studio,
    /// Initialize a new Thỏ Ngọc project structure in the current directory
    Init {
        /// Project name (optional)
        name: Option<String>,
    },
    /// Check system dependencies, environment, and engine availability
    Doctor,
}
