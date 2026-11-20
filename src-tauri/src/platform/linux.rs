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

/// Tenta abrir pelo atalho .desktop (gtk-launch) e, se falhar, pelo nome do executável.
pub fn abrir_app(nome: &str) -> Result<(), String> {
    let id = nome.to_lowercase().replace(' ', "-");
    let pelo_atalho = std::process::Command::new("gtk-launch")
        .arg(&id)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if pelo_atalho {
        return Ok(());
    }
    std::process::Command::new(&id)
        .spawn()
        .map(|_| ())
        .map_err(|_| format!("Não encontrei o app {nome}."))
}
