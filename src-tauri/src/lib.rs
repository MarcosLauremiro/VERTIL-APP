mod notch;
mod platform;

use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Shortcut, ShortcutState};
use tauri_plugin_sql::{Migration, MigrationKind};

/// Rótulo da janela principal (definida em tauri.conf.json).
pub const NOTCH_LABEL: &str = "notch";

/// Migrações do banco local. Nunca edite uma migração já lançada: crie uma nova versão.
fn migrations() -> Vec<Migration> {
    vec![Migration {
        version: 1,
        description: "criar_notas",
        sql: "CREATE TABLE IF NOT EXISTS notas (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                texto TEXT NOT NULL,
                criado_em TEXT NOT NULL DEFAULT (datetime('now'))
              );",
        kind: MigrationKind::Up,
    }]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Ctrl+Shift+Espaço (Windows/Linux) ou Cmd+Shift+Espaço (Mac) chama o Vertil.
    let summon_shortcut = Shortcut::new(Some(platform::summon_modifiers()), Code::Space);

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:vertil.db", migrations())
                .build(),
        )
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if shortcut == &summon_shortcut && event.state() == ShortcutState::Pressed {
                        notch::summon(app);
                    }
                })
                .build(),
        )
        .manage(notch::HitRegion::default())
        .invoke_handler(tauri::generate_handler![notch::set_hit_region])
        .setup(move |app| {
            platform::prepare_app(app);

            let window = app
                .get_webview_window(NOTCH_LABEL)
                .expect("a janela 'notch' precisa existir em tauri.conf.json");
            notch::place_top_center(&window)?;
            platform::prepare_notch_window(&window);
            window.show()?;

            notch::spawn_hover_watcher(app.handle().clone());

            if let Err(erro) = app.global_shortcut().register(summon_shortcut) {
                // Outro app pode estar usando o mesmo atalho; o Vertil continua funcionando.
                eprintln!("Não foi possível registrar o atalho global: {erro}");
            }

            build_tray(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("erro ao iniciar o Vertil App");
}

/// Ícone na bandeja: como a janela não tem bordas, é por aqui que o usuário sai do app.
fn build_tray(app: &tauri::App) -> tauri::Result<()> {
    let mostrar = MenuItem::with_id(app, "mostrar", "Abrir o Vertil", true, None::<&str>)?;
    let sair = MenuItem::with_id(app, "sair", "Sair do Vertil", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&mostrar, &sair])?;

    let mut tray = TrayIconBuilder::with_id("vertil")
        .tooltip("Vertil App")
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "mostrar" => notch::summon(app),
            "sair" => app.exit(0),
            _ => {}
        });

    if let Some(icone) = app.default_window_icon() {
        tray = tray.icon(icone.clone());
    }
    tray.build(app)?;
    Ok(())
}
