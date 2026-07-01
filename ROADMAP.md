# Roadmap

**Capability-based roadmap for `veritypay-reference`.**

This roadmap is **not date-driven**. Milestones complete when their success criteria are met—not when a quarter ends. Progress aligns with [Phase II Platform Plan](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/05-governance/PHASE_II_PLATFORM_PLAN.md) and the reference interpreter role defined in [CONFORMANCE_MODEL.md](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md).

**Current milestone:** **C — Domain model and builders** *(complete)*

---

## Overview

| Milestone | Name | Status |
|-----------|------|--------|
| **A** | Repository scaffold | **Complete** |
| **B** | Load specification model | **Complete** |
| **C** | Domain model and builders | **Complete** |
| **D** | Evaluate minimal claim | Not started |
| **E** | Produce verification outcome | Not started |
| **F** | Produce trace | Not started |
| **G** | Conformance integration | Not started |

Each milestone below includes **Goal**, **Outputs**, **Success criteria**, and **Not included** so scope stays explicit.

---

## Milestone A — Repository scaffold

**Goal:** Establish `veritypay-reference` as a mature **engineering project** before interpreter code exists—clear purpose, architecture, contribution rules, and boundaries with `veritypay-spec` and `veritypay-tooling`.

**Outputs:**

- [README.md](README.md) — purpose, boundaries, links to sibling repos
- [ARCHITECTURE.md](ARCHITECTURE.md) — component model (conceptual; no implementation language)
- [ROADMAP.md](ROADMAP.md) — this document with milestones A–G
- [CONTRIBUTING.md](CONTRIBUTING.md) — contributor expectations and specification boundary
- [LICENSE](LICENSE) — license terms
- [docs/adrs/0001-reference-implementation-language.md](docs/adrs/0001-reference-implementation-language.md) — ADR-0001: Rust (Accepted)
- [docs/adrs/0002-workspace-architecture.md](docs/adrs/0002-workspace-architecture.md) — ADR-0002: Cargo workspace (Accepted)
- Cargo workspace per ADR-0002 (`vp-reference-*` crates)
- `.github/workflows/ci.yml` — fmt, clippy, test
- Repository maturity declared: **Workspace bootstrap**

**Success criteria:**

- [x] A new contributor can explain what the reference interpreter does and does not do in five minutes
- [x] Dependency on `veritypay-spec` is explicit and one-directional
- [x] Relationship to `veritypay-tooling` and `vp-spec-model` is documented
- [x] Milestones B–G each define goal, outputs, success criteria, and not-included scope
- [x] No interpreter logic merged under the pretense of "early MVP"
- [x] Cargo workspace compiles; crate boundaries match ADR-0002
- [x] CI runs fmt, clippy, and tests

**Not included:**

- Claim or evidence parsing
- Verification outcomes or traces (Milestones E–F)
- Changes to normative text in `veritypay-spec`

---

## Milestone B — Load specification model

**Goal:** Load validated specification input through the shared typed model layer.

