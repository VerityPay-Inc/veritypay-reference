# Roadmap

**Capability-based roadmap for `veritypay-reference`.**

This roadmap is **not date-driven**. Milestones complete when their success criteria are met—not when a quarter ends. Progress aligns with [Phase II Platform Plan](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/05-governance/PHASE_II_PLATFORM_PLAN.md) and the reference interpreter role defined in [CONFORMANCE_MODEL.md](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md).

**Current milestone:** **D.8 — Platform 1.3 normalized text assertion** *(in progress)*

---

## Overview

| Milestone | Name | Status |
|-----------|------|--------|
| **A** | Repository scaffold | **Complete** |
| **B** | Load specification model | **Complete** |
| **C** | Domain model and builders | **Complete** |
| **D** | Evaluate minimal claim | **Complete** |
| **D.5** | Platform 1.2 model groundwork | **Complete** |
| **D.6** | Platform 1.2 multi-evidence execution | **Complete** |
| **D.7** | Assertion evaluator architecture | **Complete** |
| **D.8** | Platform 1.3 normalized text assertion | **Complete** *(implementation)* — VP-RFC-0011 acceptance still draft |
| **E** | Produce verification outcome | **Complete** |
| **F** | Produce trace | **Partial** — interpreter trace + CLI `--explain`; `vp-reference-report` crate placeholder |
| **G** | Conformance integration | **Complete** — `veritypay-conformance` invokes reference oracle |

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

**Prerequisite:** [ADR-0004](docs/adrs/0004-minimal-evaluation-semantics.md) — minimal evaluation semantics (Accepted, superseded by [VP-RFC-0001](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0001-minimal-claim-evidence-semantics.md) at D.3); [ADR-0005](docs/adrs/0005-evaluation-rule-architecture.md) — evaluation rule architecture (Accepted).

**Outputs:**

- [docs/adrs/0004-minimal-evaluation-semantics.md](docs/adrs/0004-minimal-evaluation-semantics.md) — ADR-0004: engineering scaffold for first rule (Accepted; normative semantics now **VP-RULE-0001** / **VP-RFC-0001**)
- [docs/adrs/0005-evaluation-rule-architecture.md](docs/adrs/0005-evaluation-rule-architecture.md) — ADR-0005: `EvaluationRule`, `VpRule0001` (Accepted)
- [docs/adrs/0006-ruleset-architecture.md](docs/adrs/0006-ruleset-architecture.md) — ADR-0006: `RuleSet` orchestration (Accepted, implemented)
- `Interpreter::evaluate(&EvaluationContext) -> VerificationResult` in `vp-reference-interpreter`
- Fixture-driven unit tests per **VP-RULE-0001** outcome table

**Success criteria:**

- [x] `evaluate` consumes `EvaluationContext` and returns frozen `VerificationResult`
- [x] Minimal rule outcomes match [VP-RULE-0001](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0001-minimal-claim-evidence-semantics.md) (`satisfied` / `not_satisfied` / `indeterminate`)
- [x] Trace events emitted when `trace_enabled`; empty trace when disabled
- [x] `SpecificationContext` bound on result; rule does not require manual spec reading
- [x] Code remains readable; performance is not a success criterion

**Not included:**

- Additional normative spec rules beyond the first minimal slice
- Full architecture model coverage (domain, behavior, state—expanded incrementally)
- Claim or evidence parsing
- CLI or report changes
- VP-CS runner integration (Milestone G)

---

## Milestone D.3 — Normative rule ownership

**Goal:** Replace ADR-0004 evaluation scaffolding with the first normative protocol rule from `veritypay-spec`.

**Prerequisite:** [VP-RFC-0001](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0001-minimal-claim-evidence-semantics.md) — minimal claim and evidence semantics (draft); Milestone D interpreter path (complete).

**Outputs:**

- `VpRule0001` implementing **VP-RULE-0001** (Assertion Body Evidence Match)
- Rule reference `VP-RULE-0001` on trace events and rule evaluations
- Fixture-driven tests renamed to VP-RULE-0001 profile

**Success criteria:**

- [x] Evaluation outcomes unchanged from Milestone D (`satisfied` / `not_satisfied` / `indeterminate`)
- [x] Rule layer cites **VP-RFC-0001** / **VP-RULE-0001** instead of ADR-0004 scaffolding labels
- [x] Public interpreter contract unchanged (`EvaluationContext` → `evaluate` → `VerificationResult`)

**Not included:**

- Negative VP-CS scenarios beyond existing fixture table
- VP-RULE registry publication in spec
- Conformance harness spec-path loading

---

## Milestone D.4 — Evidence claim binding

**Goal:** Implement **VP-RULE-0002** as a short-circuit precondition before **VP-RULE-0001** per [VP-RFC-0002](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0002-claim-identity-binding.md).

**Prerequisite:** [VP-RFC-0002](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0002-claim-identity-binding.md) (draft); Milestone D.3 **VP-RULE-0001** (complete); **VP-CS-0002** fixture published in `veritypay-spec`.

**Outputs:**

