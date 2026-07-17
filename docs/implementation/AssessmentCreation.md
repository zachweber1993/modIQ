# Assessment Creation

## Purpose

Defines the creation contract for Assessment aggregates.

## Creation Requirements

A valid Assessment SHALL be created with:

- AssessmentSubject
- AssessmentContext

## Initialization

Upon creation an Assessment SHALL:

- Generate a unique AssessmentId.
- Enter the Created lifecycle state.
- Initialize an empty Evidence collection.
- Initialize an empty Findings collection.
- Initialize an empty Recommendations collection.

## Ownership

Assessment SHALL own all runtime assessment state from creation until completion.