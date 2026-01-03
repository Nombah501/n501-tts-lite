# Contributing to N501-TTS Lite

Спасибо за интерес к contribution! Мы рады pull requests от всех.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Standards](#testing-standards)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Pull Request Guidelines](#pull-request-guidelines)

---

## Code of Conduct

Этот проект следует [Code of Conduct](CODE_OF_CONDUCT.md). Пожалуйста, будьте уважительны и конструктивны.

---

## Getting Started

### Prerequisites

- Rust 1.70+ (`rustup install stable`)
- Node.js 18+ (для Tauri frontend)
- Python 3.10+ (для некоторых скриптов)

### Настройка окружения

```bash
# 1. Fork и clone репозитория
git clone https://github.com/Nombah501/n501-tts-lite.git
cd n501-tts-lite

# 2. Установка зависимостей
cargo install cargo-tauri-cli
npm install

# 3. Установка pre-commit hooks (рекомендовано)
pip install pre-commit
pre-commit install
```

---

## Development Workflow

### Branching Strategy

Мы используем simplified Git Flow:

```
main      # Production releases
develop    # Development branch
feature/*  # Individual features
hotfix/*   # Emergency fixes
```

### Process

1. **Создать feature branch из develop:**
   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b feature/your-feature-name
   ```

2. **Разработка и коммиты:**
   ```bash
   # ... код ...
   git add .
   git commit -m "feat(scope): ваше описание"
   ```

3. **Push и создать Pull Request:**
   ```bash
   git push origin feature/your-feature-name
   # Создать PR на GitHub: feature/* → develop
   ```

4. **Code review и merge**

5. **Удалить branch после merge:**
   ```bash
   git checkout develop
   git branch -d feature/your-feature-name
   ```

---

## Coding Standards

### Rust Code Style

- Используйте `cargo fmt` для форматирования
- Используйте `cargo clippy` для linting
- Избегайте `unwrap()` — используйте `expect()` с сообщениями
- Пишите документацию (doc comments) для public API

### Error Handling

- Используйте `Result<T, E>` вместо `panic!` или `unwrap()`
- Создавайте custom error types для доменной логики

---

## Testing Standards

### Unit Tests

- Пишите unit tests для всех public functions
- Target: 80%+ coverage overall, 90%+ core logic
- Используйте `#[test]` атрибут

### Integration Tests

- Тестируйте интеграцию между компонентами
- Используйте mock audio files для reproducibility
- Target: 95%+ clipboard handling coverage

### Running Tests

```bash
# Все тесты
cargo test --all-features

# Тесты с выводом
cargo test -- --nocapture --test-threads=1
```

---

## Commit Message Guidelines

Мы используем Conventional Commits:

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: Новая фича
- `fix`: Bug fix
- `refactor`: Рефакторинг без функциональных изменений
- `docs`: Документация
- `style`: Форматирование (spacing, missing semicolons)
- `test`: Добавление/обновление тестов
- `chore`: Обслуживание задач (dependencies, etc.)
- `perf`: Производительность

### Examples

```
feat(core): добавить поддержку Whisper Medium

Использует Whisper Medium model для улучшенной точности.
Closes #42

fix(ui): исправить отображение кнопки на Wayland

Кнопка не была видна на Wayland. Исправлен z-index.
Fixes #15
```

---

## Pull Request Guidelines

### PR Checklist

Перед созданием PR убедитесь:

- [ ] Code проходит `cargo fmt`
- [ ] Code проходит `cargo clippy` без warnings
- [ ] Все тесты проходят (`cargo test`)
- [ ] Новые функции покрыты тестами (80%+ target)
- [ ] Documentation обновлена (если применимо)
- [ ] Commit messages следуют Conventional Commits
- [ ] PR следует [PR Template](.github/PULL_REQUEST_TEMPLATE.md)

---

## Reporting Issues

### Bug Reports

Используйте [Issue Template](.github/ISSUE_TEMPLATE.md) для багов.

### Feature Requests

Для новых фич:
- Чётко опишите use case
- Почему это важно
- Как вы можете помочь (PR, testing, etc.)

---

## Getting Help

- GitHub Issues: [Open an issue](https://github.com/Nombah501/n501-tts-lite/issues)
- GitHub Discussions: [Ask questions](https://github.com/Nombah501/n501-tts-lite/discussions)
- Email: security@example.com

---

Спасибо за contributing! 🎉
