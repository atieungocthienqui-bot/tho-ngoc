use std::fs;
use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};
use colored::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Segment {
    pub id: String,
    pub character: Option<String>,
    pub text: String,
    #[serde(default = "default_status")]
    pub status: String,
    pub audio_file: Option<String>,
}

fn default_status() -> String {
    "ready".to_string()
}

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

pub async fn execute(force: bool) {
    let script_path = Path::new("script/segments.json");
    if !script_path.exists() {
        eprintln!("❌ Không tìm thấy 'script/segments.json'. Hãy chạy 'thongoc init' trước!");
        return;
    }

    println!("\n      (\\_/)");
    println!("      ( •_•)     Project Thỏ Ngọc");
    println!("     / >🎙️      Rendering project segments...\n");

    let data = match fs::read_to_string(script_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("❌ Lỗi đọc kịch bản: {}", e);
            return;
        }
    };

    let mut segments: Vec<Segment> = match serde_json::from_str(&data) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Kịch bản segments.json bị lỗi cú pháp: {}", e);
            return;
        }
    };

    let total = segments.len();
    let mut rendered_count = 0;
    let mut skipped_count = 0;

    for seg in segments.iter_mut() {
        let target_file = seg.audio_file.clone().unwrap_or_else(|| format!("audio/{}.wav", seg.id));
        let file_exists = Path::new(&target_file).exists();

        // Nếu đã generate rồi và không dùng cờ --force thì bỏ qua (Cache thông minh!)
        if !force && seg.status == "generated" && file_exists {
            println!("[BỎ QUA] [{}] File đã tồn tại: {}", seg.id.bold(), target_file.bright_black());
            skipped_count += 1;
            continue;
        }

        println!("[RENDER] [{}] \"{}\" -> {}", seg.id.cyan(), seg.text, target_file.yellow());

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
        cmd.arg("--text").arg(&seg.text);
        cmd.arg("--output").arg(&target_file);
        
        if let Some(v) = get_project_voice() {
            cmd.arg("--model").arg(v);
        }

        let status = cmd.status();

        match status {
            Ok(s) if s.success() => {
                seg.status = "generated".to_string();
                seg.audio_file = Some(target_file);
                rendered_count += 1;
            }
            Ok(_) => {
                eprintln!("❌ Thất bại khi sinh đoạn [{}]", seg.id);
            }
            Err(e) => {
                eprintln!("❌ Lỗi gọi Python Bridge: {}", e);
                break;
            }
        }
    }

    // Ghi lại trạng thái vào segments.json
    if let Ok(updated_json) = serde_json::to_string_pretty(&segments) {
        let _ = fs::write(script_path, updated_json);
    }

    println!("\n✨ {} Đã render: {}/{} đoạn (Bỏ qua: {} đoạn cũ)", 
        "Hoàn tất!".green().bold(), 
        rendered_count.to_string().green(), 
        total, 
        skipped_count.to_string().cyan()
    );
}
