# ADR-0005: Deterministic Assessment Engine

| Property | Value |
|----------|-------|
| **ADR** | 0005 |
| **Title** | Deterministic Assessment Engine |
| **Status** | Accepted |
| **Project** | modIQ |
| **Date** | 2026-07-16 |

---

# Context

The primary objective of modIQ is to provide evidence-based, explainable assessments of Farming Simulator mods.

During architectural design, consideration was given to incorporating artificial intelligence into the assessment process. While AI may provide valuable assistance in future platform capabilities, relying on probabilistic reasoning as the authoritative source of assessment would reduce explainability, reproducibility, and user trust.

The platform required an assessment model capable of producing consistent results for identical inputs while preserving complete traceability between engineering knowledge, evidence, findings, and recommendations.

---

# Decision

The assessment engine shall be deterministic.

Authoritative assessment outcomes shall be produced through deterministic evaluation of reusable engineering knowledge against collected runtime evidence.

Assessment execution shall follow a consistent conceptual workflow:

1. Collect Evidence.
2. Select applicable Rules.
3. Evaluate Rules against Evidence.
4. Generate Findings.
5. Generate Recommendations.
6. Produce an explainable Assessment Report.

Given identical inputs, identical Version Profiles, and identical engineering knowledge, the platform shall produce identical assessment results.

Artificial intelligence may be incorporated in future platform capabilities to assist users with exploration, explanation, learning, or content generation.

AI shall not replace deterministic rule evaluation as the authoritative source of assessment.

---

# Consequences

Benefits include:

- Repeatable assessment results.
- Complete traceability from Evidence to Findings.
- Improved user trust through explainable outcomes.
- Stable engineering behavior across platform versions.
- Simplified testing and validation.
- Preservation of reusable engineering knowledge independent of implementation technology.

This decision establishes deterministic assessment as a foundational architectural characteristic of the modIQ platform while allowing future AI capabilities to complement, rather than replace, deterministic evaluation.

---

# Relationship to Other Specifications

This decision is reflected in:

- Vision.md
- Principles.md
- ProductSpecification.md
- RuleEngine.md
- EngineAPI.md
- Sprint1.md

Future platform capabilities should preserve deterministic assessment as the authoritative execution model.

---

# Status

Accepted.

This decision is authoritative for Documentation Release 1.0.