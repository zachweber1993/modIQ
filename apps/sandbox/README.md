# modIQ Developer Sandbox

A Tauri 2 + React + TypeScript + Vite application used to visualize and interact with the Rust engine during development.

**This is not production software.** It is a developer sandbox, independent of the main modIQ engineering workspace (`crates/*`).

## Architecture

- The frontend communicates with Rust exclusively through Tauri commands (`invoke`). There is no HTTP server, REST API, or networking of any kind.
- `src-tauri` is its own Cargo workspace (see the `[workspace]` table in `src-tauri/Cargo.toml`). It is intentionally **not** a member of the root modIQ workspace, and does not currently depend on `modiq-runtime` or any other engineering crate.
- This application does not implement Runtime logic itself, and does not duplicate any engineering crate's behavior. Phase 1 registers a single command (`ping_runtime`) returning a fixed string, solely to prove the Tauri command bridge works end to end.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Development

```sh
npm install
npm run tauri dev
```