- `VpRule0002` implementing **VP-RULE-0002** (Evidence Claim Binding)
- `RuleSet::platform_1()` — **VP-RULE-0002** then **VP-RULE-0001** with short-circuit on binding failure
- Trace events for binding pass, short-circuit, and content evaluation paths

**Success criteria:**

- [x] **VP-CS-0001** outcomes unchanged (`satisfied` / `not_satisfied` / `indeterminate`)
- [x] Mismatched or empty `claim_id` values yield `indeterminate` via **VP-RULE-0002** without running **VP-RULE-0001**
- [x] Rule references and reason strings cite **VP-RULE-0002** for binding failures
- [x] Public interpreter contract unchanged (`EvaluationContext` → `evaluate` → `VerificationResult`)

**Not included:**

- VP-RFC-0002 acceptance in spec
- Conformance harness **VP-CS-0002** execution (Milestone G.5)
- VP-RULE registry publication

**Milestone status:** **Complete**.

---

## Milestone D.5 — Platform 1.2 model groundwork

**Goal:** Add domain support for **Evidence Set** and **Evaluation Policy** per accepted [VP-RFC-0003](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0003-multiple-evidence.md) and [VP-RFC-0004](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0004-evidence-evaluation-policies.md) without changing public interpreter behavior.

**Prerequisite:** Platform 1.2 accepted in `veritypay-spec`; Milestone D.4 **VP-RULE-0002** (complete).

**Outputs:**

- `EvidenceSet` and `EvidenceSetBuilder` in `vp-reference-model`
- `EvaluationPolicy::AllRequired` with canonical label `ALL_REQUIRED`
- Future-facing `EvaluationInput` in `vp-reference-core`

**Success criteria:**

- [x] Empty and multi-envelope `EvidenceSet` construction with order preserved for accessors
- [x] `EvaluationPolicy::AllRequired` policy id is `ALL_REQUIRED`
- [x] `EvaluationContext` API unchanged; **VP-CS-0001** and **VP-CS-0002** interpreter tests still pass
- [x] Domain types only — execution deferred to Milestone D.6

**Not included:**

- Per-envelope rule loop and `ALL_REQUIRED` aggregation in the interpreter
- **VP-CS-0003** / **VP-CS-0004** fixtures or conformance paths

**Milestone status:** **Complete**.

---

## Milestone D.6 — Platform 1.2 multi-evidence execution

**Goal:** Execute multi-evidence evaluation with **`ALL_REQUIRED`** aggregation per accepted [VP-RFC-0004](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0004-evidence-evaluation-policies.md) without breaking the Platform 1.1 `EvaluationContext` contract.

**Prerequisite:** Milestone D.5 model groundwork (complete); Platform 1.2 accepted in `veritypay-spec`.

**Outputs:**

- `Interpreter::evaluate_input(&EvaluationInput) -> VerificationResult`
- Per-envelope evaluation via existing `RuleSet::platform_1()` pipeline (VP-RULE-0002 → VP-RULE-0001)
- `ALL_REQUIRED` outcome aggregation with trace events per evidence index/id

**Success criteria:**

- [x] Empty, single, and multi-evidence `ALL_REQUIRED` outcomes match VP-RFC-0004 table
- [x] Aggregated outcome is independent of evidence insertion order
- [x] `Interpreter::evaluate(&EvaluationContext)` unchanged; **VP-CS-0001** / **VP-CS-0002** tests pass
- [x] Single-envelope `evaluate_input` outcome matches `evaluate` for the same inputs

**Not included:**

- **VP-CS-0003** / **VP-CS-0004** fixtures or conformance wiring
- CLI, report, or new protocol rule changes

**Milestone status:** **Complete**.

---

## Milestone D.7 — Assertion evaluator architecture

**Goal:** Refactor the interpreter to dispatch by **Assertion Type** per [ADR-0009](docs/adrs/0009-assertion-evaluator-architecture.md), aligning with draft **VP-RFC-0005** / **VP-RFC-0006** without changing public API or protocol outcomes.

**Prerequisite:** Milestone D.6 multi-evidence execution (complete); **VP-RFC-0005** / **VP-RFC-0006** drafted in `veritypay-spec`.

**Outputs:**

- [ADR-0009](docs/adrs/0009-assertion-evaluator-architecture.md) — Assertion Evaluator Architecture (Accepted)
- `AssertionEvaluator` trait, `BodyEqualityEvaluator`, `AssertionEvaluatorRegistry`
- `Interpreter` dispatches via registry; rules execute only inside evaluators

**Success criteria:**

- [x] `body_equality` and `minimal` (VP-RFC-0001 alias) dispatch to **Body Equality Evaluator**
- [x] Unknown `assertion_type` yields `indeterminate` per **VP-RFC-0006**
- [x] `Interpreter::evaluate` and `evaluate_input` unchanged; existing platform tests pass
- [x] No public API break

**Not included:**

- Normative acceptance of **VP-RFC-0005** / **VP-RFC-0006**
- Additional evaluator implementations (regex, numeric, signature, …)
- Conformance or fixture changes

**Milestone status:** **Complete**.

---

