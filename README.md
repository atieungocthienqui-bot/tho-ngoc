# Project Thỏ Ngọc 🌕🐇

```text
      (\_/)
      ( •_•)     Project Thỏ Ngọc (Moon Rabbit TTS)
     / >🎙️      Local-first TTS Workbench for your Terminal
```

![Version](https://img.shields.io/badge/version-0.1.0-green)
![Rust](https://img.shields.io/badge/CLI-Rust-orange)
![Python](https://img.shields.io/badge/Bridge-Python_3.11+-blue)
![License](https://img.shields.io/badge/license-MIT-brightgreen)

**Your terminal has a voice.**

**Thỏ Ngọc (Moon Rabbit TTS)** là một bàn làm việc âm thanh (TTS Workbench) mã nguồn mở, chạy 100% cục bộ trên máy tính cá nhân và hoạt động trực tiếp trong Terminal.

Thỏ Ngọc **không phải** là một dịch vụ web bán giọng đọc AI, mà là một **bộ công cụ dựng nháp âm thanh (Toolchain)** dành cho YouTuber, Content Creator, Indie Game Developer và người dùng yêu thích dòng lệnh — những người cần tạo hàng trăm bản nháp kịch bản mà không lo đốt tiền credit.

---

## 🐇 Triết lý thiết kế (Core Philosophy)

- **Bring Your Own Computer (BYOC):** Không tài khoản, không cloud, không giới hạn ký tự. Bạn có thể tạo 2 bản nháp hoặc 200 bản nháp hoàn toàn miễn phí bằng chính CPU của mình.
- **Vietnamese Text Intelligence:** Lớp chuẩn hóa văn bản tiếng Việt mã nguồn mở. Tự động dịch ngày tháng (`12/09/2026`), đơn vị đo (`15 km/h`), tiền tệ, số lớn (`1550400`) và từ viết tắt (`AI`, `ESP32`) thông qua từ điển tùy chỉnh `dictionary.toml`.
- **Git-trackable Audio Projects:** Mọi dự án đều lưu dưới dạng file văn bản thuần (`project.toml`, `dictionary.toml`, `script/segments.json`). Dễ dàng quản lý phiên bản kịch bản và phát âm bằng Git.
- **🪶 Potato Mode:** Tối ưu hóa cho máy tính cấu hình phổ thông, chạy mượt trên CPU với các model siêu nhẹ như Piper ONNX.
- **Unix Pipeline Ready:** Hỗ trợ truyền văn bản qua Pipe (`|`), đọc từ file `.txt`, tự động phát loa (`--play`) và kết hợp dễ dàng với `ffmpeg`.

---

## 🚀 Cài đặt nhanh (Quick Install - v0.1.0)

### Yêu cầu
- **Rust** (`rustup.rs` kèm C++ Build Tools trên Windows)
- **Python 3.11+**

### Cài đặt tự động (Automated Windows installer script)
Clone repository và chạy file `install.bat`:
```cmd
git clone https://github.com/atieungocthienqui-bot/tho-ngoc.git
cd tho-ngoc
install.bat
```
*(Script dùng thử nguyên mẫu: Tự động cài đặt `piper-tts`, tải model tiếng Việt 63MB và build lệnh `thongoc` qua cargo).*

---

## 🛠️ Hướng dẫn sử dụng (Usage)

### 1. Kiểm tra hệ thống ("Hang Thỏ")
*(Hiện tại chỉ hiển thị giả lập mockup prototype)*
```bash
thongoc doctor
```

### 2. Tạo giọng nói tức thì & Tự phát qua loa (`--play`)
```bash
thongoc speak "Xin chào, tôi là Thỏ Ngọc." --play
```

### 3. Đọc kịch bản từ file `.txt` hoặc Unix Pipe
```bash
# Đọc trực tiếp từ file kịch bản
thongoc speak script.txt -o narration.wav --play

# Hoặc truyền qua đường ống Unix Pipe
echo "Hôm nay 12/09/2026, AI dùng chip ESP32 chạy tốc độ 15 km/h" | thongoc speak -o demo.wav
```

### 4. Khởi tạo dự án & Render hàng loạt (Workbench Workflow)
```bash
# Tạo cấu trúc dự án mới (project.toml, dictionary.toml, script/segments.json)
thongoc init "video-lich-su-01"

# Render toàn bộ các đoạn thoại trong segments.json ra thư mục audio/
thongoc render

# Bỏ qua các đoạn đã render nếu file âm thanh đầu ra vẫn tồn tại.
# Dùng --force nếu muốn ép render lại toàn bộ:
thongoc render --force
```

---

## 🧠 Tùy chỉnh phát âm với `dictionary.toml`

Bạn có thể dạy Thỏ Ngọc cách phát âm bất kỳ từ chuyên ngành, tên riêng hoặc ký hiệu nào:

```toml
[pronunciation.exact]
"ESP32" = "E S P ba hai"
"AI" = "ây ai"
"ElevenLabs" = "i le vần láp"

[pronunciation.regex]
"(?i)(\\d+)\\s?km/h" = "\\1 ki lô mét trên giờ"
"(\\d+)[\\.,](\\d+)\\s?triệu" = "\\1 phẩy \\2 triệu"
"\\b(\\d{1,2})/(\\d{1,2})/(\\d{4})\\b" = "ngày \\1 tháng \\2 năm \\3"
```

---

## 🏗️ Kiến trúc hệ thống

*(Lưu ý: TUI bằng ratatui hiện đang nằm trên Roadmap/Planned cho v0.2.0, lệnh `studio` hiện tại là placeholder)*

```text
          Rust CLI (clap) + TUI (ratatui - planned)
                        │
                  Engine Manager
                        │
         ┌──────────────┴──────────────┐
         ▼                             ▼
 Vietnamese Normalizer          TTS Engine Runtime
   (dictionary.toml)           (Piper ONNX / Local)
```

---

## 📜 Giấy phép (License)
Mã nguồn của **Project Thỏ Ngọc** được phát hành dưới giấy phép **MIT License**. Các mô hình TTS và tập dữ liệu giọng nói bên thứ ba (như Piper, VAIS1000, Kokoro) tuân theo giấy phép gốc của từng tác giả.
