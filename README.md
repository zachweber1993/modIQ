# modIQ

Deterministic engineering assessment platform for Farming Simulator modifications.

## Repository Structure

## Repository Structure

```text
modIQ/
├── crates/
├── docs/
├── sample-mods/
├── Cargo.toml
├── Cargo.lock
├── README.md
└── LICENSE
```

### Workspace Crates

| Crate | Responsibility |
|--------|----------------|
| modiq-runtime | Runtime assessment domain |
| modiq-knowledge | Engineering knowledge domain |
| modiq-versioning | Version Profile management |
| modiq-rules | Deterministic rule evaluation |
| modiq-engine | Assessment orchestration |
| modiq-report | Report representation and formatting |
| modiq-cli | Command-line interface |

---

## Documentation Reading Order

Documentation should be read in the following order:

1. Vision.md
2. Principles.md
3. Glossary.md
4. ProductSpecification.md
5. Architecture.md
6. DataModel.md
7. KnowledgeModel.md
8. RuleEngine.md
9. EngineAPI.md
10. Sprint0.md
11. Sprint1.md

Each document derives its authority from the specifications that precede it.

## Engineering Governance

The following documents support the Engineering Specification and should be read before contributing to the repository.

1. DocumentationRelease.md
2. EngineeringGuide.md

---

## Documentation

Project documentation is organized by purpose.

| Directory | Description |
|-----------|-------------|
| `docs/architecture/` | Engineering Specification |
| `docs/governance/` | Repository governance and project management |
| `docs/implementation/` | Sprint planning and engineering guidance |
| `docs/adrs/` | Architecture Decision Records |

---

## Getting Started

Build the workspace:

```bash
cargo build