## Milestone D.8 — Platform 1.3 normalized text assertion

**Goal:** Implement draft [VP-RFC-0011](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0011-normalized-text-assertion.md) — **`normalized_text`** assertion type and **VP-RULE-0011** — as the first Content Equality extension beyond **`body_equality`**.

**Prerequisite:** Milestone D.7 assertion evaluator architecture (complete); **VP-RFC-0011** drafted in `veritypay-spec`.

**Outputs:**

- `text_normalization` module — deterministic NFC, trim, whitespace collapse per VP-RFC-0011
- **VP-RULE-0011** (*Normalized Text Equality*)
- `NormalizedTextEvaluator` registered in `AssertionEvaluatorRegistry`
- `RuleSet::platform_1_3()` — **VP-RULE-0002** then **VP-RULE-0011**
- Integration tests for normalization edge cases and registry dispatch

**Success criteria:**

- [x] **`body_equality`** / **`minimal`** behavior unchanged (Platform 1.1)
- [x] Platform 1.2 multi-evidence **`ALL_REQUIRED`** behavior unchanged for existing assertion types
- [x] **`normalized_text`** dispatches to **Normalized Text Evaluator** → **VP-RULE-0011**
- [x] Normalization pipeline matches VP-RFC-0011 (NFC, trim, collapse, case-sensitive compare)
- [x] **VP-CS** fixtures for **VP-RULE-0011** published in `veritypay-spec` (**VP-CS-0011**–**0013**)
- [x] Conformance harness execution of **VP-RULE-0011** scenarios

**Implemented assertion types:** **`body_equality`** (and **`minimal`** alias) and **`normalized_text`** only. No other assertion types.

**Not included:**

- Normative acceptance of **VP-RFC-0011**
- Additional Content Equality types (`canonical_json`, …)
- Additional assertion families (Structural, Pattern Matching, …)

**Milestone status:** **Complete** *(implementation)* — normative RFC acceptance remains draft.

---

## Milestone E — Produce verification outcome

**Goal:** Emit normative **verification outcomes** as defined by the conformance model.

**Prerequisite (multi-rule expansion):** [ADR-0006](docs/adrs/0006-ruleset-architecture.md) — `RuleSet` architecture (Accepted).

**Outputs:**

- Outcome component: `satisfied`, `not_satisfied`, `indeterminate`
- `verify(claim, evidence, spec_version)` alignment with [CONFORMANCE_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md)
- Stable outcome record for downstream comparison

**Success criteria:**

- [x] At least one positive fixture yields `satisfied`
- [x] At least one negative fixture yields `not_satisfied`
- [x] Insufficient-evidence fixture yields `indeterminate` when spec requires it
- [x] Outcome is bound to specification version used at evaluation time
- [x] No non-normative outcome labels introduced

**Not included:**

- Dedicated report crate polish (Milestone F — partial)
- Conformance suite orchestration (Milestone G — delivered in `veritypay-conformance`)
- Cross-implementation interoperability testing (owned by `veritypay-conformance`)

**Milestone status:** **Complete**.

---

## Milestone F — Produce trace

**Goal:** Explain **how** the interpreter reached an outcome.

**Outputs:**

- Trace component recording ordered evaluation steps
- Report component summarizing outcome + trace for human review
- References to spec sections or VP-TERM IDs where applicable

**Success criteria:**

- [x] Every outcome in Milestone E fixtures includes a trace
- [x] Trace steps are ordered and human-readable
- [x] Trace does not alter outcome—explanation only
- [x] Reviewer can follow claim → rules applied → outcome without reading source first (via CLI `--explain`)

**Not included:**

- Dedicated `vp-reference-report` crate (placeholder; CLI `output.rs` / `explain.rs` fulfill developer output today)
- Machine-readable trace protocol for production systems (unless promoted upstream)
- Conformance diff tooling (`veritypay-conformance`)
- Performance profiling of trace capture

**Milestone status:** **Partial** — interpreter trace and CLI explain delivered; report crate deferred.

---

## Milestone G — Conformance integration

**Goal:** Provide **hooks for VP-CS runners** in `veritypay-conformance`.

**Prerequisite:** [ADR-0007](docs/adrs/0007-reference-interpreter-public-contract.md) — reference interpreter public contract (Accepted).

**Outputs:**

- Stable entrypoint for scenario invocation (CLI or library—via ADR)
- Machine-readable outcome + trace export for CI
- Documentation for running reference oracle against VP-CS fixtures
- Alignment with at least one VP-CS scenario from `veritypay-spec`

**Success criteria:**

- [x] `veritypay-conformance` can invoke the reference interpreter without fork-specific glue
- [x] VP-CS-0001 and VP-CS-0011–0013 produce expected outcomes via reference oracle
- [x] Exit codes or structured results suitable for CI
- [x] Reference behavior documented as oracle—not mandatory production dependency

**Not included:**

- Authoring VP-CS scenario text (remains in `veritypay-spec`)
- Certification, badges, or vendor programs
- Production deployment or SDK packaging
- Full VP-CS catalog coverage (incremental)

**Milestone status:** **Complete**.

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
