use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::process::Command;
use colored::*;

#[derive(serde::Deserialize, Debug)]
struct ProjectConfig {
    defaults: DefaultsConfig,
}

#[derive(serde::Deserialize, Debug)]
struct DefaultsConfig {
    #[allow(dead_code)]
    engine: String,
    voice: String,
}

fn get_project_voice() -> Option<String> {
    if let Ok(content) = fs::read_to_string("project.toml") {
        if let Ok(config) = toml::from_str::<ProjectConfig>(&content) {
            return Some(config.defaults.voice);
        }
    }
    None
}

pub async fn execute(
    text_arg: Option<&str>,
    engine: Option<&str>,
    voice: Option<&str>,
    output: Option<&str>,
    play: bool,
) {
    let engine_name = engine.unwrap_or("piper");
    if engine_name != "piper" {
        eprintln!("{} Engine '{}' is not supported yet.", "Error:".red().bold(), engine_name);
        eprintln!("Currently supported: piper");
        return;
    }
    if engine_name != "piper" {
        eprintln!("{} Engine '{}' is not supported yet.", "Error:".red().bold(), engine_name);
        eprintln!("Currently supported: piper");
        return;
    }
    let output_file = output.unwrap_or("narration.wav");

    // 1. XÃ¡c Ä‘á»‹nh ná»™i dung cáº§n Ä‘á»c: Tá»« file, tá»« Pipe/Stdin, hay tá»« chuá»—i trá»±c tiáº¿p
    let content = match text_arg {
        Some("-") | None => {
            // Äá»c tá»« Stdin (Unix Pipe: echo "..." | thongoc speak)
            let mut buffer = String::new();
            if let Err(e) = io::stdin().read_to_string(&mut buffer) {
                eprintln!("âŒ Lá»—i khi Ä‘á»c tá»« Ä‘Æ°á»ng á»‘ng Pipe: {}", e);
                return;
            }
            buffer.trim().to_string()
        }
        Some(val) => {
            let path = Path::new(val);
            if path.is_file() {
                // Äá»c tá»« file ká»‹ch báº£n (thongoc speak script.txt)
                match fs::read_to_string(path) {
                    Ok(data) => {
                        println!("ðŸ“„ ÄÃ£ Ä‘á»c ná»™i dung tá»« file: {}", val.cyan());
                        data.trim().to_string()
                    }
                    Err(e) => {
                        eprintln!("âŒ KhÃ´ng thá»ƒ Ä‘á»c file '{}': {}", val, e);
                        return;
                    }
                }
            } else {
                // Nháº­p trá»±c tiáº¿p chuá»—i text
                val.to_string()
            }
        }
    };

    if content.is_empty() {
        eprintln!("âš ï¸  VÄƒn báº£n rá»—ng! Vui lÃ²ng nháº­p ná»™i dung hoáº·c truyá»n file.");
        return;
    }

    println!("\n      (\\_/)");
    println!("      ( â€¢_â€¢)     Project Thá» Ngá»c");
    println!("     / >ðŸŽ™ï¸      Synthesizing speech...\n");

    let preview = if content.chars().count() > 60 {
        format!("{}...", content.chars().take(60).collect::<String>())
    } else {
        content.clone()
    };

    println!("{:<12}: \"{}\"", "VÄƒn báº£n", preview);
    println!("{:<12}: {}", "Engine", engine_name.green());
    println!("{:<12}: {}\n", "File xuáº¥t", output_file.cyan());

    // 2. Gá»i Python Bridge (Tá»± Ä‘á»™ng tÃ¬m Ä‘Æ°á»ng dáº«n ká»ƒ cáº£ khi Ä‘á»©ng á»Ÿ á»• Ä‘Ä©a khÃ¡c)
    let bridge_path = if Path::new("python_bridge").join("synthesize.py").exists() {
        std::path::PathBuf::from("python_bridge").join("synthesize.py")
    } else if let Ok(val) = std::env::var("THONGOC_HOME") {
        std::path::PathBuf::from(val).join("python_bridge").join("synthesize.py")
    } else {
        eprintln!("\n{} Cannot find python_bridge/synthesize.py.", "Error:".red().bold());
        eprintln!("Please run this command from the Thá» Ngá»c repository root,");
        eprintln!("or set the THONGOC_HOME environment variable.");
        return;
    };

    let mut cmd = Command::new("python");
    cmd.env("PYTHONIOENCODING", "utf-8");
    cmd.arg(bridge_path);
    cmd.arg("--text").arg(&content);
    cmd.arg("--output").arg(output_file);
    
    // Voice arg from clap > project.toml defaults
    let resolved_voice = voice.map(|s| s.to_string()).or_else(get_project_voice);
    if let Some(v) = resolved_voice {
        cmd.arg("--model").arg(v);
    }

    let status = cmd.status();

    match status {
        Ok(s) if s.success() => {
            println!("\nâœ¨ {} ÄÃ£ xuáº¥t file: '{}'", "HoÃ n thÃ nh!".green(), output_file.bold());

            // 3. Tá»± Ä‘á»™ng phÃ¡t Ã¢m thanh náº¿u cÃ³ cá» --play
            if play {
                if cfg!(target_os = "windows") {
                    println!("ðŸ”Š Äang phÃ¡t Ã¢m thanh...");
                let _ = Command::new("powershell")
                    .args([
                        "-NoProfile",
                        "-Command",
                        &format!("(New-Object Media.SoundPlayer '{}').PlaySync()", output_file),
                    ])
                    .status();
                } else {
                    println!("⚠️ --play is currently supported on Windows only.");
                }
            }
        }
        Ok(s) => {
            eprintln!("\nâŒ {} (Exit code: {:?})", "QuÃ¡ trÃ¬nh sinh Ã¢m thanh tháº¥t báº¡i".red(), s.code());
        }
        Err(e) => {
            eprintln!("\nâŒ KhÃ´ng thá»ƒ gá»i Python Bridge: {}", e);
        }
    }
}

