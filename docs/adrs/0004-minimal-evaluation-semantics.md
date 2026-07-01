---
id: ADR-0004
title: Minimal Evaluation Semantics
status: accepted
version: 1.0.0
authors:
  - VerityPay Core Team
reviewers: []
related_docs:
  - docs/adrs/0003-domain-model-architecture.md
  - ARCHITECTURE.md
  - ROADMAP.md
decision_date: 2026-07-03
superseded_by: null
---

# ADR-0004 — Minimal Evaluation Semantics

**Status:** Accepted · **Version:** 1.0.0 · **Date:** 2026-07-03

**Related:** [ADR-0003](0003-domain-model-architecture.md) · [ARCHITECTURE.md](../../ARCHITECTURE.md) · [ROADMAP.md](../../ROADMAP.md) · [veritypay-spec — CONFORMANCE_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md) · [veritypay-spec — GLOSSARY (VP-TERM-011)](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/00-overview/GLOSSARY.md#verification-outcome)

---

## Purpose

Define the **first executable interpreter behavior** for `veritypay-reference` before Milestone D implementation begins.

---

## Context

[Milestone C](../../ROADMAP.md) delivered:

- Pure domain types and builders in **`vp-reference-model`** ([ADR-0003](0003-domain-model-architecture.md))
- A path-free **`EvaluationContext`** in **`vp-reference-core`**
- Outcome vocabulary aligned with [VP-TERM-011](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/00-overview/GLOSSARY.md#verification-outcome): `satisfied`, `not_satisfied`, `indeterminate`

[Milestone B](../../ROADMAP.md) can load a validated specification checkout, but **no verification logic exists yet**. The interpreter crate is a placeholder.

Milestone D requires the first end-to-end evaluation: **`EvaluationContext` in → `VerificationResult` out**. Before code lands, this ADR records **what that first evaluation means**—intentionally minimal, fixture-driven, and **not** a claim about full VerityPay protocol semantics.

> **This ADR defines reference-interpreter engineering semantics for pipeline proof only.** It does **not** amend, extend, or reinterpret normative text in `veritypay-spec`. When accepted architecture documents define real verification rules, this minimal rule must be **replaced or superseded**—not mistaken for protocol truth.

---

## Decision

**Implement one temporary minimal rule** in **`vp-reference-interpreter`** that compares assertion and evidence **content bodies** and returns a frozen **`VerificationResult`**.

### Interpreter entrypoint

Per [ADR-0003](0003-domain-model-architecture.md), the interpreter exposes a **single evaluation method**:

```rust
// Conceptual API — implementation follows in Milestone D
interpreter.evaluate(&EvaluationContext) -> VerificationResult
```

| Input | Source | Role in minimal rule |
|-------|--------|----------------------|
| **`EvaluationContext`** | Caller (CLI, tests, future conformance) | Sole input bundle |
| **`context.specification()`** | Loaded **`SpecificationContext`** | Bound onto result; **not used to select rules** in this milestone |
| **`context.claim()`** | **`Claim`** envelope | Supplies **`assertion`** under evaluation |
| **`context.evidence()`** | **`Evidence`** envelope | Supplies **`content`** offered for the run |
| **`context.options()`** | **`EvaluationOptions`** | Controls trace emission (`trace_enabled`) |

The interpreter **does not** receive filesystem paths, `vp-spec-model` types, or parser artifacts. It consumes only **`EvaluationContext`**.

### Evaluation target

Rules evaluate **`claim.assertion`** using **`evidence.content`**—not claim/evidence identity fields, metadata, or envelope labels ([ADR-0003](0003-domain-model-architecture.md)).

For the minimal rule, only these fields matter:

| Field | Type | Used |
|-------|------|------|
| `claim.assertion.body` | `String` | Asserted payload |
| `evidence.content.body` | `String` | Offered material |

`assertion_type`, `content_type`, `subject`, and metadata are **ignored** by the minimal rule.

### Minimal rule — body equality

**Rule id (engineering label):** `vp-ref-minimal.body-equality`

**Rule reference (trace only):** `vp-ref-minimal.body-equality` — a reference-interpreter label, **not** a normative spec rule identifier until accepted upstream.

| Condition | Outcome | Rationale |
|-----------|---------|-----------|
| `evidence.content.body` is **empty** (zero bytes after trim is *not* applied—compare raw string; empty means `""` only) | **`indeterminate`** | Insufficient offered material to verify the assertion |
| `evidence.content.body` is **non-empty** and **equals** `claim.assertion.body` | **`satisfied`** | Offered content matches asserted payload under the minimal rule |
| `evidence.content.body` is **non-empty** and **does not equal** `claim.assertion.body` | **`not_satisfied`** | Offered content contradicts asserted payload under the minimal rule |

**Preconditions checked before rule application:**

| Check | Failure outcome | Reason |
|-------|-----------------|--------|
| `evidence.claim_id` ≠ `claim.id` | **`indeterminate`** | Evidence is not linked to the claim under evaluation |
| `EvaluationContext` missing required parts | **Error before evaluate** | Builder/`EvaluationContext` assembly responsibility—not interpreter semantics |

Evidence is **required** in `EvaluationContext`; “missing evidence” in fixtures means **empty `content.body`**, not an absent evidence envelope.

### Outcome vocabulary

Outcomes use **`vp-reference-model::Outcome`** only:

- `Outcome::Satisfied`
- `Outcome::NotSatisfied`
- `Outcome::Indeterminate`

No other labels. Informal words like “pass” or “fail” may appear in **trace messages** only—they must not appear as alternate outcome enums.

### VerificationResult production

`evaluate` returns a **frozen** **`VerificationResult`** built via **`VerificationResultBuilder`**:

| Field | Source |
|-------|--------|
| **`evaluated_claim_id`** | `context.claim().id` |
| **`outcome`** | Result of minimal rule |
| **`trace`** | See [Trace production](#trace-production) |
| **`metadata`** | Empty unless caller attaches runner metadata later (interpreter does not invent normative metadata) |
| **`specification_binding`** | Derived from `context.specification()` — `edition_id` and `protocol_version` when present |
| **`reasons`** | At least one human-readable string summarizing the outcome (e.g. “assertion body matches evidence content body”) |

The interpreter **does not** mutate `Claim`, `Evidence`, or `EvaluationContext`.

### Trace production

When **`context.options().trace_enabled`** is `true`:

1. Append a **start** event: evaluation began for `claim.id`
2. Append a **rule** event: `vp-ref-minimal.body-equality` applied to `assertion.body` and `content.body`
3. Append an **outcome** event: final outcome label (`satisfied` / `not_satisfied` / `indeterminate`)

When **`trace_enabled`** is `false`, return an **empty** `Trace`.

Each **`TraceEvent`** includes:

| Field | Minimal milestone value |
|-------|-------------------------|
| **`id`** | Stable sequential ids (`evt-1`, `evt-2`, …) within the run |
| **`rule_reference`** | Set on the rule event only (`vp-ref-minimal.body-equality`) |
| **`message`** | Short explanatory text (not normative) |
| **`metadata`** | Empty |

Trace is **explanatory**. Outcome alone is normative ([ADR-0003](0003-domain-model-architecture.md)).

When **`context.options().deterministic`** is `true`, trace event order and ids must be **stable across repeated runs** with identical input.

---

## Fixture examples

These fixtures prove the pipeline; they are **not** normative protocol scenarios.

| # | `assertion.body` | `content.body` | `claim_id` vs `evidence.claim_id` | Outcome |
|---|------------------|----------------|-----------------------------------|---------|
| 1 | `"alpha"` | `"alpha"` | match | `satisfied` |
| 2 | `"alpha"` | `"beta"` | match | `not_satisfied` |
| 3 | `"alpha"` | `""` | match | `indeterminate` |
| 4 | `"alpha"` | `"alpha"` | mismatch | `indeterminate` |

---

## Explicitly out of scope

The minimal rule **does not** implement:

| Area | Detail |
|------|--------|
| **Normative spec rules** | No rules from `BEHAVIOR_MODEL`, `DOMAIN_MODEL`, or architecture documents |
| **Specification-driven rule selection** | `SpecificationContext` is bound on the result but does not drive logic |
| **`assertion_type` / `content_type`** | Ignored in Milestone D minimal evaluation |
| **Cryptographic verification** | No signatures, hashes, or attestation checks |
| **Payment-domain semantics** | No amount, currency, party, or settlement logic |
| **Multiple assertions per claim** | Single assertion per claim only |
| **Claim or evidence parsing** | Callers supply built domain objects |
| **CLI commands** | No new `vp-reference` subcommands in Milestone D |
| **Report formatting** | `vp-reference-report` unchanged |
| **Conformance runner integration** | Milestone G |
| **Network, filesystem, or live evidence fetch** | Interpreter remains filesystem-agnostic |
| **Full trace schema** | Milestone F may expand event types; minimal trace suffices for D |

---

## Alternatives considered

### 1. Defer evaluation until spec-derived rules exist

Wait for typed rule extraction from `veritypay-spec` before any interpreter code.

**Rejected.** Without a minimal executable slice, the pipeline (`EvaluationContext` → interpreter → `VerificationResult`) cannot be tested incrementally. Risk of a large, late integration failure.

### 2. Hard-code outcome in CLI without interpreter crate

Return a fixed `satisfied` from the CLI for demos.

**Rejected.** Violates [ADR-0002](0002-workspace-architecture.md) crate boundaries and proves nothing about evaluation flow.

### 3. Use specification document text as the rule source

Parse loaded spec prose or registry entries to decide the first rule.

**Rejected for Milestone D.** Couples first evaluation to spec corpus shape before rule infrastructure exists. Specification binding on the result is sufficient for pipeline proof.

### 4. Treat body mismatch as `indeterminate`

Map all non-matches to insufficient information.

**Rejected.** Distinguishes **`not_satisfied`** (evidence offered but contradicts assertion) from **`indeterminate`** (no usable evidence content)—a minimal but useful split aligned with [VP-TERM-011](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/00-overview/GLOSSARY.md#verification-outcome).

---

## Consequences

### Positive

- **Pipeline proof** — first runnable `evaluate(context)` with stable inputs and outputs
- **Testable fixtures** — builders + rule table above yield clear unit tests without parsers
- **Outcome vocabulary exercise** — all three normative outcomes reachable in fixtures
- **Trace hook** — `trace_enabled` path validated before Milestone F depth

### Negative

- **Misinterpretation risk** — readers may confuse body equality with real VerityPay verification
- **Temporary semantics** — must be removed or clearly gated when real rules arrive
- **Spec context unused** — `SpecificationContext` carried but not consulted; may look like dead weight until later milestones

**Acceptable** because the ADR states loudly that this rule is **engineering scaffolding**, not protocol definition.

---

## Future

| Topic | Direction |
|-------|-----------|
| **Rule registry** | Replace hard-coded minimal rule with spec-derived or ADR-defined rule sets |
| **Supersession** | When real semantics land, ADR-0005+ or supersession of this ADR retires `vp-ref-minimal.body-equality` |
| **Milestone E alignment** | Formal conformance alignment with [CONFORMANCE_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md) may refine reasons and binding |
| **Milestone F** | Richer trace events, export formats, golden files |

---

## Related decisions

| Document | Relationship |
|----------|--------------|
| [ADR-0003](0003-domain-model-architecture.md) | Domain types, `EvaluationContext`, builders |
| [ADR-0002](0002-workspace-architecture.md) | Interpreter crate ownership and filesystem rule |
| [ROADMAP.md](../../ROADMAP.md) | Milestone D implements this ADR |
| [ARCHITECTURE.md](../../ARCHITECTURE.md) | Long-term verification component model |

---

## Follow-up

- [ ] Implement `Interpreter::evaluate` in `vp-reference-interpreter` per this ADR (Milestone D)
- [ ] Add unit tests for fixture table above
- [ ] Document in code that `vp-ref-minimal.body-equality` is temporary reference semantics

---

## Conclusion

The first reference interpreter evaluation is a **deliberately minimal, fixture-driven body-equality rule** over **`Assertion`** and **`EvidenceContent`**. It consumes **`EvaluationContext`**, produces **`VerificationResult`** with optional trace, and exercises the full outcome vocabulary—**without claiming to implement VerityPay protocol verification**.

This ADR records evaluation semantics only. It does **not** implement code or alter normative specification text.
