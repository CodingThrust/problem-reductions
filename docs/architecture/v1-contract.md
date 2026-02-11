# v1 Core Contract

**Status:** Draft  
**Date:** 2026-02-11  
**Scope:** Stable conceptual interfaces for problem definitions and reductions

## Purpose

This document defines the v1 contract for core concepts used across the crate:
- Problem identity and variant metadata
- Problem instance evaluation
- Assignment representation
- Reduction interface between problem instances

The goal is to keep these concepts stable while implementation details evolve.

## Core Types

1. `Assignment`
- Typed wrapper around `Vec<usize>`
- Provides validation against `(num_variables, num_flavors)`

2. `Evaluation<T>`
- Stores `(objective, feasible)`
- Replaces ad hoc raw tuples and aligns with legacy `SolutionSize<T>`

3. `ObjectiveValue`
- Semantic bound alias for objective numeric types
- Centralizes repeated generic constraints

4. `ObjectiveDirection`
- `Maximize | Minimize`
- Canonical comparison semantics (`is_better`, `is_better_or_equal`)

5. `VariantDimension`
- Typed key/value metadata replacing stringly variant pairs
- Supports compatibility mapping from legacy `("graph", "SimpleGraph")` format

## Core Traits

1. `ProblemSpec`
- `NAME`
- `Value` (objective value type)
- `variant_dimensions()`

2. `ProblemInstance`
- `num_variables()`
- `num_flavors()`
- `size_profile()`
- `objective_direction()`
- `evaluate_assignment()`

3. `Reduction<S, T>`
- `target_instance()`
- `project_assignment()`
- `source_size_profile()`
- `target_size_profile()`

## Compatibility Layer

The v1 core currently coexists with legacy APIs:
- `LegacyProblemAdapter<P>` wraps old `Problem` implementations.
- `LegacyReductionAdapter<R, S, T>` wraps old `ReductionResult` implementations.

This keeps the crate functional during migration while enabling incremental adoption of v1 interfaces.

## Contract Invariants

1. Assignment validation must detect both shape and flavor-range errors.
2. Objective-direction comparison semantics must be deterministic and match legacy energy mode behavior.
3. Reduction projection must map target assignments back into source assignment space.
4. Variant metadata must remain serializable and stable for registry/export usage.

## Next Steps

1. Migrate registry/macro metadata to typed variant dimensions.
2. Unify graph taxonomy under one canonical topology system.
3. Incrementally migrate model and rule implementations to native v1 traits.
