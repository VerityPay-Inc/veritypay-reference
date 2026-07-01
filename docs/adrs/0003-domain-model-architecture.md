---
id: ADR-0003
title: Domain Model Architecture
status: accepted
version: 1.0.0
authors:
  - VerityPay Core Team
reviewers: []
related_docs:
  - docs/adrs/0002-workspace-architecture.md
  - ARCHITECTURE.md
  - ROADMAP.md
decision_date: 2026-07-02
superseded_by: null
---

# ADR-0003 — Domain Model Architecture

**Status:** Accepted · **Version:** 1.0.0 · **Date:** 2026-07-02

**Related:** [ADR-0002](0002-workspace-architecture.md) · [ARCHITECTURE.md](../../ARCHITECTURE.md) · [ROADMAP.md](../../ROADMAP.md) · [veritypay-spec — GLOSSARY (VP-TERM-011)](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/00-overview/GLOSSARY.md#verification-outcome) · [DATA_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/01-architecture/DATA_MODEL.md)

---

## Purpose

Define the **core domain objects** used by the VerityPay reference interpreter before Milestone C implementation begins.

---

## Context

`veritypay-reference` can now load a validated specification checkout through **`vp-spec-model`** and expose a path-free **`SpecificationContext`** ([Milestone B](../../ROADMAP.md)).

The next milestone introduces **claim** and **evidence** domain types before interpretation. Those types must be:

- **Explicit** — reviewers can read struct fields and enums without hunting through parser modules
- **Readable** — the model documents protocol concepts, not Rust convenience
- **Immutable where practical** — inputs do not mutate during evaluation; results freeze after evaluation
- **Aligned with `veritypay-spec`** — normative vocabulary and relationships come from accepted documents, not this ADR

[ADR-0002](0002-workspace-architecture.md) assigned **`vp-reference-model`** as the home for pure domain types. This ADR records **what those types are**, how they relate, and what must never enter the crate.

---

## Domain pyramid

Evaluation flows downward through loaded specification context into immutable inputs, then through interpretation into a frozen result:

```
SpecificationContext          (vp-reference-core — path-free loaded spec view)
        ↓
Claim                           (vp-reference-model)
        ↓
Evidence                        (vp-reference-model)
        ↓
Evaluation                      (vp-reference-interpreter — operation, not a domain root)
        ↓
VerificationResult              (vp-reference-model)
        ├── Outcome
        ├── Trace
        └── Metadata
```

**`SpecificationContext`** lives in **`vp-reference-core`** because it is a loaded execution contract, not a claim or evidence payload. **`Evaluation`** is the interpreter operation that applies normative rules; it is not modeled as a persistent domain root in Milestone C. **`VerificationResult`** is the stable output surface for CLI, reports, conformance runners, and future SDKs.

---

## Decision

**Adopt a pure domain model in `vp-reference-model`.**

Define these objects:

### 1. Claim

Represents a **verifiable protocol claim** loaded or constructed for evaluation.

| Requirement | Detail |
|-------------|--------|
| **Scope** | Must not assume payment-specific fields beyond the **minimal accepted fixture** for Milestone C |
| **Future fields** | Claim id, claim type, subject, asserted content, specification version binding, metadata |
| **Ownership** | Defined in `vp-reference-model`; constructed by parsers or test fixtures—not by the interpreter |

A claim is **input** to evaluation. It does not embed verification verdicts.

### 2. Evidence

Represents **material offered to support or challenge a claim**.

| Requirement | Detail |
|-------------|--------|
| **Separation** | Must remain distinct from the claim—evidence is never nested as “part of” claim semantics |
| **Future fields** | Evidence id, evidence type, claim reference, content, metadata |
| **Ownership** | Defined in `vp-reference-model`; parsers and fixtures construct instances |

Evidence references a claim; it does not replace or mutate claim content.

### 3. Outcome

The **normative verification outcome** per [VP-TERM-011](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/00-overview/GLOSSARY.md#verification-outcome):

| Value | Meaning |
|-------|---------|
| **`satisfied`** | Rules passed under declared evidence and specification binding |
| **`not_satisfied`** | Rules failed under declared evidence and specification binding |
| **`indeterminate`** | Outcome cannot be determined under rules (e.g. missing required evidence—not surety gap) |

**No other protocol outcome labels** are permitted in `vp-reference-model` unless accepted upstream in `veritypay-spec`. Informal labels such as pass/fail must map to this vocabulary at report boundaries—not as alternate normative enums.

### 4. VerificationResult

The **root result object** returned by the interpreter.

| Field (conceptual) | Role |
|------------------|------|
| **`outcome`** | Normative `Outcome` |
| **`trace`** | Explanatory evaluation record |
| **`metadata`** | Non-normative context |
| **Specification version or edition binding** | Records which spec pin governed evaluation |
| **Evaluated claim id** | Links result to the claim under test |
| **Optional reasons** | Human- or machine-readable rationale strings where milestones support them |

This is the object future **CLI**, **reports**, **conformance**, and **SDKs** compare or serialize. It aggregates outcome, trace, and metadata—it does not re-derive them from side channels.

### 5. Trace

An **ordered explanation** of how evaluation proceeded.

| Requirement | Detail |
|-------------|--------|
| **Normative status** | Trace is **explanatory**, not normative—outcome alone decides protocol truth |
| **Future contents** | Ordered trace events with rule references, messages, optional input references |
| **Lifecycle** | Built during evaluation, then **frozen** as part of `VerificationResult` |

Trace events may cite specification rule identifiers; they must not introduce alternate outcome vocabulary.

### 6. Metadata

**Non-normative context** attached to claims, evidence, and results.

| Requirement | Detail |
|-------------|--------|
| **Examples** | Timestamps, runner labels, fixture names, editor hints, scenario tags |
| **Constraint** | Metadata **must never decide protocol truth**—only `Outcome` does |
| **Placement** | May appear on inputs and on `VerificationResult`; kept separate from normative fields |

### 7. Identifiers

**Stable identifiers** for claim, evidence, trace events, specification version, and rule references.

| Requirement | Detail |
|-------------|--------|
| **Explicit types** | Prefer newtypes or dedicated identifier structs over raw `String` everywhere |
| **Conformance** | Identifiers must compare predictably across runs and implementations |
| **Source of truth** | Identifier *meaning* comes from `veritypay-spec`; identifier *representation* is engineering choice in this crate |

---

## Domain rules

| Rule | Detail |
|------|--------|
| **`vp-reference-model` is pure domain** | No I/O, no loading, no evaluation |
| **No filesystem types** | No `Path`, `PathBuf`, or manifest paths |
| **No parser errors** | Parse failures belong in future parser modules or `vp-reference-core` input errors |
| **No CLI types** | No `clap` structs or exit-code mapping |
| **No report formatting** | No pretty-printing, JSON serializers tied to CLI, or terminal styling |
| **No specification loading** | No `vp-spec-model` or `SpecRepository` |
| **No verification logic** | Rules live in `vp-reference-interpreter` |
| **Manual test construction** | Every domain object must be constructible in unit tests without parsing |
| **Interpreter consumes; model defines** | The interpreter takes model types as input/output—it does not own their definitions |
| **Report formats; model stays immutable** | `vp-reference-report` reads model types; it never mutates them |

These rules extend the domain purity rule in [ADR-0002](0002-workspace-architecture.md).

---

## Immutability

| Object | Immutability expectation |
|--------|--------------------------|
| **`Claim`** | Immutable input during evaluation—the claim under test does not change when rules run |
| **`Evidence`** | Immutable input during evaluation—evidence offered for the run is fixed at evaluation start |
| **`Trace`** | Mutable **during** evaluation as events append; **immutable** once embedded in `VerificationResult` |
| **`VerificationResult`** | Immutable after evaluation completes—reports and conformance read a frozen snapshot |
| **`Metadata`** | May be attached at construction; must not be used to retroactively alter normative outcome |

Rust ownership should enforce these expectations where practical (`Clone` for snapshots, no public mutation on result types after construction).

---

## Alternatives considered

### 1. Minimal structs only as needed

Add fields incrementally per milestone without an upfront domain map.

**Rejected for Milestone C prep.** Ad-hoc growth produces ambiguous ownership (parser vs interpreter vs report), unstable conformance surfaces, and repeated refactors when trace and outcome milestones arrive. A documented pyramid is cheap now and prevents coupling mistakes.

### 2. JSON-like dynamic values

Represent claims, evidence, and results as generic maps or `serde_json::Value`.

**Rejected.** Dynamic values hide protocol structure, weaken type-checked conformance comparison, and encourage normative fields to appear without spec review. Acceptable only at **report serialization boundaries**—not as the internal domain model.

### 3. Parser-owned domain objects

Define `Claim` and `Evidence` inside future parser modules; export opaque handles to the interpreter.

**Rejected.** Couples evaluation to parse pipeline layout, complicates library use from `veritypay-conformance`, and prevents hand-built fixtures in `vp-reference-model` tests. Parsers **produce** model types; they do not **own** them.

### 4. Interpreter-owned result types

Define `VerificationResult`, `Outcome`, and `Trace` inside `vp-reference-interpreter`.

**Rejected.** Reporting and conformance must consume results without depending on interpreter internals ([ADR-0002](0002-workspace-architecture.md) dependency graph). Result types are the **stable public output contract** and belong in the leaf model crate.

---

## Consequences

### Positive

- **Readable semantics** — reviewers read `vp-reference-model` to learn what the interpreter accepts and returns
- **Stable conformance surface** — `VerificationResult` and `Outcome` become the comparison anchor for VP-CS scenarios
- **Easier reporting** — `vp-reference-report` formats known types without re-running evaluation
- **Clearer tests** — fixtures construct domain objects directly; no parser required for interpreter unit tests
- **Less parser/interpreter coupling** — parse errors and domain shapes stay in separate crates

### Negative

- **More upfront modeling** — Milestone C must align parsers with declared shapes instead of “whatever parses”
- **Types may evolve** — as `veritypay-spec` matures, fields and identifier types may require additive ADR or version notes
- **Discipline required** — contributors must resist placing convenience helpers, serialization, or rule logic in the model crate

**Acceptable** because explicit domain boundaries are cheaper before claim parsing lands than after evaluation and trace code entangle field definitions.

---

## Future

| Topic | Direction |
|-------|-----------|
| **`EvaluationGraph`** | May be introduced later if multiple interpreter stages need shared evaluation state ([ADR-0002](0002-workspace-architecture.md)). Not part of Milestone C domain roots. |
| **Claim and evidence subtypes** | Added only when accepted spec documents require them—no speculative payment-domain fields |
| **Serialization contracts** | Stabilized when conformance integration (Milestone G) needs reproducible JSON or artifact formats; internal model may precede public schema |
| **Full identifier newtypes** | Introduced incrementally as fixture and rule-reference needs clarify comparison requirements |

---

## Related decisions

| Document | Relationship |
|----------|--------------|
| [ADR-0002](0002-workspace-architecture.md) | Crate ownership of model vs interpreter vs report |
| [ARCHITECTURE.md](../../ARCHITECTURE.md) | Component pipeline this domain pyramid implements |
| [ROADMAP.md](../../ROADMAP.md) | Milestone C implements claim/evidence against this model |
| [veritypay-spec — DATA_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/01-architecture/DATA_MODEL.md) | Normative claim/evidence relationships |
| [veritypay-spec — CONFORMANCE_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md) | Outcome vocabulary and verification flow |

---

## Follow-up

- [ ] Implement domain types in `vp-reference-model` per this ADR (Milestone C — separate PR)
- [ ] Bind minimal claim fixture fields to accepted `DATA_MODEL` subset
- [ ] Add unit tests constructing `Claim`, `Evidence`, and placeholder `VerificationResult` without parsers

---

## Conclusion

The reference interpreter needs a **pure, explicit domain model** before claim parsing begins. **`vp-reference-model`** owns **`Claim`**, **`Evidence`**, **`Outcome`**, **`Trace`**, **`Metadata`**, **`Identifiers`**, and **`VerificationResult`**. The interpreter evaluates; the report formats; the model defines what verification **means** as data—without deciding protocol truth outside the normative **`Outcome`** enum accepted in `veritypay-spec`.

This ADR records domain architecture only. It does **not** implement code or alter normative specification text.
