use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::process::Command;
use colored::*;

pub async fn execute(
    text_arg: Option<&str>,
    engine: Option<&str>,
    _voice: Option<&str>,
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
        "python_bridge/synthesize.py"
    } else {
        r"C:\Users\Admin\.gemini\antigravity\scratch\tho-ngoc\python_bridge\synthesize.py"
    };

    let status = Command::new("python")
        .env("PYTHONIOENCODING", "utf-8")
        .args([
            bridge_script,
            "--text", &content,
            "--output", output_file,
        ])
        .status();

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
