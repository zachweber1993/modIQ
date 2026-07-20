# Proposal: Filesystem Collection

**Stage:** Proposal (Architecture Review)

**Prepared by:** Engineering, for Technical Director review

**Status:** Design-only. No Rust code, no documentation changes outside this file, no governance changes, no ADRs, no implementation.

---

# 1. Architectural Objective

The Evidence Collection subsystem boundary (ADR-0008) has been proven with a deterministic, synthetic collector: it takes an opaque descriptor, produces one fixed Evidence item, and never touches the outside world. That proved the *boundary* — ownership, orchestration, dependency direction — but it proved nothing about what happens when Collection actually has to look at something real.

The purpose of this milestone is to design the first collector that inspects the real filesystem: given a real location on disk, discover what is structurally present there — files and directories — without reading, parsing, or interpreting the content of anything it finds.

**Why this is the correct next milestone.** Per `ROADMAP_REVIEW_2026.md`, this is the smallest possible step past today's synthetic collector, and it is the one concrete forcing function two already-open governance items (GOV-009, Input Descriptor ownership; GOV-010, Collection Error Model) actually need. Both were deliberately left unresolved because designing them in the abstract, without a real collector to test them against, risked exactly the premature-abstraction failure mode this project has repeatedly and correctly avoided (the Rule trait question, most recently). Nothing else on the near-term roadmap — ZIP traversal, XML inspection, Rule abstraction, Knowledge integration, Version Profiles — has its own prerequisites met yet; this milestone is what unblocks several of them at once.

**Architectural questions this milestone answers:**
- What does a real, non-placeholder Assessment Input actually need to represent, for the simplest possible real case (something already sitting on the local filesystem)?
- What does a genuine collection *failure* look like, architecturally, when the synthetic collector never had one to represent?
- How is determinism actually preserved when the thing being observed is a real filesystem, which offers no ordering guarantees on its own?
- How much of "the real world" does the Sandbox need to expose, versus how much stays entirely inside Collection and the Engine?

**What this milestone intentionally does NOT attempt to solve:**
- It does not read or parse the content of any file — not ZIP, not XML, not Lua, not plain text.
- It does not decide whether a second Rule or a Rule abstraction is now justified.
- It does not touch Knowledge Domain integration or Version Profiles.
- It does not decide persistent storage.
- It does not finalize the Input Descriptor's shape for every future collector type (network sources, in-memory archives, etc.) — only its shape for this filesystem-only case. GOV-009 may need to revisit this again once non-filesystem sources are considered.
- It does not design any Sandbox UI element (file picker, drag-and-drop, progress indicator).

---

# 2. Assessment Input

At the architectural level, a valid Assessment Input for this milestone is a reference to something that already exists on the local filesystem, at the moment collection is attempted. Two shapes are both legitimately valid, and the design must accommodate either without assuming one:

- **A single file.** Farming Simulator mods are frequently distributed as one archive file (a `.zip`). Pointing an Assessment directly at such a file — without it having been extracted anywhere — is a real, common case, not an edge case.
- **A directory.** An already-unpacked or extracted mod, sitting as a folder of files, is equally real and common, particularly for local development or a mod already installed into a game's mod folder.

Several conceptual properties of "a location on the filesystem" matter architecturally, independent of any implementation:

- **Absolute vs. relative.** A relative path's meaning depends on an external, ambient factor (the current working directory of whatever process resolves it) that is itself not part of the Assessment's own recorded state. For collection to be meaningfully repeatable, the architecture should treat the input as needing to resolve to one unambiguous location, not one that depends on unrecorded execution context.
- **Existence at the time of collection.** A location either exists when collection is attempted, or it doesn't — and unlike everything built so far, this is not a fact Collection can guarantee or control. This is an irreducible property of any collector that touches real, external state, and is fundamentally different from the synthetic collector's unconditional determinism (see Section 4).
- **Symbolic links.** A path may not point directly at a file or directory but at a link to one, possibly one outside the location the caller intended to reference. This is a real architectural question — not resolved here — with both a correctness dimension (does following a link still count as inspecting "the input") and a safety dimension (should collection ever be allowed to leave the boundary of the location it was given).

This proposal does not prescribe a Rust type, an API shape, or a serialization format for representing this input. It only establishes that "a real filesystem location, either a file or a directory, existing at the time of collection" is the conceptual shape GOV-009 needs to design around for this milestone.

---

# 3. Collection Scope

## In Scope

- **Deterministic filesystem traversal.** Walking a directory's contents (and, where the input is itself a directory tree, its subdirectories) in a way that produces the same result every time, for unchanged input.
- **Directory discovery.** Recording which subdirectories are structurally present within the given input.
- **File discovery.** Recording which files are structurally present within the given input — their existence and location, nothing about their content.
- **Input validation.** Confirming the given location is well-formed and reachable before any traversal is attempted.

