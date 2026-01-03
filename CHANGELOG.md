# Change Log

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added
- GitHub Actions CI/CD workflows
- Git repository setup with best practices
- Project documentation (README, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY)
- Issue and PR templates
- Pre-commit hooks configuration
- Codeowners configuration
- Semantic versioning setup (release-plz)

### Changed
- Initial project structure

## [1.0.0] - TBD

### Added (MVP Features)
- Диктовка → текст в буфере обмена
- GUI плавающая кнопка (минималистичный интерфейс)
- Одна локальная модель (Whisper Base)
- Горячие клавиши (настраиваемые)
- Уведомления о статусе (запись/успех/ошибка)
- Basic настройки (язык модели, тест микрофона)

### Performance Targets
- Cold start time: <2s
- Warm start time: <100ms
- P50 transcription time: 2s
- P95 transcription time: 5s
- P99 transcription time: 10s

### Quality Targets
- Test coverage: 80%+ (90%+ core logic, 95%+ clipboard)
- Transcription accuracy: 95%+ (Whisper Base)
- Installation time: <5 minutes

### Dependencies
- Rust 1.70+
- Tauri 1.0+
- Whisper Base model

---

## Format

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- New feature

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Now removed features

### Fixed
- Bug fix

### Security
- Vulnerability fix
