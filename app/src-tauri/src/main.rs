// Desktop build: no console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Tauri entry point for the dozenal calculator GUI (PLAN.md Phase 6).
//!
//! This binary owns the window and the session; every calculation goes
//! through `dz-api`, the same entry point the CLI uses. There is no
//! dozenal arithmetic in this crate at all -- deliberately, so the GUI
//! cannot drift from the terminal interface.

mod commands;
mod state;

fn main() {
    tauri::Builder::default()
        .manage(state::AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::calc,
            commands::set_backend,
            commands::set_mode,
            commands::set_scale,
            commands::get_session,
            commands::clear_history,
            commands::memory_store,
            commands::memory_clear,
            commands::list_functions,
        ])
        .run(tauri::generate_context!())
        .expect("error while running the dozenal calculator");
}