Everything in scope is *structural observation* — the same conceptual category `Glossary.md` already names as "File structure analysis," an existing Evidence example. This milestone does not introduce a new kind of Evidence; it produces the platform's first real instance of a category that has existed in the domain model since Documentation Release 1.0.

## Out of Scope

- **ZIP parsing.** A discovered file may have a `.zip`-shaped name, but *opening* it and looking at what's inside is a distinct, more complex capability — Evidence about "a file exists" is different from Evidence about "what an archive contains." This is explicitly the next milestone after this one, not part of it.
- **XML parsing.** Discovering that a file named `modDesc.xml` exists at a given path is in scope (file discovery). Reading or parsing what's inside it is not.
- **Lua inspection.** Entirely out of scope; not adjacent to this milestone in any way.
- **Semantic evaluation.** Judging whether a discovered structure is correct, complete, or sufficient for a valid mod is Rule Engine's responsibility, not Collection's — this is the same producer/consumer boundary `EvidenceCollection.md` already established, restated here in its concrete filesystem form: "this directory contains N files" is Evidence; "this directory is missing a required file" is a Finding, produced later, by the Rule Engine evaluating that Evidence.
- **Rule execution.** Not part of Collection under any circumstance; already a separately owned subsystem.
- **Reading file contents, at all, even as raw bytes.** This milestone needs to know that something exists, not what it contains. Excluding content-reading entirely keeps this milestone's risk surface to filesystem metadata only — no memory concerns for large files, no encoding concerns, no format-specific parsing of any kind.
- **Traversing beyond the given input's boundary.** A collector should never wander into unrelated parts of the filesystem outside the location it was actually given, including via symbolic links that point elsewhere (see Section 2) — this is a scope boundary, not just a determinism concern.

---

# 4. Determinism

A real filesystem offers no inherent ordering guarantee: directory listings can be returned in whatever order the underlying storage happens to hold them, which can vary across operating systems, filesystems, and even between runs on the same machine. The synthetic collector never had to confront this, because it never asked the outside world anything. This milestone's design must confront it directly, and resolve it above the level of "whatever the OS happens to return":

- **Traversal ordering.** Collection must impose its own explicit, defined ordering over whatever the underlying filesystem API returns — for example, a consistent lexicographic ordering of discovered paths — so that Evidence describing the same, unchanged structure is always produced in the same sequence. This is a requirement on the architecture, not a specific sorting algorithm to implement.
- **Repeatability.** Given the same filesystem content, inspected twice, collection must produce Evidence with identical content both times — same categories, same descriptions, same structural facts. This is achievable only if the collector limits itself to facts that are themselves stable across repeat observation. Concretely: recording that a file *exists* is stable; recording its last-modified timestamp, access time, or physical storage details is not, since those can change or vary without the structure itself having meaningfully changed. This milestone's scope (Section 3) should exclude any such point-in-time or storage-level fact for exactly this reason.
- **Stable outputs, not stable identity.** As with every Runtime entity in the platform (ADR-0005, ADR-0007), each Evidence item's own identity is freshly assigned per collection run and was never part of any determinism guarantee. What must be stable is *content* — the same structural facts, described the same way, in the same order — not the identifiers attached to them.

**An honest limitation, not a defect.** Unlike the synthetic collector, which is unconditionally deterministic because it depends on nothing external, a real filesystem collector's determinism is conditional: it holds only if the filesystem itself does not change between two collection attempts. This is an irreducible property of any collector that observes real, external state, and should be documented as such rather than treated as a gap to be engineered away. `EvidenceCollection.md`'s Collector Contract already anticipates this class of concern under Determinism Expectations; this section is that guidance applied concretely to the filesystem case.

---

# 5. Error Model

This section describes categories of failure at the behavioral level — what kind of thing can go wrong and what it means — not a concrete error type or representation. That remains GOV-010's open question; this section gives it real categories to design against.

- **Invalid input.** The descriptor itself is malformed or empty before any filesystem access is even attempted. This is not a new category — it's the same class of failure already guarded against today (an empty descriptor is already rejected). Restated here only because it remains the first check in the filesystem case too.
- **Inaccessible paths.** The descriptor is well-formed, but the real location it names cannot actually be reached: it does not exist, or it exists but access is denied, or it sits on storage that is not currently available. This is a genuinely new failure category — the synthetic collector never had anything to represent here, because it never depended on anything external.
- **Unsupported input.** The location exists and is reachable, but is not a kind of thing this collector knows how to look at — for example, a device file or named pipe rather than a regular file or directory. This is distinct from inaccessibility: the thing is right there, it simply isn't a supported shape.
- **Empty collections.** The input is valid, reachable, and a supported kind of location, but structurally contains nothing (an empty directory, for instance). This is explicitly **not** a failure. `EvidenceCollection.md`'s Collector Contract already establishes this distinction in the abstract — "legitimate absence" is a successful result, not an error — and it applies here without modification: an empty directory is a fact about the subject, not a problem with collection.

