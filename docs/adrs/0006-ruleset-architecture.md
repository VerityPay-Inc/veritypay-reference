---
id: ADR-0006
title: RuleSet Architecture
status: accepted
version: 1.0.0
authors:
  - VerityPay Core Team
reviewers: []
related_docs:
  - docs/adrs/0005-evaluation-rule-architecture.md
  - docs/adrs/0004-minimal-evaluation-semantics.md
  - ROADMAP.md
decision_date: 2026-07-05
superseded_by: null
---

# ADR-0006 — RuleSet Architecture

**Status:** Accepted · **Version:** 1.0.0 · **Date:** 2026-07-05

**Related:** [ADR-0005](0005-evaluation-rule-architecture.md) · [ADR-0004](0004-minimal-evaluation-semantics.md) · [ADR-0003](0003-domain-model-architecture.md) · [ROADMAP.md](../../ROADMAP.md)

---

## Purpose

Define how **multiple `EvaluationRule` implementations are grouped and executed** in `vp-reference-interpreter` after [Milestone D](../../ROADMAP.md).

---

## Context

[Milestone D](../../ROADMAP.md) implemented the first executable evaluation path per [ADR-0004](0004-minimal-evaluation-semantics.md) and [ADR-0005](0005-evaluation-rule-architecture.md):

- **`Interpreter::evaluate(&EvaluationContext) -> VerificationResult`**
- **`EvaluationRule`** trait and **`RuleEvaluation`** fragment
- **`MinimalBodyEqualityRule`** as the sole rule

The interpreter currently holds a single rule directly. That is correct for one rule; it does not scale when verification grows toward many checks.

[ADR-0005](0005-evaluation-rule-architecture.md) deferred **`RuleSet`**, rule registry, and specification-driven selection. Before the second rule lands, the project must record **how rules are collected and ordered** without rewriting interpreter orchestration each time.

> **This ADR defines reference-interpreter engineering structure only.** It does **not** define VerityPay protocol rules or amend `veritypay-spec`.

---

## Decision

**Introduce a conceptual `RuleSet`** that owns an **ordered collection of `EvaluationRule` implementations**. The **interpreter receives a `RuleSet`** and executes its rules in deterministic order.

### Architecture overview

```
EvaluationContext
        ↓
   Interpreter              (orchestration — unchanged public API)
        ↓
     RuleSet                (ordered rule collection — new grouping layer)
        ↓
 EvaluationRule × N        (MinimalBodyEqualityRule, future rules, …)
        ↓
   RuleEvaluation × N       (per-rule fragments)
        ↓
VerificationResult          (aggregated, frozen)
```

The interpreter **does not** grow a new field per rule. It **does not** manually instantiate every future rule inline. It **executes whatever ordered rules the `RuleSet` exposes**.

---

## 1. RuleSet

**`RuleSet`** is a conceptual object in **`vp-reference-interpreter`** owning an ordered collection of rules.

| Responsibility | Detail |
|----------------|--------|
| **Expose ordered rules** | Provide a deterministic sequence of `EvaluationRule` references for one evaluation run |
| **Deterministic execution** | Same `RuleSet` + same `EvaluationContext` → same rule invocation order |
| **No protocol semantics** | `RuleSet` names and orders rules; it does **not** decide outcomes—that remains in each `EvaluationRule` and aggregation logic |

`RuleSet` **does not**:

- Parse specification files or registry YAML
- Read filesystem paths
- Define normative protocol meaning
- Mutate `EvaluationContext` or domain inputs

Conceptual interface (illustrative only):

```rust
// Conceptual — not implemented in this ADR
trait RuleSet {
    fn rules(&self) -> &[dyn EvaluationRule]; // or equivalent ordered collection
}
```

Concrete storage (slice, `Vec`, static array, builder) is an implementation detail deferred to a follow-on PR.

---

## 2. Interpreter relationship

| Layer | Role |
|-------|------|
| **`Interpreter`** | Receives **`EvaluationContext`**; holds or receives a **`RuleSet`**; iterates rules; aggregates **`RuleEvaluation`**; builds **`VerificationResult`** and **`Trace`** |
| **`RuleSet`** | Answers “which rules, in what order?” for this evaluation configuration |
| **`EvaluationRule`** | Answers “what outcome does this check produce?” for one step |

Public interpreter entrypoint **unchanged**:

```rust
interpreter.evaluate(&EvaluationContext) -> VerificationResult
```

Internal evolution: Milestone D’s direct `MinimalBodyEqualityRule` field becomes **`RuleSet`** containing that rule. Callers of `evaluate` see no API change.

The interpreter **does not**:

- Hard-code a growing list of rule constructors in `evaluate`
- Select rules ad hoc per milestone in orchestration code
- Bypass `RuleSet` ordering when multiple rules exist

---

## 3. Initial state

The **Milestone D equivalent `RuleSet`** contains **exactly one rule**:

| Order | Rule | Implements |
|-------|------|------------|
| 1 | **`MinimalBodyEqualityRule`** | [ADR-0004](0004-minimal-evaluation-semantics.md) |

Future milestones **add rules to the set** (or substitute edition-specific sets) **without changing interpreter orchestration**.