**Prerequisite:** `veritypay-tooling` readiness gate passed; `vp-spec-model` stable for v1 ([ADR-0007](https://github.com/VerityPay-Inc/veritypay-tooling/blob/main/docs/adrs/0007-specification-model-stability.md)).

**Outputs:**

- [docs/adrs/0002-workspace-architecture.md](docs/adrs/0002-workspace-architecture.md) — ADR-0002 (Accepted)
- `vp-reference-spec`: `SpecificationLoader`, `SpecificationLoadOptions`, `LoadedSpecification` via `vp-spec-model`
- `vp-reference-core`: path-free `SpecificationContext` with loaded summary counts
- CLI: `vp-reference load-spec --spec <path>`
- Fixture and optional sibling `veritypay-spec` integration tests

**Success criteria:**

- [x] Can load registries and document corpus from a validated spec tree
- [x] Specification version or Edition identifier is recorded on the loaded context (optional `edition_id` / `protocol_version` pins)
- [x] Missing or invalid spec input fails with a clear error—not silent partial load
- [x] No duplicate registry or document parsing logic where `vp-spec-model` suffices

**Not included:**

- Claim or evidence parsing (Milestone C)
- Verification or outcome production (Milestones D–E)
- Edition Manifest typed model (may use path pins until available in `vp-spec-model`)

---

## Milestone C — Domain model and builders

**Goal:** Establish **claim** and **evidence** domain types—with **identity separated from content**—and **builders** for readable fixtures before parsing or evaluation.

**Prerequisite:** [ADR-0003](docs/adrs/0003-domain-model-architecture.md) — domain model architecture (Accepted, v1.1.0).

### Milestone C.1 — Domain types and builders *(complete)*

Pure domain types and builders in `vp-reference-model` per ADR-0003 v1.1.0. No parsers, `EvaluationContext` expansion, or verification logic.

**Success criteria (C.1):**

- [x] Domain types match ADR-0003 identity/content split (`Assertion`, `EvidenceContent`, envelopes, identifiers)
- [x] `ClaimBuilder`, `EvidenceBuilder`, `TraceBuilder`, `VerificationResultBuilder` construct readable fixtures
- [x] No normative fields beyond minimal generic assertion/evidence content
- [x] No file or JSON parsers

### Milestone C.2 — EvaluationContext *(complete)*

Path-free interpreter input contract in `vp-reference-core` per ADR-0003 v1.1.0.

**Success criteria (C.2):**

- [x] `EvaluationOptions` with `deterministic` and `trace_enabled` defaults
- [x] `EvaluationContext` bundles specification, claim, evidence, and options
- [x] `EvaluationContextBuilder` with clear errors for missing required fields
- [x] No filesystem types in the evaluation input contract

### Milestone C — complete

**Outputs:**

- [docs/adrs/0003-domain-model-architecture.md](docs/adrs/0003-domain-model-architecture.md) — ADR-0003: domain pyramid, `EvaluationContext`, builders (Accepted)
- `Claim`, `Assertion`, `Evidence`, `EvidenceContent` in `vp-reference-model` *(C.1)*
- `ClaimBuilder`, `EvidenceBuilder`, `VerificationResultBuilder`, `TraceBuilder` *(C.1)*
- `EvaluationContext`, `EvaluationOptions`, `EvaluationContextBuilder` in `vp-reference-core` *(C.2)*
- Unit tests constructing evaluation inputs via builders *(C.1–C.2)*

**Success criteria:**

- [x] Domain types match ADR-0003 identity/content split
- [x] Builders construct minimal claim and evidence fixtures without sprawling struct literals
- [x] `EvaluationContext` bundles specification, claim, and evidence for a single evaluation
- [x] No normative claim or evidence fields invented beyond accepted spec documents
- [x] No file or JSON parsers required for this milestone phase

**Not included:**

- Claim or evidence **parsing** (follow-on within Milestone C or early Milestone D)
- Verification logic (Milestone D)
- Full claim type coverage
- Network or file watchers for live claim intake

---

## Milestone D — Evaluate minimal claim

**Goal:** Run the **interpreter** against a minimal claim under the first executable rule set.

**Prerequisite:** [ADR-0004](docs/adrs/0004-minimal-evaluation-semantics.md) — minimal evaluation semantics (Accepted).

**Outputs:**

- [docs/adrs/0004-minimal-evaluation-semantics.md](docs/adrs/0004-minimal-evaluation-semantics.md) — ADR-0004: body-equality rule, `evaluate(context)` contract (Accepted)
- `Interpreter::evaluate(&EvaluationContext) -> VerificationResult` in `vp-reference-interpreter`
- Fixture-driven unit tests per ADR-0004 outcome table

**Success criteria:**

- [ ] `evaluate` consumes `EvaluationContext` and returns frozen `VerificationResult`
- [ ] Minimal rule outcomes match ADR-0004 (`satisfied` / `not_satisfied` / `indeterminate`)
- [ ] Trace events emitted when `trace_enabled`; empty trace when disabled
- [ ] `SpecificationContext` bound on result; rule does not require manual spec reading
- [ ] Code remains readable; performance is not a success criterion

**Not included:**

- Normative spec rules from architecture documents
- Full architecture model coverage (domain, behavior, state—expanded incrementally)
- Claim or evidence parsing
- CLI or report changes
- VP-CS runner integration (Milestone G)

---

## Milestone E — Produce verification outcome

**Goal:** Emit normative **verification outcomes** as defined by the conformance model.

**Outputs:**

- Outcome component: `satisfied`, `not_satisfied`, `indeterminate`
- `verify(claim, evidence, spec_version)` alignment with [CONFORMANCE_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md)
- Stable outcome record for downstream comparison

**Success criteria:**

- [ ] At least one positive fixture yields `satisfied`
- [ ] At least one negative fixture yields `not_satisfied`
- [ ] Insufficient-evidence fixture yields `indeterminate` when spec requires it
- [ ] Outcome is bound to specification version used at evaluation time
- [ ] No non-normative outcome labels introduced

**Not included:**

- Full trace (Milestone F)
- Conformance suite orchestration (Milestone G)
- Cross-implementation interoperability testing (owned by `veritypay-conformance`)

---

## Milestone F — Produce trace

**Goal:** Explain **how** the interpreter reached an outcome.

**Outputs:**

- Trace component recording ordered evaluation steps
- Report component summarizing outcome + trace for human review
- References to spec sections or VP-TERM IDs where applicable

**Success criteria:**

- [ ] Every outcome in Milestone E fixtures includes a trace
- [ ] Trace steps are ordered and human-readable
- [ ] Trace does not alter outcome—explanation only
- [ ] Reviewer can follow claim → rules applied → outcome without reading source first

**Not included:**

- Machine-readable trace protocol for production systems (unless promoted upstream)
- Conformance diff tooling (Milestone G / `veritypay-conformance`)
- Performance profiling of trace capture

---

## Milestone G — Conformance integration

**Goal:** Provide **hooks for VP-CS runners** in `veritypay-conformance`.

**Outputs:**

- Stable entrypoint for scenario invocation (CLI or library—via ADR)
- Machine-readable outcome + trace export for CI
- Documentation for running reference oracle against VP-CS fixtures
- Alignment with at least one VP-CS scenario from `veritypay-spec`

**Success criteria:**

- [ ] `veritypay-conformance` (or equivalent harness) can invoke the reference interpreter without fork-specific glue
- [ ] VP-CS-0001 or agreed minimal scenario produces expected outcome
- [ ] Exit codes or structured results suitable for CI
- [ ] Reference behavior documented as oracle—not mandatory production dependency

**Not included:**

- Authoring VP-CS scenario text (remains in `veritypay-spec`)
- Certification, badges, or vendor programs
- Production deployment or SDK packaging
- Full VP-CS catalog coverage (incremental)

---

## After Milestone G

The reference interpreter enters **maintenance and extension** mode: broader claim types, deeper architecture model coverage, and Edition-aware evaluation as spec governance defines them.

**Explicitly deferred:**

- SDK or integrator API surface
- Blockchain adapters and chain-specific execution
- Production performance optimization
- Replacing independent implementations

---

## How to propose roadmap changes

Roadmap changes are **reference governance**, not protocol changes.

1. Open an issue describing capability gap and proposed milestone adjustment
2. For structural evaluation contracts or public API changes, write an ADR in this repository
3. If semantics imply **new normative spec requirements**, propose RFC in `veritypay-spec` first

---

*Capability before calendar. Readable semantics before performance.*
