# Fixture: `clean-base-game`

| Property | Value |
|---|---|
| **Fixture ID** | `clean-base-game` |
| **Status** | **Awaiting capture** — no log file present in this directory |
| **Farming Simulator Version** | Pending capture — record the exact version/build/patch number at capture time |
| **Platform** | Pending capture |
| **Map** | The game's own default/official map, no modded map installed |
| **Enabled Mods** | None (clean base game) — this fixture's entire purpose depends on zero mods being active |
| **Purpose** | Establish ground truth for what a normal, successful session's log output looks like with no mod-related content whatsoever — the platform's baseline for "nothing is wrong." Without this, there is no way to know what a Rule must *not* flag. |
| **Expected Observable Behavior** | The log should show a normal load and session with no mod-related errors, warnings, or failure signatures of any kind. Any Runtime Log Interpretation Rule must produce no Finding against this fixture. |
| **Source** | Pending capture |
| **Consent / Licensing** | Pending capture — expected to be modIQ engineering's own controlled capture, requiring no third-party permission |
| **Captured** | Pending |
| **Redaction Applied** | Pending capture — to be assessed once the real log is in hand |
| **Format Notes** | Pending capture |
| **Known Limitations** | A single sample of "no mods installed." Does not by itself establish that every possible clean-base-game session log looks identical — later fixtures or repeated captures may be warranted if the log format proves session-dependent. |
| **File(s)** | None yet |

## Notes

**Acquisition priority: 1st of 3.** Establishing what "no failure present" looks like, in the simplest possible case (zero mods), is the precondition every other fixture in this corpus builds on. Captured first, before `single-compatible-mod` and `single-incompatible-mod`, so both later fixtures can be read against an already-established baseline rather than in isolation.
