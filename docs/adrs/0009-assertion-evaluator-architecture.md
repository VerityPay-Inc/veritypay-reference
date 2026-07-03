---
id: ADR-0009
title: Assertion Evaluator Architecture
status: accepted
version: 1.0.0
authors:
  - VerityPay Core Team
reviewers: []
related_docs:
  - docs/adrs/0005-evaluation-rule-architecture.md
  - docs/adrs/0006-ruleset-architecture.md
  - docs/adrs/0007-reference-interpreter-public-contract.md
  - ARCHITECTURE.md
  - ROADMAP.md
decision_date: 2026-07-02
superseded_by: null
---

# ADR-0009 — Assertion Evaluator Architecture

**Status:** Accepted · **Version:** 1.0.0 · **Date:** 2026-07-02

**Related:** [ADR-0005](0005-evaluation-rule-architecture.md) · [ADR-0006](0006-ruleset-architecture.md) · [ADR-0007](0007-reference-interpreter-public-contract.md) · [VP-RFC-0005](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0005-assertion-types.md) · [VP-RFC-0006](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0006-assertion-evaluation-dispatch.md) · [ROADMAP.md](../../ROADMAP.md)

---

## Purpose

Align the reference interpreter internal architecture with draft protocol dispatch in **VP-RFC-0005** (*Assertion Types*) and **VP-RFC-0006** (*Assertion Evaluation Dispatch*) ahead of **Platform 1.3**.

> **This ADR defines reference-interpreter engineering structure only.** It does **not** amend `veritypay-spec` or change the public interpreter contract in [ADR-0007](0007-reference-interpreter-public-contract.md).

---

## Context

### Current implementation (Platform 1.2)

```text
EvaluationContext
        ↓
    RuleSet
        ↓
 EvaluationRule(s)
        ↓
VerificationResult
```

The **interpreter** owns a **`RuleSet`** and executes **`EvaluationRule`** implementations directly ([ADR-0005](0005-evaluation-rule-architecture.md), [ADR-0006](0006-ruleset-architecture.md)).

### Target protocol model (Platform 1.3)

```text
EvaluationContext
        ↓
 Assertion Type
        ↓
Assertion Evaluator
        ↓
 EvaluationRule(s)
        ↓
VerificationResult
```

**VP-RFC-0006** requires **Evaluation Dispatch** — deterministic selection of exactly one **Assertion Evaluator** from `assertion_type` before type-specific protocol rules execute. Unknown types yield **`indeterminate`**.

---

## Decision

**Evolve the reference interpreter from a rule-centric orchestrator to an assertion-evaluator architecture** while preserving [ADR-0007](0007-reference-interpreter-public-contract.md) public entrypoints.

### Introduce `AssertionEvaluator`

| Responsibility | Detail |
|----------------|--------|
| Semantic interpretation | Own evaluation semantics for one or more **Assertion Type** identifiers |
| Rule execution | **MAY** execute one or more protocol **`EvaluationRule`** implementations |
| Result | Produce one **`VerificationResult`** per **`EvaluationContext`** invocation |

**`EvaluationRule`** remains the unit of per-check logic but becomes an **implementation detail** inside evaluators — not the interpreter's direct dependency.

### `AssertionEvaluatorRegistry`

- Dispatches from `claim.assertion.assertion_type` to exactly one evaluator.
- Dispatch is **deterministic** and depends **only** on **Assertion Type** ([VP-RFC-0006](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0006-assertion-evaluation-dispatch.md)).
- Unknown types yield **`indeterminate`** without executing type-specific rules.

### Initial evaluator — `BodyEqualityEvaluator`

```text
body_equality
        ↓
BodyEqualityEvaluator
        ↓
    RuleSet::platform_1()
        ↓
   VP-RULE-0002 → VP-RULE-0001
```

**Engineering compatibility:** **`minimal`** (accepted **VP-RFC-0001** fixture profile label) **MAY** alias to **`BodyEqualityEvaluator`** until published VP-CS fixtures align on `body_equality`. This preserves Platform 1.1/1.2 oracle behavior without amending fixtures in this ADR.

