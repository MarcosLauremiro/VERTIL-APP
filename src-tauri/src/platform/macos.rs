use tauri::{ActivationPolicy, WebviewWindow};
use tauri_plugin_global_shortcut::Modifiers;

pub fn summon_modifiers() -> Modifiers {
    Modifiers::SUPER | Modifiers::SHIFT
}

pub fn prepare_app(app: &mut tauri::App) {
    // Sem ícone no Dock: o Vertil vive no topo da tela e na barra de menus.
    app.set_activation_policy(ActivationPolicy::Accessory);
}

pub fn prepare_notch_window(window: &WebviewWindow) {
    // Aparece em todas as mesas (Spaces).
    let _ = window.set_visible_on_all_workspaces(true);
    // Pendente: subir o nível da janela acima da barra de menus para encostar no entalho
    // físico do MacBook (NSWindow level = status bar). Fica para a Fase 6.
}
