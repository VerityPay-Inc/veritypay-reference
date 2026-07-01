---
id: ADR-0005
title: Evaluation Rule Architecture
status: accepted
version: 1.0.0
authors:
  - VerityPay Core Team
reviewers: []
related_docs:
  - docs/adrs/0004-minimal-evaluation-semantics.md
  - docs/adrs/0003-domain-model-architecture.md
  - ARCHITECTURE.md
  - ROADMAP.md
decision_date: 2026-07-04
superseded_by: null
---

# ADR-0005 — Evaluation Rule Architecture

**Status:** Accepted · **Version:** 1.0.0 · **Date:** 2026-07-04

**Related:** [ADR-0004](0004-minimal-evaluation-semantics.md) · [ADR-0003](0003-domain-model-architecture.md) · [ADR-0002](0002-workspace-architecture.md) · [ARCHITECTURE.md](../../ARCHITECTURE.md) · [ROADMAP.md](../../ROADMAP.md)

---

## Purpose

Define how **verification rules are organized** inside **`vp-reference-interpreter`** before Milestone D implementation begins.

---

## Context

[ADR-0004](0004-minimal-evaluation-semantics.md) defines **what** the first minimal evaluation means: body-equality over `Assertion` and `EvidenceContent`, with outcome mapping and trace expectations.

[ADR-0003](0003-domain-model-architecture.md) defines **domain types** and **`EvaluationContext`** as the single interpreter input.

Milestone D will implement the first executable `evaluate(context)` path. Without a rule architecture decision, the natural implementation is an interpreter method containing inline conditionals:

```
Interpreter
    ↓
if assertion.body == evidence.content.body { ... }
```

That pattern collapses orchestration and protocol decision logic into one type. As verification rules grow from one fixture rule toward dozens of normative checks, an inline interpreter becomes an unmaintainable match statement—hard to test in isolation, hard to extend, and hard to align with specification-driven rule sets.

This ADR records **how rules decompose from the interpreter**—architecture only, no code.

---

## Decision

**Separate interpreter orchestration from rule decision logic** using a conceptual **`EvaluationRule`** abstraction.

### Architecture overview

```
EvaluationContext
        ↓
   Interpreter          (orchestration — vp-reference-interpreter)
        ↓
 EvaluationRule         (decision logic — one or more rule implementations)
        ↓
MinimalBodyEqualityRule (Milestone D — implements ADR-0004)
        ↓
   RuleEvaluation       (per-rule outcome fragment)
        ↓
VerificationResult     (aggregated, frozen — vp-reference-model)
```

The interpreter **coordinates**. Rules **decide**. The interpreter **aggregates** rule output into **`VerificationResult`** and final **`Trace`**.

---

## 1. Interpreter responsibility

**`vp-reference-interpreter`** owns evaluation **orchestration**, not protocol rules inline.

| Responsibility | Detail |
|----------------|--------|
| **Receive input** | Accept **`&EvaluationContext`** as the sole evaluation input ([ADR-0003](0003-domain-model-architecture.md)) |
| **Execute rules** | Invoke one or more **`EvaluationRule`** implementations in Milestone D order |
| **Aggregate result** | Combine **`RuleEvaluation`** values into a single normative outcome and reason set |
| **Produce `VerificationResult`** | Build a frozen result via **`VerificationResultBuilder`** ([ADR-0003](0003-domain-model-architecture.md)) |
| **Produce `Trace`** | Merge rule trace events with interpreter lifecycle events; honor **`EvaluationOptions::trace_enabled`** ([ADR-0004](0004-minimal-evaluation-semantics.md)) |

The interpreter **does not**:

- Embed protocol rule conditionals as the long-term pattern
- Read filesystem paths or load specification files
- Parse CLI arguments or format reports
- Mutate **`EvaluationContext`**, **`Claim`**, or **`Evidence`**

Public entrypoint remains:

```rust
// Conceptual API
interpreter.evaluate(&EvaluationContext) -> VerificationResult
```

