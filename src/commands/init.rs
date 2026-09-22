use std::fs;
use std::path::Path;
use colored::*;

pub async fn execute(name: Option<&str>) {
    let proj_name = name.unwrap_or("my-speech-project");
    println!("\n      (\\_/)");
    println!("      ( •_•)     Project Thỏ Ngọc");
    println!("     / >📁      Initializing new project: {}\n", proj_name.cyan());

    // Tạo các thư mục dự án
    let dirs = vec!["script", "audio"];
    for dir in dirs {
        if !Path::new(dir).exists() {
            if let Err(e) = fs::create_dir(dir) {
                eprintln!("❌ Không thể tạo thư mục '{}': {}", dir, e);
                return;
            }
        }
    }

    // 1. project.toml
    let project_toml = format!(
        "[project]\nname = \"{}\"\nversion = \"0.1.0\"\nlanguage = \"vi\"\n\n[defaults]\nengine = \"piper\"\nvoice = \"vi_VN-vais1000-medium\"\nspeed = 1.0\n",
        proj_name
    );
    let _ = fs::write("project.toml", project_toml);

    // 2. dictionary.toml
    let dict_toml = r#"[pronunciation.exact]
"ESP32" = "E S P ba hai"
"AI" = "ây ai"
"Thỏ Ngọc" = "Thỏ Ngọc"

[pronunciation.regex]
"(?i)(\\d+)\\s?km/h" = "\\1 ki lô mét trên giờ"
"(\\d+)[\\.,](\\d+)\\s?triệu" = "\\1 phẩy \\2 triệu"
"\\b(\\d{1,2})/(\\d{1,2})/(\\d{4})\\b" = "ngày \\1 tháng \\2 năm \\3"
"#;
    let _ = fs::write("dictionary.toml", dict_toml);

    // 3. script/segments.json
    let segments_json = r#"[
  {
    "id": "001",
    "text": "Chào mừng bạn đến với Project Thỏ Ngọc.",
    "character": "Narrator",
    "status": "ready",
    "audio_file": "audio/001.wav"
  },
  {
    "id": "002",
    "text": "Đây là bàn làm việc tạo giọng nói cục bộ dành cho mọi người.",
    "character": "Narrator",
    "status": "ready",
    "audio_file": "audio/002.wav"
  }
]
"#;
    let _ = fs::write("script/segments.json", segments_json);

    println!("✓ {} project.toml", "Đã tạo".green());
    println!("✓ {} dictionary.toml", "Đã tạo".green());
    println!("✓ {} script/segments.json", "Đã tạo".green());
    println!("✓ {} audio/", "Đã tạo".green());

    println!("\n✨ Khởi tạo dự án thành công!");
    println!("👉 Gõ '{}' để xuất toàn bộ âm thanh kịch bản.", "thongoc render".bold());
}
