//! modIQ Console — the production interaction layer `FrontendArchitecture.md`
//! authorizes. This crate owns Tauri application bootstrap only.
//!
//! Phase 1 (Sprint 21) intentionally has no dependency on any
//! `modiq-*` crate: nothing in this file can evaluate Evidence,
//! generate a Finding, or reach into Assessment state, because
//! nothing here has ever been given the means to. The Request/Response
//! Mechanism connecting this application to the engine is Phase 2's
//! own, separate work — see this crate's own `Cargo.toml`.

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
