---
id: ADR-0007
title: Reference Interpreter Public Contract
status: accepted
version: 1.0.0
authors:
  - VerityPay Core Team
reviewers: []
related_docs:
  - docs/adrs/0006-ruleset-architecture.md
  - docs/adrs/0003-domain-model-architecture.md
  - ARCHITECTURE.md
  - ROADMAP.md
decision_date: 2026-07-05
superseded_by: null
---

# ADR-0007 — Reference Interpreter Public Contract

**Status:** Accepted · **Version:** 1.0.0 · **Date:** 2026-07-05

**Related:** [ADR-0006](0006-ruleset-architecture.md) · [ADR-0005](0005-evaluation-rule-architecture.md) · [ADR-0003](0003-domain-model-architecture.md) · [ARCHITECTURE.md](../../ARCHITECTURE.md) · [ROADMAP.md](../../ROADMAP.md) · [veritypay-spec — CONFORMANCE_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md)

---

## Purpose

Declare the **stable public contract** of `veritypay-reference` before conformance integration and downstream consumer work accelerates.

---

## Context

[Milestones B–D](../../ROADMAP.md) delivered an executable path:

- Load validated specification input ([Milestone B](../../ROADMAP.md))
- Pure domain types, builders, and **`EvaluationContext`** ([Milestone C](../../ROADMAP.md))
- **`Interpreter::evaluate`** with **`RuleSet`** and **`MinimalBodyEqualityRule`** ([Milestone D](../../ROADMAP.md), [ADR-0004](0004-minimal-evaluation-semantics.md), [ADR-0005](0005-evaluation-rule-architecture.md), [ADR-0006](0006-ruleset-architecture.md))

Upcoming milestones will add richer traces, reporting, additional rules, and **`veritypay-conformance`** integration. Before those expand surface area, the project must record **what callers may depend on**—distinct from internal rule implementations that will change.

> **This ADR defines an engineering contract for the reference interpreter.** It does **not** define VerityPay protocol semantics or amend `veritypay-spec`.

---

## Stable contract

```
EvaluationContext
        ↓
Interpreter::evaluate(&EvaluationContext)
        ↓
VerificationResult
```

Future milestones may add **rules**, **traces**, **metadata**, richer **`RuleSet`** construction, and **reporting**—but should **preserve this core call shape** unless a **successor ADR** explicitly supersedes it.

---

## Decision

**Adopt the call shape above as the stable public contract** between callers and **`vp-reference-interpreter`**.

---

## 1. Stable inputs

**`EvaluationContext`** (`vp-reference-core`) is the **single interpreter input**.

| Field | Type | Role |
|-------|------|------|
| **`specification`** | **`SpecificationContext`** | Path-free loaded spec view |
| **`claim`** | **`Claim`** | Claim envelope + **`Assertion`** |
| **`evidence`** | **`Evidence`** | Evidence envelope + **`EvidenceContent`** |
| **`options`** | **`EvaluationOptions`** | Evaluation knobs (e.g. `deterministic`, `trace_enabled`) |

| Requirement | Detail |
|-------------|--------|
| **Path-free** | No `Path`, `PathBuf`, or live filesystem handles in the contract |
| **Filesystem-agnostic** | Callers resolve paths upstream; the interpreter never receives them |
| **Immutable for one call** | Inputs do not mutate during `evaluate` |
| **Construction** | **`EvaluationContextBuilder`** or future parsers/builders upstream—not inside the interpreter |

Specification loading, CLI path resolution, and fixture parsing remain **outside** this contract ([ADR-0002](0002-workspace-architecture.md)).

---

## 2. Stable entrypoint

| Element | Contract |
|---------|----------|
| **Type** | **`Interpreter`** (`vp-reference-interpreter`) |
| **Method** | **`evaluate(&EvaluationContext) -> VerificationResult`** |
| **Orchestration** | Interpreter owns execution through **`RuleSet`** ([ADR-0006](0006-ruleset-architecture.md)) |
| **Rules** | Individual **`EvaluationRule`** implementations may change, be added, or be replaced |
| **Caller stability** | Callers depend on **input/output types and method signature**—not on which rules run internally |

The interpreter **does not** expose a growing list of positional parameters (`spec`, `claim`, `evidence`, …). **`EvaluationOptions`** and future fields extend **`EvaluationContext`** instead.

Internal evolution (new rules, aggregation policy, **`EvaluationGraph`**) must not break **`evaluate(&EvaluationContext) -> VerificationResult`** without ADR supersession.

---

## 3. Stable output

**`VerificationResult`** (`vp-reference-model`) is the **root oracle artifact** returned by evaluation.

| Field (conceptual) | Role |
|--------------------|------|
| **`evaluated_claim_id`** | Links result to the claim under test |
| **`outcome`** | Normative **`Outcome`** |
| **`trace`** | Explanatory **`Trace`** (may be empty when disabled) |
| **`specification_binding`** | Edition / protocol version pin used at evaluation |
| **`metadata`** | Non-normative context (must not decide protocol truth) |
| **`reasons`** | Human- or machine-readable rationale strings |

