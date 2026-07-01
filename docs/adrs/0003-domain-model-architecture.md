---
id: ADR-0003
title: Domain Model Architecture
status: accepted
version: 1.1.0
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

**Status:** Accepted · **Version:** 1.1.0 · **Date:** 2026-07-02

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

Evaluation is invoked with a single **`EvaluationContext`**. The interpreter evaluates **assertions** using **evidence content**—not opaque claim envelopes alone.

```
EvaluationContext                 (vp-reference-core — single interpreter input)
├── SpecificationContext          (path-free loaded spec view)
├── Claim                         (vp-reference-model — identity + assertion)
│     ├── id, subject, metadata
│     └── Assertion               (payload being asserted)
├── Evidence                      (vp-reference-model — identity + content)
│     ├── id, claim reference, metadata
│     └── EvidenceContent         (material offered)
└── EvaluationOptions             (future — debug, conformance, timeout, …)

        ↓  interpreter.evaluate(context)

VerificationResult              (vp-reference-model)
├── Outcome
├── Trace
└── Metadata
```

**`SpecificationContext`** and **`EvaluationContext`** live in **`vp-reference-core`**. **`Evaluation`** is the interpreter operation (`evaluate(context)`); it is not a persistent domain root. **`VerificationResult`** is the stable output surface for CLI, reports, conformance runners, and future SDKs.

---

## Decision

**Adopt a pure domain model in `vp-reference-model`**, with **identity/content separation**, a **single evaluation input object**, and **builders before parsers**.

### Identity vs content

A **claim** is not the same thing as the **assertion** it carries. A **evidence record** is not the same thing as the **content** it carries. Conflating identity and payload makes multi-assertion claims and multi-piece evidence awkward and pushes verification logic toward the wrong abstraction.

| Envelope | Identity (who / which) | Content (what is asserted or offered) |
|----------|------------------------|---------------------------------------|
| **Claim** | id, subject, claim type, metadata | **`Assertion`** — the verifiable payload |
| **Evidence** | id, evidence type, claim reference, metadata | **`EvidenceContent`** — the supporting material |

The interpreter applies normative rules to **`Assertion`** in light of **`EvidenceContent`**. Claim and evidence envelopes provide stable references and non-normative context; they are not substitutes for the content under evaluation.

This split pays off when:

- A claim carries **multiple assertions** (each evaluated or traced separately)
- Evidence bundles **multiple content pieces** tied to one evidence id
- Conformance compares outcomes by **assertion id** without reshaping claim envelopes

---

### Domain objects

#### 1. Claim

Represents a **verifiable protocol claim**—identity and binding—not the assertion body alone.

| Field (conceptual) | Role |
|--------------------|------|
| **id** | Stable claim identifier |
| **subject** | Who or what the claim is about |
| **assertion** | **`Assertion`** — content being asserted |
| **metadata** | Non-normative context |

| Requirement | Detail |
|-------------|--------|
| **Scope** | Must not assume payment-specific fields beyond the **minimal accepted fixture** for Milestone C |
| **Ownership** | Defined in `vp-reference-model`; built via **`ClaimBuilder`** or future parsers—not by the interpreter |

A claim is **input** to evaluation. It does not embed verification verdicts.

#### 2. Assertion

Represents the **payload being asserted**—what verification rules actually evaluate.

| Requirement | Detail |
|-------------|--------|
| **Separation** | Distinct from claim identity; a claim references exactly one assertion in the minimal milestone, more later |
| **Future fields** | Assertion type, structured asserted fields per accepted `DATA_MODEL` subset |
| **Evaluation target** | Interpreter rules target **assertions**, not claim metadata |

#### 3. Evidence

Represents **material offered to support or challenge a claim**—identity and binding—not the bytes or structured content alone.

| Field (conceptual) | Role |
|--------------------|------|
| **id** | Stable evidence identifier |
| **claim reference** | Links evidence to the claim under test |
| **content** | **`EvidenceContent`** — the offered material |
| **metadata** | Non-normative context |

