#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![cfg_attr(
    all(not(debug_assertions), target_os = "macos"),
    allow(unsafe_code = "std", arch_powerpc)
)]

fn main() {
    tauri_builder::run()
}
