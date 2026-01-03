# N501-TTS Lite

> Высококачественный CLI инструмент для профессиональной диктовки текста с автоматическим помещением в буфер обмена.

## 🎯 Vision

N501-TTS Lite решает проблему низкой продуктивности при написании промптов для AI (6-8 часов в день) через быструю и надёжную транскрибацию речи.

**Больше не нужно печатать — говори и продолжай работать!**

## ✨ Features

### MVP (v1.0)
- ✅ Диктовка → текст в буфере обмена
- ✅ GUI плавающая кнопка (минималистичный интерфейс)
- ✅ Одна локальная модель (Whisper Base)
- ✅ Горячие клавиши (настраиваемые)
- ✅ Уведомления о статусе
- ✅ Basic настройки (язык модели, тест микрофона)

### Планируется
- [ ] Выбор локальных моделей (Tiny/Base/Medium/Large)
- [ ] Облачные API (OpenAI, Google)
- [ ] Advanced настройки (hot-reload, логирование)
- [ ] CLI интерфейс
- [ ] Multiple usage modes (Queue, File)
- [ ] Text-to-Audio (v1.5+)

## 🚀 Quick Start

### Установка

**Linux/macOS:**
```bash
# Скачать и установить
curl -sSL https://raw.githubusercontent.com/Nombah501/n501-tts-lite/main/scripts/install.sh | bash

# Или через cargo (после релиза)
cargo install n501-tts-lite
```

**Windows:**
```bash
# Скачать .exe installer
# https://github.com/Nombah501/n501-tts-lite/releases/latest

# Запустить installer
```

### Первая настройка

```bash
# Запустить интерактивный wizard
tts-lite --setup
```

Wizard проводит через:
1. Выбор языка модели
2. Тест микрофона
3. Первая транскрибация

## 📖 Usage

### GUI (плавающая кнопка)

```
1. Нажмите горячую клавишу (дефолт: Cmd+Shift+V)
2. Наговорите текст
3. Нажмите снова для завершения
4. Текст автоматически скопирован в буфер обмена
5. Вставьте куда нужно (Ctrl+V / Cmd+V)
```

### Настройки горячих клавиш

```bash
# Изменить горячую клавишу
tts-lite config set hotkey "Cmd+Shift+T"

# Или редактировать config file напрямую
~/.config/tts-lite/config.yaml
```

## 🏗️ Architecture

- **Language:** Rust
- **GUI Framework:** Tauri
- **Model:** Whisper (Base для MVP)
- **Platform:** Cross-platform (Linux, macOS, Windows)

## 📊 Quality

- **Test Coverage:** 80%+ (90%+ core logic, 95%+ clipboard)
- **Accuracy:** 95%+ транскрибации, <5% WER
- **Performance:** P95 транскрибации <5 секунд
- **Installation:** <5 минут для "download and go"

## 🤝 Contributing

Мы рады contributions! Пожалуйста:

1. Fork репозитория
2. Создать feature branch (`git checkout -b feature/amazing-feature`)
3. Commit ваши изменения (`git commit -m 'feat: добавить amazing feature'`)
4. Push к branch (`git push origin feature/amazing-feature`)
5. Открыть Pull Request

### Code Style

- Используйте Conventional Commits
- `cargo fmt` перед коммитом
- `cargo clippy` для проверки warnings
- Добавляйте тесты для новых функций

## 📄 License

MIT License — см. [LICENSE](LICENSE) для деталей

## 🙏 Acknowledgments

- [Whisper](https://github.com/openai/whisper) — OpenAI speech recognition
- [Tauri](https://tauri.app/) — Cross-platform GUI framework
- [Rust](https://www.rust-lang.org/) — The programming language

## 📞 Support

- **GitHub Issues:** [Submit issue](https://github.com/Nombah501/n501-tts-lite/issues)
- **Discussions:** [GitHub Discussions](https://github.com/Nombah501/n501-tts-lite/discussions)
- **Email:** your-email@example.com

## 🗺️ Roadmap

### v1.1 (Q1 2026)
- Выбор локальных моделей
- Облачные API (OpenAI, Google)
- Advanced настройки

### v1.2 (Q2 2026)
- CLI интерфейс
- Multiple usage modes
- History transcriptions

### v1.5 (Q3 2026)
- Text-to-Audio
- Выбор голосов
- Настройка характеристик

### v2.0 (Q4 2026)
- Интеграция с другими инструментами
- Cloud sync настроек
- Team/enterprise features

---

**Built by developers, for developers** 🚀
