use tauri::WebviewWindow;
use tauri_plugin_global_shortcut::Modifiers;

pub fn summon_modifiers() -> Modifiers {
    Modifiers::CONTROL | Modifiers::SHIFT
}

pub fn prepare_app(_app: &mut tauri::App) {}

pub fn prepare_notch_window(_window: &WebviewWindow) {
    // Fase 3: aqui entra a técnica da janela WorkerW para o wallpaper atrás dos ícones.
}