Aggregation with one rule remains trivial pass-through ([ADR-0005](0005-evaluation-rule-architecture.md)). When multiple rules exist, aggregation policy is defined in a successor ADR or Milestone E+ work—not in this document.

---

## 4. Ordering

| Principle | Detail |
|-----------|--------|
| **Deterministic order** | Rule execution order is fixed for a given `RuleSet` instance |
| **Policy owner** | Ordering policy belongs to **`RuleSet`** construction—not scattered in `Interpreter::evaluate` |
| **Interpreter follows** | Interpreter iterates rules in the order `RuleSet` exposes |

Examples of future ordering policies (deferred):

- Fixed compile-time registration order
- Edition manifest rule list order
- Explicit priority field per rule

Milestone D’s single-rule set has trivial ordering. The abstraction exists so multi-rule order is not retrofitted as special cases.

---

## 5. Future evolution

`RuleSet` may later become—**without changing `Interpreter::evaluate` API**:

| Direction | Purpose |
|-----------|---------|
| **Edition-specific** | Different rule collections per specification Edition pin |
| **RFC-extension aware** | Activate rules when optional RFC extensions are declared |
| **Specification-selected** | Choose active rules from loaded `SpecificationContext` metadata |
| **Dynamically built** | Construct sets from fixtures, conformance scenarios, or builder APIs at call time |

These are **architectural allowances**, not Milestone E commitments. Selection logic may live in factory helpers or `vp-reference-spec` integration later—**not** inside individual rules.

[ADR-0005](0005-evaluation-rule-architecture.md) **`EvaluationGraph`** remains separate: shared evaluation state across stages, not rule list ownership.

---

## 6. Non-goals

This ADR explicitly **does not** introduce:

| Non-goal | Reason |
|----------|--------|
| **Plugin architecture** | No dynamic shared-library rule loading |
| **Runtime loading** | Rules are compile-time workspace members |
| **Filesystem access** | RuleSet construction stays path-free at evaluation time |
| **Registry parsing** | Specification registry interpretation stays in `veritypay-tooling` / loaders—not rule list parsing in the interpreter |
| **Outcome aggregation policy for N rules** | Deferred until a second rule requires it |
| **Implementation** | Architecture record only; code follows in a separate PR |

---

## Alternatives considered

### 1. Interpreter holds `Vec<Box<dyn EvaluationRule>>` without `RuleSet` type

Add rules directly on the interpreter struct.

**Rejected.** Conflates orchestration with rule registration. Edition-specific sets and test doubles become awkward; interpreter struct grows every milestone.

### 2. Static match in `evaluate` for each new rule

```rust
run_rule_a();
run_rule_b();
```

**Rejected.** Same failure mode ADR-0005 rejected—unmaintainable as rule count grows.

### 3. Specification-driven rule list in Milestone E

Parse loaded spec to build rule order immediately.

**Deferred.** Requires stable spec-to-rule mapping not yet defined. `RuleSet` abstraction is prerequisite; spec selection attaches later.

### 4. `RuleSet` in `vp-reference-core`

Share rule collections via the contracts crate.

**Rejected.** Rule sets are interpreter implementation detail ([ADR-0002](0002-workspace-architecture.md), [ADR-0005](0005-evaluation-rule-architecture.md)).

---

## Consequences

### Positive

- **Stable interpreter API** — `evaluate(context)` survives rule growth
- **Clear registration boundary** — new rules touch `RuleSet` construction, not orchestration loops
- **Deterministic conformance** — ordered execution is explicit and testable
- **Edition-ready shape** — swap rule collections without rewriting evaluation entrypoint

### Negative

- **Indirection for one rule** — Milestone D code will refactor from direct rule field to `RuleSet` wrapper
- **Aggregation TBD** — multiple-rule outcome combination still undefined
- **Selection TBD** — how Edition or spec chooses a `RuleSet` is future work

**Acceptable** because grouping rules now prevents orchestration churn when the second rule arrives.

---

## Related decisions

| Document | Relationship |
|----------|--------------|
| [ADR-0005](0005-evaluation-rule-architecture.md) | `EvaluationRule` abstraction; deferred `RuleSet` |
| [ADR-0004](0004-minimal-evaluation-semantics.md) | First rule member |
| [ADR-0003](0003-domain-model-architecture.md) | `EvaluationContext` input |
| [ROADMAP.md](../../ROADMAP.md) | Milestone D complete; multi-rule expansion follows this ADR |

---

## Follow-up

- [ ] Refactor `Interpreter` to hold a `RuleSet` with `MinimalBodyEqualityRule` (separate PR)
- [ ] Define multi-rule outcome aggregation when a second rule is added
- [ ] Revisit Edition-specific set construction when specification binding drives rule selection

---

## Conclusion

**`RuleSet`** owns an **ordered, deterministic collection of `EvaluationRule` implementations**. The **interpreter executes the set**—it does not manually instantiate every rule forever. The initial set contains **`MinimalBodyEqualityRule` only**; future milestones add rules without changing **`evaluate(&EvaluationContext)`**.

This ADR records architecture only. It does **not** implement code or alter normative specification text.
