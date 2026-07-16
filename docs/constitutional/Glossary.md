# Glossary

> **The authoritative vocabulary of the modIQ platform.**

---

| Property | Value |
|----------|-------|
| **Document** | Glossary.md |
| **Version** | 1.0.0 |
| **Status** | Draft |
| **Project** | modIQ |
| **Documentation Release** | 1.0 |
| **Owner** | Zach Weber |
| **Created** | 2026-07-15 |
| **Last Updated** | 2026-07-15 |

---

## Purpose

No specification may redefine terminology established by this glossary. New terminology must be added here before it is used elsewhere.

Every product, engineering, architectural, and implementation document should use the terminology defined here. If a term is defined within this glossary, that definition takes precedence over any informal usage elsewhere.

---

# A

## Assessment

The complete analytical evaluation of a submitted mod.

An Assessment consists of evidence collection, rule evaluation, compatibility analysis, dependency analysis, performance observations, findings, recommendations, and the generation of an Assessment Report.

Assessment is the primary function of the modIQ platform.

---

## Assessment Framework (MAF)

The **modIQ Assessment Framework (MAF)** defines the standards, categories, rules, scoring methodology, and evidence requirements used during every Assessment.

The MAF is versioned independently from the platform so assessment standards can evolve over time.

---

## Assessment Report

The structured output generated after an Assessment.

Every Assessment Report should answer three questions:

1. Does the mod work?
2. Why was this assessment reached?
3. What can be done to improve it?

Assessment Reports prioritize explanation over scoring.

---

## Assessment Subject

The artifact or entity being evaluated during an Assessment.

Every Assessment operates on exactly one Assessment Subject.

An Assessment Subject represents the primary object of evaluation and provides the context for evidence collection, rule execution, and the generation of Findings and an Assessment Report.

Examples of Assessment Subjects may include:

- Individual Farming Simulator mods
- Mod collections
- Savegames
- Maps
- Vehicle packs
- Placeable packs
- Future assessment targets supported by the platform

The Assessment Subject defines **what** is being assessed, while the Assessment defines **how** it is evaluated.

---

# C

## Compatibility

The degree to which a mod functions correctly within a specified Farming Simulator version and environment.

Compatibility includes but is not limited to:

- Game Version
- ModDesc Version
- Dependencies
- Runtime Behavior
- XML Structure
- Script Compatibility

---

## Confidence

A measurement indicating how certain modIQ is about an Assessment conclusion based upon available evidence.

Confidence reflects evidence quality—not correctness.

---

# D

## Dependency

Any external requirement needed for a mod to function correctly.

Examples include:

- Required Mods
- Script Libraries
- DLC
- Shared Assets
- Game Features

---

## Design Directive

A mandatory design principle established by Vision.md that influences future engineering decisions.

Design Directives are architectural constraints rather than implementation details.

---

# E

## Evidence

Any objective information collected during an Assessment.

Evidence may include:

- XML inspection
- Lua analysis
- Runtime logs
- Asset validation
- Dependency resolution
- Performance observations
- File structure analysis

Every Assessment conclusion should be traceable back to evidence.

---

# F

## Finding

A single observation produced during an Assessment.

Findings may represent:

- Errors
- Warnings
- Informational observations
- Best Practice recommendations

Multiple Findings contribute to the overall Assessment.

---

# K

## Knowledge Base (MKB)

The **modIQ Knowledge Base (MKB)** is the continuously evolving repository of validated knowledge accumulated through Assessments.

The Knowledge Base stores patterns, known issues, repair strategies, compatibility history, and engineering knowledge that improve future Assessments.

The MKB is one of the platform's primary long-term assets.

---

# M

## Mod Health

An overall representation of a mod's quality based upon the Assessment Framework.

Mod Health considers multiple dimensions rather than a single score.

Examples include:

- Compatibility
- Stability
- Maintainability
- Performance
- Structure
- Engineering Quality

---

# R

## Repair Recipe

A structured recommendation describing how a specific issue can be resolved.

Repair Recipes may reference:

- XML changes
- Lua changes
- Dependency installation
- Asset replacement
- Version updates

Repair Recipes should be reproducible.

---

## Rule

An individual evaluation criterion executed during an Assessment.

Rules examine one specific aspect of a mod.

Examples include:

- Invalid ModDesc Version
- Missing Dependency
- Missing Asset
- Lua Syntax Error
- Unsupported XML Structure

---

## Rule Engine

The subsystem responsible for executing Rules during an Assessment.

The Rule Engine evaluates evidence and produces Findings.

It does not make subjective judgments.

---

# S

## Stability

The likelihood that a mod will function reliably without producing runtime failures or unexpected behavior.

---

# T

## Transparency

The principle that every Assessment conclusion should be explainable and supported by evidence.

Transparency is a core design philosophy of modIQ.

---

# V

## Validation

The process of confirming that Assessment conclusions accurately reflect the collected evidence.

Validation exists to ensure reproducible Assessments.

---

## Version Profile

The collection of compatibility information associated with a specific Farming Simulator release.

Examples include:

- FS22
- FS25
- Future releases

Version Profiles allow modIQ to support multiple game generations without redesigning the Assessment Framework.

---

# Related Documents

- Vision.md
- Principles.md
- ProductSpecification.md
- DataModel.md

---

## Document Status

**Current Version:** 1.0.0

**Status:** Draft

This glossary is considered the authoritative vocabulary of the modIQ Design Specification.