**Outcome vocabulary** remains fixed per [VP-TERM-011](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/00-overview/GLOSSARY.md#verification-outcome):

| Value | Meaning |
|-------|---------|
| **`satisfied`** | Rules passed under declared evidence and binding |
| **`not_satisfied`** | Rules failed under declared evidence and binding |
| **`indeterminate`** | Outcome cannot be determined under rules |

No alternate normative outcome labels in the public contract unless accepted upstream in `veritypay-spec` and reflected in a successor ADR.

**`VerificationResult`** is **immutable** after `evaluate` returns. Reports and conformance compare or serialize it—they do not mutate it ([ADR-0003](0003-domain-model-architecture.md)).

---

## 4. Consumers

This contract is intended for:

| Consumer | Usage |
|----------|--------|
| **CLI** (`vp-reference-cli`) | Assemble **`EvaluationInput`**; call **`evaluate_input`** (verify/serve) or **`evaluate`** (conformance oracle path); human/JSON output via `output.rs` / `explain.rs` |
| **Reports** (`vp-reference-report`) | Format **`VerificationResult`** without re-running evaluation *(crate placeholder; CLI output delivered)* |
| **`veritypay-conformance`** | Default oracle for VP-CS expected outcomes via **`evaluate`** |
| **Examples and education** | Demonstrate claim → evidence → outcome |
| **SDK / implementation comparisons** | Compare independent stacks against the same result shape |

Consumers **may** depend on:

- **`EvaluationContext`** and **`VerificationResult`** field presence and semantics as documented in ADRs and model types
- **`Outcome`** enum variants and canonical labels
- Stable **`evaluate`** signature

Consumers **must not** depend on:

- Internal rule implementation details
- Private interpreter modules
- Temporary minimal rule semantics ([ADR-0004](0004-minimal-evaluation-semantics.md)) as protocol truth

---

## 5. Boundaries

The public contract **excludes**:

| Excluded | Belongs elsewhere |
|----------|-------------------|
| **Filesystem types** | `vp-reference-cli`, `vp-reference-spec` |
| **CLI argument types** | `vp-reference-cli` |
| **Report formatting** | `vp-reference-report` |
| **Parser-owned result types** | Future parser modules; parsers **produce** model types |
| **Protocol semantics invention** | `veritypay-spec` only |
| **`vp-spec-model` in interpreter API** | `vp-reference-spec` loading boundary |
| **Tooling validator diagnostics** | `veritypay-tooling` |

The interpreter API is a **library contract** for evaluation—not a CLI, not a serializer, not a spec loader.

---

## Alternatives considered

### 1. Unstable `evaluate` until conformance ships

Defer contract declaration until Milestone G.

**Rejected.** Conformance, reports, and examples need a declared oracle surface before integration code proliferates around ad hoc signatures.

### 2. Return `(Outcome, Trace)` tuple instead of `VerificationResult`

Minimal return type without root result object.

**Rejected.** [`VerificationResult`](0003-domain-model-architecture.md) already aggregates binding, reasons, and metadata—required for conformance comparison.

### 3. Separate `evaluate_spec`, `evaluate_claim` entrypoints

Multiple interpreter methods per input kind.

**Rejected.** [`EvaluationContext`](0003-domain-model-architecture.md) already unifies inputs; multiple entrypoints would churn as options grow.

### 4. Public trait objects for every rule

Expose `EvaluationRule` to all consumers.

**Rejected.** Rules are interpreter internals ([ADR-0005](0005-evaluation-rule-architecture.md)). Consumers depend on **`VerificationResult`**, not rule wiring.

---

## Consequences

### Positive

- **Conformance-ready surface** — `veritypay-conformance` can target one call shape
- **Clear consumer boundaries** — CLI, reports, and SDKs share the same oracle types
- **Internal flexibility** — rules and `RuleSet` evolve without breaking callers
- **Reviewable stability** — contract changes require explicit ADR supersession

### Negative

- **Contract discipline** — contributors must not extend `evaluate` casually
- **`EvaluationContext` growth** — new options must fit the input object carefully
- **Milestone E/F overlap** — some outcome/trace success criteria are partially met by Milestone D; contract declares stability before all milestones complete

**Acceptable** because declaring stability now is cheaper than retrofitting conformance after ad hoc integrations.

---

## Future

| Topic | Direction |
|-------|-----------|
| **Contract change** | Requires **successor ADR**—not drive-by signature changes |
| **Serialization schema** | May stabilize JSON export for conformance; **`VerificationResult`** remains the in-memory contract |
| **Additional `EvaluationOptions`** | Added to **`EvaluationContext`** without new `evaluate` overloads |
| **Richer `RuleSet` factories** | Internal; **`evaluate`** unchanged |
| **Milestone G** | Conformance runners invoke **`evaluate`** as library consumers |

---

## Related decisions

| Document | Relationship |
|----------|--------------|
| [ADR-0003](0003-domain-model-architecture.md) | Domain types and `EvaluationContext` |
| [ADR-0004](0004-minimal-evaluation-semantics.md) | Current rule semantics (temporary engineering slice) |
| [ADR-0005](0005-evaluation-rule-architecture.md) | Rule vs orchestration split |
| [ADR-0006](0006-ruleset-architecture.md) | Internal rule grouping |
| [ADR-0002](0002-workspace-architecture.md) | Crate boundaries |
| [ROADMAP.md](../../ROADMAP.md) | Milestones E–G build on this contract |

---

## Follow-up

- [ ] Wire CLI evaluation command through `evaluate` when CLI scope expands (separate PR)
- [ ] Align `vp-reference-report` formatters to `VerificationResult` only
- [ ] Document conformance integration against this contract in Milestone G

---

## Conclusion

The stable public contract of `veritypay-reference` is:

**`EvaluationContext` → `Interpreter::evaluate` → `VerificationResult`**

Inputs stay **path-free**. The entrypoint stays **singular**. Outputs stay **`VerificationResult`** with normative **`Outcome`** vocabulary. Internal rules may change; **this call shape should not**—unless a future ADR supersedes this one.

This ADR records the contract only. It does **not** implement code or alter normative specification text.
