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
    let output_file = output.unwrap_or("narration.wav");

    // 1. Xác định nội dung cần đọc: Từ file, từ Pipe/Stdin, hay từ chuỗi trực tiếp
    let content = match text_arg {
        Some("-") | None => {
            // Đọc từ Stdin (Unix Pipe: echo "..." | thongoc speak)
            let mut buffer = String::new();
            if let Err(e) = io::stdin().read_to_string(&mut buffer) {
                eprintln!("❌ Lỗi khi đọc từ đường ống Pipe: {}", e);
                return;
            }
            buffer.trim().to_string()
        }
        Some(val) => {
            let path = Path::new(val);
            if path.is_file() {
                // Đọc từ file kịch bản (thongoc speak script.txt)
                match fs::read_to_string(path) {
                    Ok(data) => {
                        println!("📄 Đã đọc nội dung từ file: {}", val.cyan());
                        data.trim().to_string()
                    }
                    Err(e) => {
                        eprintln!("❌ Không thể đọc file '{}': {}", val, e);
                        return;
                    }
                }
            } else {
                // Nhập trực tiếp chuỗi text
                val.to_string()
            }
        }
    };

    if content.is_empty() {
        eprintln!("⚠️  Văn bản rỗng! Vui lòng nhập nội dung hoặc truyền file.");
        return;
    }

    println!("\n      (\\_/)");
    println!("      ( •_•)     Project Thỏ Ngọc");
    println!("     / >🎙️      Synthesizing speech...\n");

    let preview = if content.chars().count() > 60 {
        format!("{}...", content.chars().take(60).collect::<String>())
    } else {
        content.clone()
    };

    println!("{:<12}: \"{}\"", "Văn bản", preview);
    println!("{:<12}: {}", "Engine", engine_name.green());
    println!("{:<12}: {}\n", "File xuất", output_file.cyan());

    // 2. Gọi Python Bridge (Tự động tìm đường dẫn kể cả khi đứng ở ổ đĩa khác)
    let bridge_script = if Path::new("python_bridge/synthesize.py").exists() {
        "python_bridge/synthesize.py".to_string()
    } else if let Ok(val) = std::env::var("THONGOC_HOME") {
        format!("{}/python_bridge/synthesize.py", val)
    } else {
        eprintln!("\n{} Cannot find python_bridge/synthesize.py.", "Error:".red().bold());
        eprintln!("Please run this command from the Thỏ Ngọc repository root,");
        eprintln!("or set the THONGOC_HOME environment variable.");
        return;
    };

    let mut cmd = Command::new("python");
    cmd.env("PYTHONIOENCODING", "utf-8");
    cmd.arg(&bridge_script);
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
            println!("\n✨ {} Đã xuất file: '{}'", "Hoàn thành!".green(), output_file.bold());

            // 3. Tự động phát âm thanh nếu có cờ --play
            if play {
                println!("🔊 Đang phát âm thanh...");
                let _ = Command::new("powershell")
                    .args([
                        "-NoProfile",
                        "-Command",
                        &format!("(New-Object Media.SoundPlayer '{}').PlaySync()", output_file),
                    ])
                    .status();
            }
        }
        Ok(s) => {
            eprintln!("\n❌ {} (Exit code: {:?})", "Quá trình sinh âm thanh thất bại".red(), s.code());
        }
        Err(e) => {
            eprintln!("\n❌ Không thể gọi Python Bridge: {}", e);
        }
    }
}
