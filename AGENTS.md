# Agent Guidelines for This Repository

## Основные принципы

### Фреймворк BMad
- Все разработки ведутся с использованием фреймворка BMad
- При создании новых компонентов и функций следуйте паттернам BMad
- Используйте встроенные утилиты и библиотеки фреймворка

### Язык разработки
- **Русский язык** для комментариев в коде, документации, README, commit messages
- **Английский язык** для переменных, функций, классов (стандарт разработки)
- Документация для пользователей - на русском языке

### Работа с MCP (Model Context Protocol)
Используйте MCP серверы для конкретных задач:

- **context7-mcp**: поиск документации библиотек, когда нужно узнать как использовать пакет/фреймворк
- **puppeteer-mcp-server**: автоматизация браузера (скриншоты, тестирование UI, веб-скрейпинг)
- **mcp-git**: Git операции (статус, diff, коммиты, создание веток)
- **mcp-server-brave-search**: поиск информации в интернете, актуальные данные
- **mcp-server-sequential-thinking**: разложение сложных задач на шаги, планирование

Разбивайте сложные задачи на подзадачи и используйте соответствующие MCP инструменты.

### Git и коммиты
- Делайте коммиты **своевременно** - после завершения каждого логического блока работы
- Не копите изменения в одной большой работе
- Сообщения коммитов на русском языке, в повелительном наклонении
- Форматы:
  - `[тип]: краткое описание`
  - `тип(scope): описание`
  - Типы: `feat:`, `fix:`, `refactor:`, `docs:`, `test:`, `chore:`
- Примеры: `[feat]: добавить авторизацию пользователей`, `fix(auth): исправить ошибку токена`

## Build, Lint, and Test Commands

### Git Commands
- Используйте mcp-git инструменты для всех операций с Git
- `git status` - показать статус рабочего дерева
- `git diff` - показать изменения в рабочей директории
- `git diff staged` - показать измененные файлы, готовые к коммиту
- `git diff [branch]` - показать различия между ветками
- `git commit` - создать коммит с сообщением
- `git add` - добавить файлы в staging area
- `git reset` - отмена staging изменений
- `git log` - показать историю коммитов
- `git create-branch` - создать новую ветку
- `git checkout` - переключиться на ветку
- `git stash` - сохранить изменения в рабочей директории
- `git stash pop` - восстановить и удалить стеш

### Essential Commands
> Примечание: команды адаптировать после выбора технологического стека

- `[npm|yarn|pnpm] install` - установить все зависимости
- `[npm|yarn|pnpm] run build` - собрать проект для продакшена
- `[npm|yarn|pnpm] run dev` - запустить dev сервер или watch режим
- `[npm|yarn|pnpm] run lint` - проверить код с линтером
- `[npm|yarn|pnpm] run lint:fix` - автоматически исправить ошибки линтера
- `[npm|yarn|pnpm] run format` - форматировать код
- `[npm|yarn|pnpm] run test` - запустить все тесты
- `[npm|yarn|pnpm] run test:watch` - запустить тесты в watch режиме
- `[npm|yarn|pnpm] run test:coverage` - запустить тесты с отчетом покрытия
- `[npm|yarn|pnpm] run typecheck` - проверить типы

### Running Single Tests
- `[npm|yarn|pnpm] test -- <path-to-test-file>` - запустить конкретный тестовый файл
- `[npm|yarn|pnpm] test -- -t "<test-name>"` - запустить тесты по названию
- `[npm|yarn|pnpm] test -- --grep "<pattern>"` - запустить тесты по регулярному выражению

## Code Style Guidelines

### Imports
- Use ES6 import syntax: `import { foo } from 'module'`
- Group imports: 1) Built-ins, 2) External packages, 3) Internal modules
- Sort imports alphabetically within each group
- Keep default imports separate: `import React, { useState } from 'react'`

### Formatting
- 2 spaces indentation (no tabs)
- Single quotes for strings
- Max line length: 100 characters
- Trailing commas in multi-line arrays/objects/imports
- No semicolons unless required

### Types
- Use interfaces for object shapes: `interface User { name: string }`
- Use type aliases for unions/primitives: `type ID = string | number`
- Avoid `any` - use `unknown`
- Use readonly for immutable: `readonly items: string[]`
- Explicit return types on public functions

### Naming Conventions
- **Files**: kebab-case (`user-profile.ts`, `api-utils.ts`)
- **Variables/Functions**: camelCase (`getUserData`, `isValidEmail`)
- **Classes**: PascalCase (`UserService`, `ApiClient`)
- **Constants**: UPPER_SNAKE_CASE (`MAX_RETRIES`, `API_BASE_URL`)
- **Private members**: underscore prefix (`_internalMethod`)
- **Booleans**: prefix with is/has/can (`isLoading`, `hasPermission`)
- **Components**: PascalCase (`UserProfile`, `DataTable`)

### Error Handling
- Always handle errors from async operations with try/catch
- Use specific error types when possible
- Log errors with context: `logger.error('Ошибка получения пользователя', { userId, error })`
- Use early returns for error conditions
- Re-throw errors with additional context
- Never swallow errors silently

### Code Organization
- One logical concern per file (max ~600 lines)
- Export named functions/classes, use default export sparingly
- Use index files for barrel exports in directories
- Extract reusable logic into utility functions
- Follow BMad recommendations for project structure

### Comments & Documentation
- Write self-documenting code
- **Комментарии на русском языке** для функций, классов, сложной логики
- Use JSDoc: `/** Создает нового пользователя */`
- Документация и README на русском языке
- Keep comments updated with code changes
- Avoid TODO comments - create issues instead

## Performance
- Avoid premature optimization
- Use memoization for expensive computations
- Lazy load heavy components and modules
- Debounce/throttle expensive operations
- Use efficient data structures (Set for lookups, Map for key-value)
- Optimize images and assets

## Security
- Никогда не коммитьте секреты (.env, credentials.json, API keys)
- Используйте переменные окружения для конфиденциальных данных
- Проверяйте зависимости на уязвимости
- Следуйте OWASP Best Practices

## Project Cleanliness
- Удаляйте временные файлы после работы
- Используйте .gitignore для исключения лишних файлов
- Не создавайте файлы в корне проекта без необходимости
- Следите за порядком в директориях, регулярно чистите от мусора

## Pre-commit Checklist

- [ ] Код проходит линтинг и типизацию
- [ ] Тесты проходят успешно
- [ ] Документация обновлена
- [ ] Commit message на русском, следует конвенции
- [ ] Файлы в правильных папках
- [ ] Нет лишних файлов в корне проекта
- [ ] Нет секретов в коде

## Project Structure Guidelines

Рекомендуемая структура (адаптировать под рекомендации BMad):

```
/
├── src/                  # Исходный код приложения
├── tests/                # Тесты (unit/integration/e2e)
├── docs/                 # Документация для разработчиков
├── config/               # Конфигурационные файлы
├── AGENTS.md             # Этот файл
└── README.md             # Описание проекта на русском
```

При инициализации через агентов BMad следуйте их рекомендациям по структуре проекта.