---

## 2. Rule abstraction

Introduce a conceptual **`EvaluationRule`**:

```rust
// Conceptual — names and signatures are illustrative
trait EvaluationRule {
    fn evaluate(&self, context: &EvaluationContext) -> RuleEvaluation;
}
```

| Concept | Owner | Role |
|---------|-------|------|
| **`EvaluationRule`** | `vp-reference-interpreter` (rule modules) | Encapsulates one verification decision unit |
| **`RuleEvaluation`** | `vp-reference-interpreter` (or colocated type) | Per-rule outcome fragment returned to orchestrator |
| **`Interpreter`** | `vp-reference-interpreter` | Selects rules, runs them, aggregates, builds final artifacts |

**Separation principle:** If a change is “what outcome does this check produce?”, it belongs in a **rule**. If a change is “how are checks run and combined?”, it belongs in the **interpreter**.

Milestone D may use a single rule with trivial aggregation (the rule’s outcome **is** the final outcome). The abstraction still exists so Milestone E+ can add rules without rewriting orchestration.

---

## 3. Initial implementation

Milestone D contains **exactly one rule**:

| Rule | Implements | Rule reference label |
|------|------------|----------------------|
| **`MinimalBodyEqualityRule`** | [ADR-0004](0004-minimal-evaluation-semantics.md) body-equality semantics | `vp-ref-minimal.body-equality` |

Future milestones **replace or extend** this set—they do not grow the interpreter into a monolithic conditional chain.

The interpreter’s Milestone D orchestration:

1. Optionally emit evaluation-start trace events
2. Run **`MinimalBodyEqualityRule`**
3. Map **`RuleEvaluation`** → **`VerificationResult`**
4. Return frozen result

---

## 4. Rule ownership

### Rules may inspect (read-only)

| Input | Access |
|-------|--------|
| **`Claim`** | Via `context.claim()` — identity and **`Assertion`** |
| **`Evidence`** | Via `context.evidence()` — identity and **`EvidenceContent`** |
| **`SpecificationContext`** | Via `context.specification()` — binding metadata; unused by minimal rule in D |
| **`EvaluationOptions`** | Via `context.options()` — e.g. deterministic trace behavior |

Rules evaluate **`Assertion`** using **`EvidenceContent`** per [ADR-0003](0003-domain-model-architecture.md).

### Rules may not

| Prohibited | Reason |
|------------|--------|
| Read files or paths | Filesystem boundary ([ADR-0002](0002-workspace-architecture.md)) |
| Parse YAML, JSON fixtures, or registry documents | Parsing belongs outside interpreter |
| Access CLI types or environment | CLI is a thin shell |
| Mutate **`EvaluationContext`** or domain inputs | Inputs are immutable for the duration of `evaluate` |
| Call **`vp-reference-spec`** or **`vp-spec-model`** | Specification loading is upstream |

---

## 5. Rule result

Each rule returns a conceptual **`RuleEvaluation`**:

| Field | Required | Role |
|-------|----------|------|
| **`outcome`** | Yes | `Outcome` — `Satisfied`, `NotSatisfied`, or `Indeterminate` |
| **`reason`** | Yes | Human-readable explanation of the rule’s decision |
| **`rule_reference`** | Optional | Engineering or normative rule id (e.g. `vp-ref-minimal.body-equality`) |
| **`trace_events`** | Optional | Rule-local trace events to merge into final `Trace` |

The **interpreter** converts one or more **`RuleEvaluation`** values into **`VerificationResult`**:

| `VerificationResult` field | Source |
|----------------------------|--------|
| **`evaluated_claim_id`** | `context.claim().id` |
| **`outcome`** | Aggregated from rule outcome(s) — trivial pass-through in Milestone D |
| **`reasons`** | Collected from rule reason(s) |
| **`trace`** | Merged interpreter + rule events when `trace_enabled` |
| **`specification_binding`** | Derived from `context.specification()` |
| **`metadata`** | Interpreter does not invent normative metadata |

