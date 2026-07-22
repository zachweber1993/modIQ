# Fixture: `single-compatible-mod`

| Property | Value |
|---|---|
| **Fixture ID** | `single-compatible-mod` |
| **Status** | **Awaiting capture** — no log file present in this directory |
| **Farming Simulator Version** | Pending capture |
| **Platform** | Pending capture |
| **Map** | The game's own default/official map — kept identical to `clean-base-game` and `single-incompatible-mod` so the mod itself is the only variable |
| **Enabled Mods** | Exactly one mod, known to be fully compatible and to load successfully |
| **Purpose** | The negative control for the positive signal `single-incompatible-mod` establishes. Confirms that ordinary, successful mod-loading activity in the log — which may itself be verbose, or contain the word "warning" in a benign context — is not mistakenly recognized as a failure signature. A Rule that cannot correctly stay silent here is not safe to ship, regardless of how well it recognizes genuine failures. |
| **Expected Observable Behavior** | The log should show the mod loading and running successfully, with no failure signature of the kind `single-incompatible-mod` is expected to show. Any Runtime Log Interpretation Rule must produce no Finding against this fixture. |
| **Source** | Pending capture |
| **Consent / Licensing** | Pending capture — expected to be modIQ engineering's own controlled capture, using a mod with clearly documented, unambiguous compatibility |
| **Captured** | Pending |
| **Redaction Applied** | Pending capture |
| **Format Notes** | Pending capture |
| **Known Limitations** | One sample of one successful-load case. Does not by itself establish that every compatible mod's log output is free of anything superficially resembling a failure signature — a real risk this fixture exists specifically to surface early. |
| **File(s)** | None yet |

## Notes

**Acquisition priority: 2nd of 3 — captured before `single-incompatible-mod`, deliberately.** This ordering establishes what ordinary, successful mod-loading log output looks like before the failure case is ever examined, so that fixture can be read against a real contrastive baseline rather than in isolation (see `single-incompatible-mod`'s own Notes). Should be captured using a mod as structurally different as reasonably possible from whatever mod ultimately produces `single-incompatible-mod` (different scripting footprint, different dependency shape), so that any resemblance between the two fixtures' log output points toward the actual failure signature, not an artifact of the two mods happening to be similar.
