import sys
import argparse
import subprocess
from pathlib import Path

# Đảm bảo in UTF-8 không bị lỗi font trên Windows
if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8')
if hasattr(sys.stderr, 'reconfigure'):
    sys.stderr.reconfigure(encoding='utf-8')

# Import bộ chuẩn hóa tiếng Việt của chúng ta
from normalizer import load_dictionary, normalize_text

def synthesize(text: str, output_wav: str, voice_model: str = None):
    project_dir = Path(__file__).parent.parent
    
    # 1. Đường dẫn file dictionary (Ưu tiên thư mục hiện tại của project)
    dict_path = Path.cwd() / "dictionary.toml"
    if not dict_path.exists():
        dict_path = project_dir / "dictionary.toml"

    if voice_model is None:
        voice_model = project_dir / "models" / "piper" / "vi_VN-vais1000-medium.onnx"
    else:
        voice_model = Path(voice_model)
        # Nếu chỉ truyền tên voice (VD: vi_VN-vais1000-medium) thay vì đường dẫn tuyệt đối
        if not voice_model.exists() and not "/" in str(voice_model) and not "\\" in str(voice_model):
            voice_model = project_dir / "models" / "piper" / f"{voice_model.name}.onnx"

    if not voice_model.exists():
        print(f"[Lỗi] Không tìm thấy model tại {voice_model}")
        sys.exit(1)

    # 2. Bước Chuẩn hóa Tiếng Việt (Text Intelligence)
    dictionary = load_dictionary(dict_path)
    cleaned_text = normalize_text(text, dictionary)
    print(f"👉 Văn bản sau chuẩn hóa: \"{cleaned_text}\"")

    # 3. Đẩy vào Piper TTS qua CLI
    cmd = [
        sys.executable, "-m", "piper",
        "-m", str(voice_model),
        "-f", str(output_wav)
    ]

    print(f"🎙️  Đang sinh âm thanh qua Piper (CPU)...")
    try:
        proc = subprocess.Popen(
            cmd,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            encoding="utf-8"
        )
        stdout, stderr = proc.communicate(input=cleaned_text)
        if proc.returncode != 0:
            print(f"[Lỗi Piper] {stderr}")
            sys.exit(1)
        print(f"✓ Đã tạo thành công file âm thanh: {output_wav}")
    except Exception as e:
        print(f"[Lỗi thực thi] {e}")
        sys.exit(1)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Thỏ Ngọc TTS Synthesizer Bridge")
    parser.add_argument("--text", type=str, required=True, help="Văn bản cần đọc")
    parser.add_argument("--output", type=str, default="output.wav", help="File wav đầu ra")
    parser.add_argument("--model", type=str, default=None, help="Đường dẫn file .onnx")

    args = parser.parse_args()
    synthesize(args.text, args.output, args.model)
