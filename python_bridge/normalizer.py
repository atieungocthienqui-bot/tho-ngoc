import tomllib
import re
from pathlib import Path

def load_dictionary(dict_path: str) -> dict:
    """Tải file TOML chứa các quy tắc phát âm."""
    try:
        with open(dict_path, "rb") as f:
            return tomllib.load(f)
    except FileNotFoundError:
        print(f"[Cảnh báo] Không tìm thấy {dict_path}. Sử dụng quy tắc rỗng.")
        return {}

def normalize_text(text: str, dictionary: dict) -> str:
    """Chuẩn hóa chuỗi văn bản đầu vào dựa trên dictionary.toml."""
    normalized = text
    
    # 1. Áp dụng các quy tắc Thay thế chính xác (Exact Matches)
    exact_rules = dictionary.get("pronunciation", {}).get("exact", {})
    for word, replacement in exact_rules.items():
        # Dùng regex \b để đảm bảo chỉ thay thế toàn bộ từ (whole word match)
        # Ví dụ: "AI" -> "ây ai", nhưng không làm hỏng từ "THAI"
        pattern = re.compile(rf'\b{re.escape(word)}\b', re.IGNORECASE)
        normalized = pattern.sub(replacement, normalized)

    # 2. Áp dụng các quy tắc Biểu thức chính quy (Regex Rules)
    regex_rules = dictionary.get("pronunciation", {}).get("regex", {})
    for pattern_str, replacement in regex_rules.items():
        try:
            pattern = re.compile(pattern_str)
            # Python re.sub hỗ trợ thay thế group bằng \1, \2... giống hệt cấu hình TOML
            normalized = pattern.sub(replacement, normalized)
        except re.error as e:
            print(f"[Lỗi] Quy tắc Regex không hợp lệ '{pattern_str}': {e}")
            
    # 3. Áp dụng đọc số lớn (Chỉ xử lý các số đứng riêng lẻ từ 4 chữ số trở lên)
    def num_to_vi(match):
        n_str = match.group(0)
        n = int(n_str)
        if n == 0: return "không"
        
        units_name = ["", "nghìn", "triệu", "tỷ", "nghìn tỷ"]
        UNITS = ["không", "một", "hai", "ba", "bốn", "năm", "sáu", "bảy", "tám", "chín"]
        
        chunks = []
        temp = n
        while temp > 0:
            chunks.append(temp % 1000)
            temp //= 1000
            
        words = []
        for i, chunk in enumerate(chunks):
            if chunk == 0: continue
            
            h = chunk // 100
            rem = chunk % 100
            t = rem // 10
            u = rem % 10
            
            res = []
            is_highest = (i == len(chunks) - 1)
            
            if h > 0:
                res.append(f"{UNITS[h]} trăm")
            elif not is_highest:
                res.append("không trăm")
                
            if t > 1:
                res.append(f"{UNITS[t]} mươi")
            elif t == 1:
                res.append("mười")
            elif t == 0 and u > 0 and (h > 0 or not is_highest):
                res.append("lẻ")
                
            if u > 0:
                if u == 5 and t > 0: res.append("lăm")
                elif u == 1 and t > 1: res.append("mốt")
                elif u == 4 and t > 1: res.append("tư")
                else: res.append(UNITS[u])
                
            words.insert(0, f"{' '.join(res)} {units_name[i]}".strip())
            
        return " ".join(words).strip()
        
    # Chỉ đọc những số đứng độc lập có 4 chữ số trở lên (để tránh phá cấu trúc ngày tháng hoặc phiên bản)
    normalized = re.sub(r'\b\d{4,}\b', num_to_vi, normalized)
            
    return normalized

if __name__ == "__main__":
    # Kịch bản test khét lẹt
    test_text = "Hôm nay 12/09/2026, AI dùng chip ESP32 chạy tốc độ 15 km/h. Doanh thu của chúng tôi đạt 1550400 đồng."
    
    print("\n🐇 [THỎ NGỌC] - VIETNAMESE TEXT NORMALIZER TEST\n")
    print(f"👉 VĂN BẢN GỐC:\n{test_text}\n")
    
    dict_path = Path(__file__).parent.parent / "dictionary.toml"
    
    dictionary = load_dictionary(dict_path)
    result = normalize_text(test_text, dictionary)
    
    print(f"✨ SAU KHI CHUẨN HÓA (Đẩy vào TTS):\n{result}\n")
