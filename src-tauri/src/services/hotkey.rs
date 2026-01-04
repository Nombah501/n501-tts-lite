use tauri::AppHandle;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

use crate::models::AppError;

pub fn parse_record_hotkey(input: &str) -> Result<Shortcut, AppError> {
    let normalized = input.trim().to_lowercase();
    if normalized.is_empty() {
        return Err(AppError::new("HOTKEY", "Пустая горячая клавиша"));
    }

    let mut modifiers = Modifiers::empty();
    let mut key_part: Option<&str> = None;

    for part in normalized.split(|ch| ch == '+' || ch == '-') {
        if part.is_empty() {
            continue;
        }

        match part {
            "ctrl" | "control" => {
                modifiers |= Modifiers::CONTROL;
            }
            "shift" => {
                modifiers |= Modifiers::SHIFT;
            }
            "alt" | "option" => {
                modifiers |= Modifiers::ALT;
            }
            "cmd" | "command" | "meta" | "super" => {
                modifiers |= Modifiers::SUPER;
            }
            _ => {
                key_part = Some(part);
            }
        }
    }

    let key = key_part.ok_or_else(|| AppError::new("HOTKEY", "Не задана клавиша"))?;
    let code = parse_key_code(key)?;

    Ok(Shortcut::new(Some(modifiers), code))
}

pub fn rebind_record_hotkey(app_handle: &AppHandle, shortcut: Shortcut) -> Result<(), AppError> {
    app_handle
        .global_shortcut()
        .unregister_all()
        .map_err(|error| AppError::new("HOTKEY_UNREGISTER", error.to_string()))?;

    app_handle
        .global_shortcut()
        .register(shortcut)
        .map_err(|error| AppError::new("HOTKEY_REGISTER", error.to_string()))?;

    Ok(())
}

fn parse_key_code(key: &str) -> Result<Code, AppError> {
    let code = match key {
        "space" | "spacebar" => Code::Space,
        "a" => Code::KeyA,
        "b" => Code::KeyB,
        "c" => Code::KeyC,
        "d" => Code::KeyD,
        "e" => Code::KeyE,
        "f" => Code::KeyF,
        "g" => Code::KeyG,
        "h" => Code::KeyH,
        "i" => Code::KeyI,
        "j" => Code::KeyJ,
        "k" => Code::KeyK,
        "l" => Code::KeyL,
        "m" => Code::KeyM,
        "n" => Code::KeyN,
        "o" => Code::KeyO,
        "p" => Code::KeyP,
        "q" => Code::KeyQ,
        "r" => Code::KeyR,
        "s" => Code::KeyS,
        "t" => Code::KeyT,
        "u" => Code::KeyU,
        "v" => Code::KeyV,
        "w" => Code::KeyW,
        "x" => Code::KeyX,
        "y" => Code::KeyY,
        "z" => Code::KeyZ,
        _ => {
            return Err(AppError::new(
                "HOTKEY",
                format!("Неподдерживаемая клавиша: {key}"),
            ))
        }
    };

    Ok(code)
}
