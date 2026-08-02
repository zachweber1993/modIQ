//! modIQ Console — the production interaction layer `FrontendArchitecture.md`
//! authorizes. This crate owns Tauri application bootstrap and the
//! Request/Response Mechanism connecting it to the engine.
//!
//! `assessment` is the only module permitted to depend on a
//! `modiq-*` crate (see its own doc comment and this crate's
//! `Cargo.toml`). Nothing in this file can evaluate Evidence,
//! generate a Finding, or reach into Assessment state — it only
//! wires the one command `assessment` exposes into Tauri's own
//! dispatch. That command's return type is a provisional
//! summary, not final or stable — see `assessment.rs`.

mod assessment;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![assessment::submit_assessment])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
