// Desktop build: no console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Tauri entry point for the dozenal calculator GUI (PLAN.md Phase 6).
//!
//! This binary owns the window and the session; every calculation goes
//! through `dz-api`, the same entry point the CLI uses. There is no
//! dozenal arithmetic in this crate at all -- deliberately, so the GUI
//! cannot drift from the terminal interface.

mod commands;
mod research;
mod state;

fn main() {
    tauri::Builder::default()
        // Phase 7: the research panel saves CSV through the native
        // "Save as" dialog. `plugin-fs` carries the filesystem capability
        // the chosen path is registered against; the write itself happens
        // in `research::export_csv`, so the CSV format has exactly one
        // authority (Rust) on both sides of the dialog.
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(state::AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::calc,
            commands::set_backend,
            commands::set_mode,
            commands::set_scale,
            commands::set_input_base,
            commands::chain_operand,
            commands::get_session,
            commands::clear_history,
            commands::memory_store,
            commands::memory_clear,
            commands::set_alphabet,
            commands::reset_alphabet,
            commands::list_functions,
            research::trace_operation,
            research::compare_backends,
            research::export_csv,
        ])
        .run(tauri::generate_context!())
        .expect("error while running the dozenal calculator");
}
