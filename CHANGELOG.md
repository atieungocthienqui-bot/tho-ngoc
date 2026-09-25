# Changelog

Tất cả những thay đổi nổi bật của dự án này sẽ được ghi chú trong file này.

Dự án tuân thủ [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - Tightening the Burrow 🐇

### Fixed
- Clarified Piper as the currently supported TTS engine
- Corrected low-resource mode documentation
- Improved platform-specific `--play` behavior
- Clarified render skip behavior (avoiding the word 'cache' until content-hash is implemented)
- Improved release/version consistency

## [0.1.1]

### Added
- Thêm script cài đặt cho Windows (`install.bat`) giúp tự động cài tải python bridge và model.
- Lệnh `thongoc doctor` giờ kiểm tra cấu hình thật bằng `sysinfo` và soát biến môi trường `THONGOC_HOME`.
- Thêm file `CONTRIBUTING.md` và Issue Templates (Bug Report, Feature Request) để đón cộng đồng.

### Changed
- Cập nhật thư viện `Cargo.toml`.
- Cảnh báo người dùng đây chỉ là early prototype (v0.1.x).
