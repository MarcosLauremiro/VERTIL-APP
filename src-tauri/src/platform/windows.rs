use tauri::WebviewWindow;
use tauri_plugin_global_shortcut::Modifiers;

pub fn summon_modifiers() -> Modifiers {
    Modifiers::CONTROL | Modifiers::SHIFT
}

pub fn prepare_app(_app: &mut tauri::App) {}

pub fn prepare_notch_window(_window: &WebviewWindow) {
    // Fase 3: aqui entra a técnica da janela WorkerW para o wallpaper atrás dos ícones.
}

/// Abre um app pelo nome com `start`. O nome já chega validado (sem & | < > etc.).
pub fn abrir_app(nome: &str) -> Result<(), String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    std::process::Command::new("cmd")
        .args(["/C", "start", "", nome])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("Não consegui abrir {nome}: {e}"))
}
