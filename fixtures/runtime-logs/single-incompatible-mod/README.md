# Fixture: `single-incompatible-mod`

| Property | Value |
|---|---|
| **Fixture ID** | `single-incompatible-mod` |
| **Status** | **Awaiting capture** — no log file present in this directory |
| **Farming Simulator Version** | Pending capture |
| **Platform** | Pending capture |
| **Map** | The game's own default/official map — kept identical to `clean-base-game` and `single-compatible-mod` so the mod itself is the only variable |
| **Installed Mods (global)** | **Must contain exactly the one target mod — verified directly, not inferred from the savegame.** Per the finding recorded in `clean-base-game`'s own README, the runtime log enumerates the global mods directory regardless of savegame; any other mod left in that directory will contaminate this fixture's "single mod" isolation even if the savegame itself never uses it. |
| **Savegame Mod State** | A savegame that actually uses the one target mod, deliberately chosen or constructed to fail to load (e.g., a declared-version mismatch, a missing dependency, or a scripting fault a real Farming Simulator session actually surfaces) |
| **Purpose** | **The single most important fixture in this initial corpus, by content — though third in acquisition order (see Notes).** This is the platform's first real, concrete example of what a genuine mod-failed-to-load signature actually looks like in Farming Simulator's own log output — the one fact this platform cannot currently derive from documentation, assumption, or any existing fixture. Every other fixture in this corpus exists to make this one safe to act on; this one is what Sprint 10's capability exists to recognize. |
| **Expected Observable Behavior** | The log should show a specific, identifiable failure signature attributable to the single enabled mod — the concrete, real-world shape of "this mod failed to load," to be transcribed into Architectural Resolution's own design once captured, not assumed in advance. |
| **Source** | Pending capture |
| **Consent / Licensing** | Pending capture — expected to be modIQ engineering's own controlled capture, using either a deliberately malformed test mod or a real community mod with a documented, reproducible incompatibility |
| **Captured** | Pending |
| **Normalization Applied** | Pending capture |
| **Format Notes** | Pending capture |
| **Known Limitations** | One sample of one failure class. This fixture alone cannot establish that all mod-load failures share one common signature — broader signature coverage is explicitly deferred future work (`SPRINT10_CAPABILITY_DEFINITION.md`, Section 9), not assumed here. |
| **File(s)** | None yet |

## Notes

**Acquisition priority: 3rd of 3, deliberately — not because it matters least, but because it is read most safely last.** Capture `clean-base-game` and `single-compatible-mod` first, so this fixture is examined with both a no-mod baseline and a normal-mod baseline already in hand. Reading this log first, with nothing to compare it against, risks misattributing ordinary mod-loading noise — unfamiliar simply because no compatible-mod log has been seen yet — to the actual failure signature. Acquiring it remains the critical-path engineering activity Architectural Resolution must complete before finalizing any signal-recognition design, per `SPRINT10_CAPABILITY_DEFINITION.md`, Section 11's own strengthened precondition — but it should be the third capture, not the first. No plausible-sounding assumption about what a Farming Simulator failure log "probably" contains may substitute for this fixture actually existing.

**Before capturing:** verify the global mods directory contains only the intended mod, for the same reason recorded in `clean-base-game`'s own README — a new savegame does not clear or otherwise affect what the log enumerates from the global mods directory.