| Requirement | Detail |
|-------------|--------|
| **Separation** | Must remain distinct from the claim—evidence is never nested as “part of” claim semantics |
| **Ownership** | Defined in `vp-reference-model`; built via **`EvidenceBuilder`** or future parsers |

Evidence references a claim; it does not replace or mutate claim content.

#### 4. EvidenceContent

Represents the **material offered**—what rules consume when checking an assertion.

| Requirement | Detail |
|-------------|--------|
| **Separation** | Distinct from evidence identity; supports multiple content pieces under one evidence id later |
| **Evaluation input** | Paired with **`Assertion`** during rule application |

#### 5. Outcome

The **normative verification outcome** per [VP-TERM-011](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/00-overview/GLOSSARY.md#verification-outcome):

| Value | Meaning |
|-------|---------|
| **`satisfied`** | Rules passed under declared evidence and specification binding |
| **`not_satisfied`** | Rules failed under declared evidence and specification binding |
| **`indeterminate`** | Outcome cannot be determined under rules (e.g. missing required evidence—not latency) |

**No other protocol outcome labels** are permitted in `vp-reference-model` unless accepted upstream in `veritypay-spec`. Informal labels such as pass/fail must map to this vocabulary at report boundaries—not as alternate normative enums.

#### 6. VerificationResult

The **root result object** returned by the interpreter.

| Field (conceptual) | Role |
|--------------------|------|
| **`outcome`** | Normative `Outcome` |
| **`trace`** | Explanatory evaluation record |
| **`metadata`** | Non-normative context |
| **Specification version or edition binding** | Records which spec pin governed evaluation |
| **Evaluated claim id** | Links result to the claim under test |
| **Optional reasons** | Human- or machine-readable rationale strings where milestones support them |

Built via **`VerificationResultBuilder`** during evaluation, then frozen. This is the object future **CLI**, **reports**, **conformance**, and **SDKs** compare or serialize.

#### 7. Trace

An **ordered explanation** of how evaluation proceeded.

| Requirement | Detail |
|-------------|--------|
| **Normative status** | Trace is **explanatory**, not normative—outcome alone decides protocol truth |
| **Future contents** | Ordered trace events with rule references, messages, optional assertion/evidence references |
| **Lifecycle** | Appended during evaluation via **`TraceBuilder`**, then **frozen** inside `VerificationResult` |

#### 8. Metadata

**Non-normative context** attached to claims, evidence, and results.

| Requirement | Detail |
|-------------|--------|
| **Examples** | Timestamps, runner labels, fixture names, scenario tags |
| **Constraint** | Metadata **must never decide protocol truth**—only `Outcome` does |

#### 9. Identifiers

**Stable identifiers** for claim, evidence, assertion, trace events, specification version, and rule references.

Prefer explicit newtypes over raw `String` where comparison and conformance matter.

---

### EvaluationContext

The interpreter accepts **one input object**, not a growing parameter list.

```rust
// Conceptual API — not implemented in this ADR
interpreter.evaluate(&EvaluationContext) -> VerificationResult
```

**`EvaluationContext`** (`vp-reference-core`) bundles everything required for one evaluation run:

| Field | Role |
|-------|------|
| **`specification`** | Loaded **`SpecificationContext`** |
| **`claim`** | **`Claim`** under evaluation |
| **`evidence`** | **`Evidence`** offered for the run |
| **`options`** | **`EvaluationOptions`** (future) — debug mode, deterministic mode, conformance mode, cancellation, timeout policy, … |

**Why one object:** evaluation knobs accumulate. A single context keeps the interpreter entrypoint stable while options expand—callers (CLI, conformance, tests) construct one struct instead of threading positional arguments through every milestone.

The CLI loads specification input via **`vp-reference-spec`**, constructs **`Claim`** and **`Evidence`** via builders (or parsers later), assembles **`EvaluationContext`**, and calls **`interpreter.evaluate`**.

---

### Builders before parsers

**Milestone C implements builders before file or JSON parsers.**

| Builder | Purpose |
|---------|---------|
| **`ClaimBuilder`** | Construct `Claim` + nested `Assertion` for tests and fixtures |
| **`EvidenceBuilder`** | Construct `Evidence` + nested `EvidenceContent` |
| **`VerificationResultBuilder`** | Assemble frozen results in interpreter tests |
| **`TraceBuilder`** | Append trace events during evaluation |

Builders live in **`vp-reference-model`** (or builder modules colocated with domain types). They are **not** parser error types and **not** CLI concerns.

**Fixture ergonomics** — prefer:

```rust
Claim::builder()
    .id(claim_id)
    .subject(subject)
    .assertion(assertion)
    .build()
```

over large struct literals as the model grows.

**Parser target** — when claim/evidence parsing arrives, parsers map external input **onto builders** rather than constructing domain structs ad hoc. The builder API becomes the stable construction contract; parsers stay thin.

---

## Domain rules

| Rule | Detail |
|------|--------|
| **`vp-reference-model` is pure domain** | No I/O, no loading, no evaluation |
| **No filesystem types** | No `Path`, `PathBuf`, or manifest paths |
| **No parser errors in model** | Parse failures belong in future parser modules or `vp-reference-core` input errors |
| **No CLI types** | No `clap` structs or exit-code mapping |
| **No report formatting** | No pretty-printing or terminal styling |
| **No specification loading** | No `vp-spec-model` or `SpecRepository` |
| **No verification logic** | Rules live in `vp-reference-interpreter` |
| **Builders and manual construction** | Every domain object constructible via builders in unit tests without parsing |
| **Interpreter consumes; model defines** | The interpreter takes **`EvaluationContext`** and returns model outputs—it does not own domain definitions |
| **Report formats; model stays immutable** | `vp-reference-report` reads model types; it never mutates them |
| **Evaluate assertions, not envelopes** | Verification logic targets **`Assertion`** + **`EvidenceContent`**, not claim/evidence identity fields alone |

These rules extend the domain purity rule in [ADR-0002](0002-workspace-architecture.md).

---

## Immutability

| Object | Immutability expectation |
|--------|--------------------------|
| **`Claim` / `Assertion`** | Immutable input during evaluation |
| **`Evidence` / `EvidenceContent`** | Immutable input during evaluation |
| **`EvaluationContext`** | Immutable for the duration of one `evaluate` call |
| **`Trace`** | Mutable **during** evaluation via `TraceBuilder`; **immutable** once embedded in `VerificationResult` |
| **`VerificationResult`** | Immutable after evaluation completes |
| **`Metadata`** | Must not retroactively alter normative outcome |

Rust ownership should enforce these expectations where practical.

---

## Alternatives considered

### 1. Minimal structs only as needed

Add fields incrementally per milestone without an upfront domain map.

**Rejected for Milestone C prep.** Ad-hoc growth produces ambiguous ownership, unstable conformance surfaces, and repeated refactors when trace and outcome milestones arrive.

### 2. JSON-like dynamic values

Represent claims, evidence, and results as generic maps or `serde_json::Value`.

**Rejected.** Dynamic values hide protocol structure and weaken type-checked conformance comparison. Acceptable only at **report serialization boundaries**.

### 3. Parser-owned domain objects

Define `Claim` and `Evidence` inside future parser modules.

**Rejected.** Couples evaluation to parse layout and blocks hand-built fixtures. Parsers **target builders**; they do not **own** domain types.

### 4. Interpreter-owned result types

Define `VerificationResult`, `Outcome`, and `Trace` inside `vp-reference-interpreter`.

**Rejected.** Reporting and conformance must consume results without depending on interpreter internals.

### 5. Flat claim and evidence structs (identity + content combined)

Single struct per claim with id, subject, and asserted fields in one flat shape.

**Rejected for v1 modeling.** Works for one assertion per claim but entangles identity with payload. Becomes costly when claims carry multiple assertions or evidence carries multiple content pieces. **`Assertion`** and **`EvidenceContent`** are introduced now to avoid a breaking split later.

### 6. Multi-parameter interpreter API

`evaluate(spec, claim, evidence, options, …)` with positional arguments.

**Rejected.** Parameter lists grow with every milestone (debug, conformance, cancellation, timeout). **`EvaluationContext`** keeps **`interpreter.evaluate(context)`** stable.

### 7. Parsers before builders

Implement fixture file parsing as the first Milestone C code path.

**Deferred.** Parsing without a stable construction API produces one-off struct literals and parser-specific shapes. **Builders first** makes tests readable and gives parsers a single target.

---

## Consequences

### Positive

- **Readable semantics** — identity vs content matches how verification actually works
- **Stable interpreter API** — `evaluate(EvaluationContext)` absorbs future options without signature churn
- **Beautiful fixtures** — builders keep tests legible as fields grow
- **Parser-ready construction** — builders define the contract parsers implement later
- **Stable conformance surface** — `VerificationResult` and `Outcome` remain the comparison anchor
- **Less parser/interpreter coupling** — parse errors stay outside the model crate

### Negative

- **More upfront modeling** — `Assertion`, `EvidenceContent`, builders, and `EvaluationContext` before first parse milestone
- **Types may evolve** — additive fields as `veritypay-spec` matures
- **Discipline required** — resist putting rule logic or serialization inside builders

**Acceptable** because these patterns are cheaper to establish before Milestone C code than to refactor after evaluation and parsing entangle.

---

## Future

| Topic | Direction |
|-------|-----------|
| **`EvaluationOptions`** | Debug, deterministic, conformance, cancellation, timeout—added to `EvaluationContext` without API breakage |
| **`EvaluationGraph`** | Shared evaluation state across interpreter stages if needed ([ADR-0002](0002-workspace-architecture.md)) |
| **Multiple assertions per claim** | Natural extension once `Assertion` is a first-class object |
| **Serialization contracts** | Stabilized at conformance integration (Milestone G); builders precede public JSON schema |
| **Minimal claim/evidence parsers** | After builders land; map external fixtures onto builder APIs |

---

## Related decisions

| Document | Relationship |
|----------|--------------|
| [ADR-0002](0002-workspace-architecture.md) | Crate ownership; interpreter receives loaded context |
| [ARCHITECTURE.md](../../ARCHITECTURE.md) | Component pipeline this domain pyramid implements |
| [ROADMAP.md](../../ROADMAP.md) | Milestone C implements builders and domain types |
| [veritypay-spec — DATA_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/01-architecture/DATA_MODEL.md) | Normative claim/evidence relationships |
| [veritypay-spec — CONFORMANCE_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md) | Outcome vocabulary and verification flow |

---

## Follow-up

- [ ] Implement `Assertion`, `EvidenceContent`, and envelope types in `vp-reference-model`
- [ ] Implement `ClaimBuilder`, `EvidenceBuilder`, `VerificationResultBuilder`, `TraceBuilder`
- [ ] Expand `EvaluationContext` in `vp-reference-core` (spec + claim + evidence; options stub)
- [ ] Add unit tests constructing evaluation inputs via builders—no parsers required
- [ ] Bind minimal fixture fields to accepted `DATA_MODEL` subset when parsers follow

---

## Conclusion

The reference interpreter needs a **pure, explicit domain model** with **identity separated from content**, a **single evaluation input**, and **builders before parsers**. **`vp-reference-model`** owns claims, assertions, evidence, outcomes, traces, metadata, identifiers, and results. **`vp-reference-core`** owns **`SpecificationContext`** and **`EvaluationContext`**. The interpreter **`evaluate`s assertions using evidence content**; the report formats frozen **`VerificationResult`** values—without deciding protocol truth outside the normative **`Outcome`** enum accepted in `veritypay-spec`.

This ADR records domain architecture only. It does **not** implement code or alter normative specification text.
