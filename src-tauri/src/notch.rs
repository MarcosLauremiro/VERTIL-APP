//! A janela do entalho é maior que o entalho em si e transparente.
//! Para não bloquear cliques no que está atrás, ela ignora o mouse fora da
//! "região clicável" que a interface informa. Um vigia lê a posição do cursor
//! e liga ou desliga isso, avisando a interface com o evento `notch://hover`.

use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use serde::Deserialize;
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, State, WebviewWindow};

const INTERVALO_VIGIA: Duration = Duration::from_millis(40);

/// Retângulo em pixels lógicos (CSS), relativo ao canto da janela.
#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Region {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Region {
    fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

#[derive(Default)]
pub struct HitRegion(pub Mutex<Region>);

#[tauri::command]
pub fn set_hit_region(region: Region, state: State<'_, HitRegion>) {
    if let Ok(mut atual) = state.0.lock() {
        *atual = region;
    }
}

/// Centraliza a janela no topo do monitor principal.
pub fn place_top_center(window: &WebviewWindow) -> tauri::Result<()> {
    let monitor = window.primary_monitor()?.or(window.current_monitor()?);
    if let Some(monitor) = monitor {
        let tamanho = window.outer_size()?;
        let x = monitor.position().x + (monitor.size().width as i32 - tamanho.width as i32) / 2;
        let y = monitor.position().y;
        window.set_position(PhysicalPosition::new(x, y))?;
    }
    Ok(())
}

/// Mostra o entalho aberto e com o campo de texto em foco (atalho global ou bandeja).
pub fn summon(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(crate::NOTCH_LABEL) {
        let _ = window.show();
        let _ = window.set_ignore_cursor_events(false);
        let _ = window.set_focus();
        let _ = window.emit("notch://summon", ());
    }
}

pub fn spawn_hover_watcher(app: AppHandle) {
    thread::spawn(move || {
        let mut estava_dentro: Option<bool> = None;
        loop {
            thread::sleep(INTERVALO_VIGIA);
            let Some(window) = app.get_webview_window(crate::NOTCH_LABEL) else {
                continue;
            };
            let regiao = app
                .state::<HitRegion>()
                .0
                .lock()
                .map(|r| *r)
                .unwrap_or_default();

            let dentro = match (
                window.cursor_position(),
                window.outer_position(),
                window.scale_factor(),
            ) {
                (Ok(cursor), Ok(janela), Ok(escala)) => {
                    let x = (cursor.x - janela.x as f64) / escala;
                    let y = (cursor.y - janela.y as f64) / escala;
                    regiao.contains(x, y)
                }
                // Sem leitura do cursor (ex.: alguns ambientes Wayland), a janela fica clicável.
                _ => true,
            };

            if estava_dentro != Some(dentro) {
                let _ = window.set_ignore_cursor_events(!dentro);
                let _ = window.emit("notch://hover", dentro);
                estava_dentro = Some(dentro);
            }
        }
    });
}
