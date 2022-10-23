use tauri::Theme;

#[tauri::command]
fn get_theme(window: tauri::Window) -> String {
    match window.theme() {
        Ok(Theme::Dark) => "dark".into(),
        Ok(Theme::Light) => "light".into(),
        _ => "".into(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_theme])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
