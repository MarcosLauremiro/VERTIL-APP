use tauri::WebviewWindow;
use tauri_plugin_global_shortcut::Modifiers;

pub fn summon_modifiers() -> Modifiers {
    Modifiers::CONTROL | Modifiers::SHIFT
}

pub fn prepare_app(_app: &mut tauri::App) {}

pub fn prepare_notch_window(_window: &WebviewWindow) {
    // Funciona no X11. No Wayland, posição da janela, atalhos globais e leitura do
    // cursor dependem do ambiente (KDE, GNOME, Hyprland). Tratar na Fase 6.
}