### RuleSet ownership

Each evaluator **owns** its **`RuleSet`**. The **interpreter never executes rules directly** — it dispatches to an evaluator, which runs its rule set.

`Interpreter::with_rule_set` remains available for tests and advanced wiring by configuring **`BodyEqualityEvaluator`**'s rule set only.

### Future evaluators (informative)

| Evaluator | Typical assertion themes |
|-----------|--------------------------|
| **RegexEvaluator** | Pattern match |
| **NumericEvaluator** | Numeric comparison |
| **SignatureEvaluator** | Digital signature verification |
| **SchemaEvaluator** | Schema validation |
| **HashEvaluator** | Cryptographic hash comparison |

Each future evaluator **MUST** own its **`RuleSet`** (or equivalent internal pipeline) per this ADR.

---

## Public API (unchanged)

Per [ADR-0007](0007-reference-interpreter-public-contract.md):

| Entrypoint | Status |
|------------|--------|
| `Interpreter::evaluate(&EvaluationContext) -> VerificationResult` | **Unchanged** |
| `Interpreter::evaluate_input(&EvaluationInput) -> VerificationResult` | **Unchanged** |
| `Interpreter::new()`, `with_rule_set`, `placeholder`, `rule_set` | **Unchanged** |

Only **internal** orchestration changes.

---

## Consequences

### Positive

- Matches **VP-RFC-0005** / **VP-RFC-0006** dispatch vocabulary in code structure.
- Isolates future assertion semantics behind evaluators without rewriting interpreter entrypoints.
- Keeps **`EvaluationRule`** and **`RuleSet`** patterns from [ADR-0005](0005-evaluation-rule-architecture.md) / [ADR-0006](0006-ruleset-architecture.md).

### Negative / trade-offs

- Additional indirection layer between interpreter and rules.
- `with_rule_set` configures only the body-equality evaluator until multi-evaluator test hooks are needed.

### Neutral

- **Platform 1.2** verification outcomes and traces remain unchanged for existing fixtures using `minimal` or `body_equality`.

---

## Alternatives considered

### Alternative A — Keep rule-centric interpreter

**Rejected.** Does not align with **VP-RFC-0006** dispatch model or prepare for multiple assertion types.

### Alternative B — Replace `EvaluationRule` with evaluators only

**Rejected.** Rules remain the normative protocol slice; evaluators compose rules.

### Alternative C — Break public API with `evaluate_assertion`

**Rejected.** [ADR-0007](0007-reference-interpreter-public-contract.md) stability is required for conformance integration.

---

## Implementation notes

| Crate | Change |
|-------|--------|
| `vp-reference-interpreter` | Add `evaluators/` module: `AssertionEvaluator` trait, `BodyEqualityEvaluator`, `AssertionEvaluatorRegistry`; refactor `Interpreter` |

**Out of scope for this ADR:**

- Normative acceptance of **VP-RFC-0005** / **VP-RFC-0006** in `veritypay-spec`
- Conformance fixture `assertion_type` string alignment
- Additional evaluator implementations beyond **`BodyEqualityEvaluator`**

---

## Related documents

| Document | Role |
|----------|------|
| [ADR-0005](0005-evaluation-rule-architecture.md) | `EvaluationRule` trait |
| [ADR-0006](0006-ruleset-architecture.md) | `RuleSet` ordering |
| [ADR-0007](0007-reference-interpreter-public-contract.md) | Public contract preserved |
| [VP-RFC-0005](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0005-assertion-types.md) | Assertion Type taxonomy |
| [VP-RFC-0006](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0006-assertion-evaluation-dispatch.md) | Evaluation Dispatch |

---

## Changelog

| Version | Date | Summary |
|---------|------|---------|
| 1.0.0 | 2026-07-02 | Accepted — assertion-evaluator architecture; `BodyEqualityEvaluator`; public API unchanged |