Trace merge order: interpreter start events → rule events → interpreter outcome summary ([ADR-0004](0004-minimal-evaluation-semantics.md)).

---

## 6. Future evolution

Later milestones may introduce—**without specifying implementation here**:

| Concept | Purpose |
|---------|---------|
| **`RuleSet`** | Named collection of rules for a scenario or spec edition |
| **Rule registry** | Register rules by id; support spec-driven lookup |
| **Rule ordering** | Deterministic sequencing when multiple rules apply |
| **Specification-driven rule selection** | Choose active rules from loaded `SpecificationContext` |
| **Parallel rule evaluation** | Independent checks with aggregated outcome |
| **`EvaluationGraph`** | Shared evaluation state across stages ([ADR-0003](0003-domain-model-architecture.md)) |

These are **deferred**. Milestone D proves orchestration + one rule; architecture must not preclude them.

---

## Alternatives considered

### 1. Inline conditionals in `Interpreter::evaluate`

Implement ADR-0004 logic directly in the interpreter method.

**Rejected.** Fastest for one rule; fails as rule count grows. Tests couple to orchestration; every new rule touches the same file.

### 2. Rules in `vp-reference-model`

Place `EvaluationRule` trait and rule types in the pure domain crate.

**Rejected.** Rules contain verification **logic**; domain crate holds data only ([ADR-0003](0003-domain-model-architecture.md)). Rules stay in **`vp-reference-interpreter`**.

### 3. Rules in `vp-reference-core`

Share rules via the contracts crate.

**Rejected.** `vp-reference-core` stays small—contexts and errors, not verification implementations ([ADR-0002](0002-workspace-architecture.md)).

### 4. External rule plugin system (Milestone D)

Dynamic loading of rules from shared libraries.

**Deferred.** Unnecessary complexity before rule count and conformance integration justify it.

---

## Consequences

### Positive

- **Scalable structure** — new rules add types/modules, not interpreter branches
- **Isolated testing** — `MinimalBodyEqualityRule` tested against ADR-0004 fixtures without full orchestration noise
- **Clear ownership** — orchestration vs decision logic matches [ARCHITECTURE.md](../../ARCHITECTURE.md) verification component
- **ADR-0004 preserved** — semantics unchanged; only **where** logic lives is decided

### Negative

- **Indirection for one rule** — Milestone D carries a thin abstraction tax
- **Aggregation rules TBD** — multiple-rule outcome combination not defined until needed
- **Naming stability** — `EvaluationRule` / `RuleEvaluation` may evolve when `RuleSet` arrives

**Acceptable** because one extra layer today prevents a thousand-line interpreter tomorrow.

---

## Related decisions

| Document | Relationship |
|----------|--------------|
| [ADR-0004](0004-minimal-evaluation-semantics.md) | First rule semantics — `MinimalBodyEqualityRule` |
| [ADR-0003](0003-domain-model-architecture.md) | Inputs, outputs, `EvaluationContext` |
| [ADR-0002](0002-workspace-architecture.md) | Interpreter crate boundaries |
| [ROADMAP.md](../../ROADMAP.md) | Milestone D implements this architecture |

---

## Follow-up

- [ ] Implement `Interpreter`, `EvaluationRule`, `RuleEvaluation`, and `MinimalBodyEqualityRule` (Milestone D)
- [ ] Unit-test rule in isolation and via `evaluate(context)` integration
- [ ] Revisit aggregation when a second rule is added (successor ADR if needed)

---

## Conclusion

**`vp-reference-interpreter`** orchestrates evaluation; **`EvaluationRule`** implementations own decision logic. Milestone D ships **`MinimalBodyEqualityRule`** as the sole rule, implementing [ADR-0004](0004-minimal-evaluation-semantics.md). The interpreter aggregates **`RuleEvaluation`** into **`VerificationResult`** and **`Trace`**—without inline protocol rules as the long-term pattern.

This ADR records architecture only. It does **not** implement code or alter normative specification text.
