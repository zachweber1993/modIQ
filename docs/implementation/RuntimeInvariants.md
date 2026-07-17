# Runtime Invariants

Runtime invariants define the rules that every Assessment must enforce regardless of implementation.

Violating an invariant represents a programming error or invalid assessment lifecycle.

## Assessment Lifecycle

### INV-001

An Assessment SHALL begin in the Created state.

### INV-002

Evidence MAY only be added before rule evaluation begins.

### INV-003

Evidence SHALL become immutable once rule evaluation starts.

### INV-004

Findings SHALL only be produced by deterministic rule evaluation.

### INV-005

Recommendations SHALL only be produced from one or more Findings.

## Aggregate Ownership

### INV-006

Assessment SHALL be the sole owner of runtime assessment state.

### INV-007

Evidence, Findings, and Recommendations SHALL only be mutated through the Assessment aggregate.

### INV-008

Assessment SHALL enforce all runtime lifecycle invariants.

### INV-009

No external component SHALL directly mutate Assessment-owned collections.