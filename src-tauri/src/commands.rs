#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("¡Hola, {}! Has sido saludado desde Rust!", name)
}
