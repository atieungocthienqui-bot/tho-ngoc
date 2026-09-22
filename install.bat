@echo off
chcp 65001 > nul
echo.
echo       (\_/)
echo       ( •_•)     Project Thỏ Ngọc v0.1.0 - Installer
echo      / ^>🎙️      Local-first TTS Workbench
echo.

echo [1/3] Đang cài đặt thư viện Python Bridge (piper-tts)...
python -m pip install piper-tts --quiet

if not exist "models\piper\vi_VN-vais1000-medium.onnx" (
    echo [2/3] Đang tải Model Tiếng Việt (vi_VN-vais1000-medium - 63MB)...
    mkdir "models\piper" 2>nul
    powershell -Command "$ProgressPreference = 'SilentlyContinue'; Invoke-WebRequest -Uri 'https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/vi/vi_VN/vais1000/medium/vi_VN-vais1000-medium.onnx' -OutFile 'models\piper\vi_VN-vais1000-medium.onnx'; Invoke-WebRequest -Uri 'https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/vi/vi_VN/vais1000/medium/vi_VN-vais1000-medium.onnx.json' -OutFile 'models\piper\vi_VN-vais1000-medium.onnx.json'"
) else (
    echo [2/3] Model Tiếng Việt đã sẵn sàng.
)

echo [3/3] Đang biên dịch Thỏ Ngọc CLI (Rust Release)...
cargo build --release

if errorlevel 1 (
    echo [!] Biên dịch thất bại. Vui lòng đảm bảo đã cài Rust và C++ Build Tools.
    pause
    exit /b 1
)

copy /Y target\release\tho-ngoc.exe "%USERPROFILE%\.cargo\bin\thongoc.exe" > nul

echo.
echo [✓] Cài đặt hoàn tất 100%%!
echo [👉] Bạn có thể mở CMD bất kỳ và gõ: thongoc doctor
echo.
pause
