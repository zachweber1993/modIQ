# Fixture: `<fixture-id>`

> Copy this file into a new fixture directory as `README.md` and fill in every field. Do not leave **Source**, **Consent / Licensing**, or **Redaction Applied** blank — these three exist to protect the project and the people whose data may appear in a real log, and are the fields most likely to be skipped under time pressure.

| Property | Value |
|---|---|
| **Fixture ID** | `<fixture-id>` — matches the directory name exactly |
| **Status** | Awaiting capture / Captured / Superseded |
| **Farming Simulator Version** | Exact version/build/patch number, if known — not just "FS25" |
| **Platform** | The operating system the log was captured on (Windows / macOS / Linux). Farming Simulator's own log output may differ by platform; this field is load-bearing, not decorative. |
| **Map** | The in-game map active during capture, or "N/A" |
| **Enabled Mods** | The complete list of active mods, name and version each, exactly as active during capture — or "None (clean base game)" |
| **Purpose** | What this fixture exists to prove or test. One or two sentences, stated as a claim someone could disagree with, not a restatement of the fixture's name. |
| **Expected Observable Behavior** | What a human reviewer — and, eventually, a Rule — should conclude this log's content shows. Stated as an observation, not an inference beyond what the log literally contains. |
| **Source** | Who captured this log and how (e.g., "Captured directly by modIQ engineering on a controlled test install, `<date>`"). Never leave this generic. |
| **Consent / Licensing** | An explicit statement that this content is safe and permitted to store in this repository. If the log did not originate from modIQ's own controlled capture, name the permission obtained. |
| **Captured** | Date of capture (or acquisition, if not captured directly) |
| **Redaction Applied** | Exactly what was redacted and how (e.g., "Windows username in file paths replaced with the fixed token `REDACTED_USER`"), or "None — no personally identifying content was present in the raw log." Redaction must happen before the file is ever committed; git history is not a safe place to fix this afterward. |
| **Format Notes** | Encoding, line-ending convention, any truncation applied, and why |
| **Known Limitations** | What a future reader should not over-generalize from this single fixture — e.g., "one sample of one failure class; does not establish this is the only way this failure appears" |
| **File(s)** | The raw log file name(s) present in this directory |

## Notes

Free-text elaboration, if useful — e.g., why this fixture was prioritized, what makes it a clean/minimal example of its class, or anything a future engineer should know before relying on it.