**Which failures belong to Collection.** Invalid input, inaccessible paths, and unsupported input are all Collection's own concern — each occurs before or during the act of gathering Evidence, and only Collection has the context at the moment it happens to distinguish one from another. None of these should ever surface as a Finding: a Finding is an assessment conclusion about the subject, and a collection failure is not a conclusion about the subject at all — it's a statement that no conclusion could be reached because gathering the facts didn't succeed. Blurring the two would compromise the same producer/consumer boundary the whole subsystem exists to preserve.

**What does not belong to Collection.** Judging whether a successfully discovered structure is adequate for a valid mod belongs to the Rule Engine, evaluating Evidence Collection already produced. Deciding what happens to the rest of an Assessment when a collection failure occurs — does the whole Assessment abort, does it proceed with partial results, is the failure recorded some other way — is an orchestration-level policy decision. That decision is tied to how the Engine's entry points behave (relevant to GOV-008) and to how a failure is ultimately represented (GOV-010); this proposal characterizes the failure *categories* those decisions will need to account for, without resolving either.

---

# 6. Sandbox Interaction

The Sandbox remains exactly what it has always been: a thin developer visualization tool owning zero domain logic. This section evaluates what *class* of capability it needs, not how it should look.

- **What the Sandbox provides.** A way to identify which real filesystem location to inspect. At minimum, this is a real, existing path — not a synthetic string. The Evidence Collection Boundary proposal already reasoned that a native file/folder picker returning a path is a thin, non-domain-logic capability, and that reasoning still holds. But this milestone does not require building one: a fixed, real path to an actual location that already exists in the development environment (for example, a small fixture directory, or the Sandbox's own source tree) would exercise a genuine filesystem collector end-to-end without any new Sandbox UI at all — the same minimal-footprint pattern that successfully proved out the pipeline in Sprint 3 Phase 1 and Phase 4. Whether a real file-picker capability is worth adding is a separate, later, and explicitly UI-scoped decision, not part of this proposal.
- **What the Engine owns.** Orchestration — invoking Collection with whatever descriptor it is given, exactly as the existing entry point already does. Whether or how the Engine's behavior changes when Collection reports a failure (Section 5) is an orchestration-level policy question, not something this proposal resolves.
- **What Collection owns.** The actual traversal, the discovery of files and directories, and the classification of any failure into one of the categories in Section 5.

No UI design — a picker, drag-and-drop, a progress indicator, or anything else — is proposed or should be inferred from this section.

---

# 7. Success Criteria

This milestone should be considered architecturally complete when:

- A real, non-synthetic filesystem location can be given to the pipeline and inspected, producing Evidence that reflects what is actually present on disk — not a hardcoded value.
- Both a success case and at least one genuine failure case (Section 5) have been demonstrated — the synthetic collector never had to prove failure-handling at all, and this milestone should not be considered complete on the strength of its success path alone.
- Collection of the same, unchanged real filesystem content produces identical Evidence content across repeated runs (Section 4).
- No content-level parsing — ZIP, XML, or Lua — has occurred anywhere in the process; the scope boundary in Section 3 held throughout.
- The Sandbox demonstrates the capability end-to-end through the real engine, without requiring new domain logic inside the Sandbox itself.
- GOV-009 and/or GOV-010 have been resolved, or are concretely ready to be resolved, using this real implementation as the evidence — not deferred again for lack of a real case to design against.

---

# 8. Future Evolution

This milestone is deliberately narrow, but its boundary is exactly what the next several capabilities need in order to exist:

- **ZIP traversal.** Once file discovery can observe that a particular path is a file (including, incidentally, one shaped like a `.zip`), a future archive-aware collector becomes a second, additive Collector that takes a location this milestone already knows how to identify and looks *inside* it. The line this proposal draws between "a file exists" and "what an archive contains" is exactly the line ZIP traversal will sit on the far side of.
- **XML inspection.** Once a real file is known to exist at a specific, real location — the direct product of this milestone — XML inspection is the next collector that receives that same kind of location reference and, for the first time, looks at *content* rather than *existence*. The Input Descriptor concept this milestone establishes for "a location on disk" extends naturally to "a location within a location."
- **Lua inspection.** Follows the same pattern as XML: a collector operating on a real, already-discovered file. This milestone does not touch Lua in any way, but by proving that a Collector can be safely and deterministically handed a reference to a real file without crossing into evaluation, it establishes the template the platform's highest-risk future collector will need to follow.
- **Manifest analysis.** As `ROADMAP_REVIEW_2026.md` already observed, manifest parsing is XML inspection scoped to a specific, expected file (`modDesc.xml`). This milestone's file-discovery capability is exactly what would first confirm such a file exists at its expected location, before any manifest-specific parsing logic ever runs.
- **Dependency analysis.** Depends on manifest data existing first, since declared dependencies live inside `modDesc.xml`. This milestone's contribution is indirect but foundational: nothing about dependency analysis can begin until there is a reliable, deterministic way to locate the manifest file in the first place — which is precisely what this milestone proves out.
