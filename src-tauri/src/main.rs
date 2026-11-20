// Esconde o console extra no Windows em builds de produção.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    vertil_app_lib::run()
}
