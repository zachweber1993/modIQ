# Fixture: `<fixture-id>`

> Copy this file into a new fixture directory as `README.md` and fill in every field. Do not leave **Source**, **Consent / Licensing**, or **Normalization Applied** blank — these three exist to protect the project and the people whose data may appear in a real log, and are the fields most likely to be skipped under time pressure.

| Property | Value |
|---|---|
| **Fixture ID** | `<fixture-id>` — matches the directory name exactly |
| **Status** | Awaiting capture / Captured / Superseded |
| **Farming Simulator Version** | Exact version/build/patch number, if known — not just "FS25" |
| **Platform** | The operating system the log was captured on (Windows / macOS / Linux). Farming Simulator's own log output may differ by platform; this field is load-bearing, not decorative. |
| **Map** | The in-game map active during capture, or "N/A" |
| **Installed Mods (global)** | The complete, verified contents of the game's global mods directory at capture time — name and version each, or "Verified empty." **Do not infer this from savegame state.** A runtime log has been directly observed to enumerate installed mods from the global mods directory regardless of which savegame is active or whether that savegame itself uses any of them — creating a new savegame does not clear or otherwise affect this list. This field must reflect what was actually checked in the mods directory at capture time, not an assumption. |
| **Savegame Mod State** | Whatever the specific savegame itself records as active/used, which may be a subset of, equal to, or (for a freshly created savegame) disjoint from **Installed Mods (global)** above. State plainly whether the savegame is new/fresh or an existing one, and which installed mods (if any) it actually uses. |
| **Purpose** | What this fixture exists to prove or test. One or two sentences, stated as a claim someone could disagree with, not a restatement of the fixture's name. |
| **Expected Observable Behavior** | What a human reviewer — and, eventually, a Rule — should conclude this log's content shows. Stated as an observation, not an inference beyond what the log literally contains. |
| **Source** | Who captured this log and how (e.g., "Captured directly by modIQ engineering on a controlled test install, `<date>`"). Never leave this generic. |
| **Consent / Licensing** | An explicit statement that this content is safe and permitted to store in this repository. If the log did not originate from modIQ's own controlled capture, name the permission obtained. |
| **Captured** | Date of capture (or acquisition, if not captured directly) |
| **Normalization Applied** | Exactly what was normalized and how, described precisely enough that re-applying it to the same raw source would reproduce this fixture byte for byte — the exact pattern matched (e.g., a path prefix, or the specific line(s) it appears on), the exact placeholder used, and the exact number of substitutions (e.g., "Every `/Users/<username>/` path prefix replaced with the fixed token `/Users/REDACTED_USER/`, 2 occurrences, lines 122 and 1443"), or "None — no personally identifying or machine-specific content was present in the raw log." Vague descriptions ("a username was redacted") are not sufficient — normalization must be deterministic, and this field is what makes that verifiable. Normalization is strictly substitutive: it must never alter parser-relevant content, runtime semantics, ordering, line counts, or formatting beyond the exact documented substitution — see `README.md`'s own "Runtime Log Normalization" section. Must happen before the file is ever committed; git history is not a safe place to fix this afterward. |
| **Format Notes** | Encoding, line-ending convention, any truncation applied, and why |
| **Known Limitations** | What a future reader should not over-generalize from this single fixture — e.g., "one sample of one failure class; does not establish this is the only way this failure appears" |
| **File(s)** | The raw log file name(s) present in this directory |

## Notes

Free-text elaboration, if useful — e.g., why this fixture was prioritized, what makes it a clean/minimal example of its class, or anything a future engineer should know before relying on it.
