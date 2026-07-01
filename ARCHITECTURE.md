# Architecture

**Long-term architecture for the VerityPay reference interpreter.**

This document describes **components, responsibilities, and data flow**. It does not specify implementation language, module layout, or runtime topology. Those decisions follow milestones in [ROADMAP.md](ROADMAP.md) and ADRs in this repository when code begins.

**Audience:** maintainers, contributors, implementers, auditors, and grant reviewers who need to understand what the reference interpreter will become—not how it is coded today.

**Upstream dependency:** [`veritypay-spec`](https://github.com/veritypay/veritypay-spec) defines domain semantics, behavior, state, data representation, verification outcomes, and conformance scenarios. The interpreter **implements accepted semantics**; it does not invent normative requirements.

**Shared input layer:** [`vp-spec-model`](https://github.com/veritypay/veritypay-tooling/blob/main/docs/SPECIFICATION_MODEL.md) in `veritypay-tooling` provides typed specification structures. The interpreter consumes validated specification input through that layer where practical.

---

## Design stance

| Principle | Meaning |
|-----------|---------|
| **Specification upstream** | All semantics derive from accepted documents and RFCs in `veritypay-spec` |
| **Readable over fast** | Code exists to be read, reviewed, and compared—not to win benchmarks |
| **Traceable evaluation** | Outcomes must be explainable; opaque verdicts are unacceptable for a reference |
| **Version-bound** | Every evaluation is tied to a specification version or Edition pin |
| **Not normative** | This repository demonstrates behavior; [`veritypay-spec`](https://github.com/veritypay/veritypay-spec) defines it |
| **Oracle, not gatekeeper** | Independent implementations may conform without shipping this code |

---

## System context

```
┌─────────────────────────────────────────────────────────────┐
│                     veritypay-spec                          │
│  architecture docs · RFCs · registries · VP-CS scenarios    │
└───────────────────────────┬─────────────────────────────────┘
                            │ validated corpus
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   veritypay-tooling                         │
│  vp validate · vp-spec-model (RegistrySet, DocumentCorpus)  │
└───────────────────────────┬─────────────────────────────────┘
                            │ typed specification input
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                  veritypay-reference                        │
│  claim + evidence → interpreter → outcome + trace + report  │
└───────────────────────────┬─────────────────────────────────┘
                            │ expected outcomes
                            ▼
              veritypay-conformance · local dev · education
```

The reference interpreter produces **verification outcomes and traces**. It does not validate registry YAML or fix broken markdown links—that is tooling.

---

## Evaluation pipeline

Conceptual flow aligned with [CONFORMANCE_MODEL.md — verification](https://github.com/veritypay/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md):

```
Specification input ──┐
Claim input ──────────┼──► Interpreter ──► Verification ──► Outcome
Evidence input ───────┘         │                              │
                                  └──────────► Trace ──────────┘
                                                    │
                                                    ▼
                                                 Report
```

Each stage below is a **component boundary**. Exact APIs and file layout are deferred until implementation milestones.

---

## Major components

### Specification input

**Purpose:** Load the normative rule set under which claims are evaluated.

**Responsibilities:**

- Accept a specification version, Edition pin, or validated checkout reference
- Load registries, architecture documents, and accepted RFC content needed for evaluation
- Prefer typed loading through `vp-spec-model` (`RegistrySet`, `DocumentCorpus`, future `EditionManifest`)
- Fail clearly when required normative material is missing or inconsistent

**Boundaries:**

- Does **not** define registry schema or document text
- Does **not** perform corpus hygiene checks—that is `veritypay-tooling`
- Does **not** mutate `veritypay-spec`

**Inputs:** Path or Edition reference; tooling-validated specification snapshot.

**Outputs:** In-memory specification context bound to a version identifier.

---

### Claim input

**Purpose:** Accept a claim to be interpreted under the loaded specification.

**Responsibilities:**

- Parse or deserialize claim data in a format defined by the spec (initial milestones use minimal fixtures)
- Validate syntactic well-formedness before semantic evaluation
- Attach source location or scenario ID when invoked from conformance runners
- Reject claims that cannot be interpreted under the bound specification version

**Boundaries:**

- Does **not** invent claim types or fields not defined upstream
- Does **not** perform network or chain I/O in early milestones

**Inputs:** Claim document, fixture, or scenario setup block.

**Outputs:** Typed claim representation ready for the interpreter.

---

### Evidence input

**Purpose:** Accept evidence offered in support of (or against) a claim.

**Responsibilities:**

- Parse or deserialize evidence alongside the claim
- Preserve linkage between claim and evidence as required by [DATA_MODEL](https://github.com/veritypay/veritypay-spec/blob/main/docs/01-architecture/DATA_MODEL.md)
- Surface parse errors before verification begins

**Boundaries:**

- Does **not** fetch evidence from external systems in scaffold or early milestones
- Does **not** define what counts as sufficient evidence—that is normative spec content implemented in verification

**Inputs:** Evidence document or scenario fixture.

**Outputs:** Typed evidence representation paired with the claim.

---

### Interpreter

**Purpose:** Apply normative semantics from the loaded specification to the claim and evidence.

**Responsibilities:**

- Resolve terminology and rules from specification input
- Evaluate domain, identity, behavior, state, and data constraints as scoped by the milestone
- Remain **readable**: explicit steps over clever abstractions
- Record intermediate decisions for trace production

**Boundaries:**

- Does **not** define protocol meaning—implements accepted documents only
- Does **not** optimize for throughput at the expense of clarity
- Does **not** embed product-specific business logic

**Inputs:** Specification context, claim, evidence.

**Outputs:** Verification result (internal) and trace events.

---

### Verification

**Purpose:** Execute the normative verification function defined by the specification.

**Responsibilities:**

- Implement `verify(claim, evidence, spec_version)` semantics as documented in the conformance model
- Apply behavioral and state invariants from architecture documents
- Map rule failures to structured reasons without inventing new outcome labels
- Distinguish **not satisfied** (rule failure) from **indeterminate** (insufficient information)

**Boundaries:**

- Outcome vocabulary is fixed by spec: `satisfied`, `not_satisfied`, `indeterminate`
- Does **not** add vendor-specific pass/fail grades

**Inputs:** Interpreter state, claim, evidence, specification context.

**Outputs:** Verification decision with structured reasons.

---

### Outcome

**Purpose:** Record the protocol-level verification result.

**Responsibilities:**

- Emit one of the normative outcomes: `satisfied`, `not_satisfied`, or `indeterminate`
- Bind outcome to specification version used at evaluation time
- Expose outcome in a stable, machine-readable form for conformance runners
- Preserve immutability after evaluation completes

**Boundaries:**

- Does **not** introduce non-normative outcome labels (e.g., `partial`, `error` as protocol outcomes) unless defined upstream
- Does **not** persist to database or chain in early milestones—in-memory/report only

**Inputs:** Verification decision.

**Outputs:** Outcome record suitable for report and conformance comparison.

---

### Trace

**Purpose:** Explain **how** the interpreter reached an outcome.

**Responsibilities:**

- Record ordered evaluation steps, rule references, and intermediate states
- Cite spec sections, VP-TERM IDs, or RFC IDs where rules are applied (when milestones support it)
- Support human review and diff against other implementations
- Enable conformance debugging without re-running opaque logic

**Boundaries:**

- Trace is **explanatory**, not normative—the spec text remains authoritative
- Trace format is this repository's contract; not a protocol transport format unless promoted upstream

**Inputs:** Interpreter and verification events.

**Outputs:** Trace artifact attached to the outcome.

---

### Report

**Purpose:** Present outcome and trace for humans and CI/conformance tools.

**Responsibilities:**

- Summarize outcome + spec version + claim/evidence identifiers
- Render trace for local dev and conformance integration
- Support machine-readable export when Milestone G requires it
- Use clear, institutional tone—not stack traces alone

**Boundaries:**

- Report is **downstream** of verification; does not alter outcome
- Does **not** replace `veritypay-conformance` pass/fail orchestration—that repo owns suite reporting

**Inputs:** Outcome, trace, specification context metadata.

**Outputs:** Human-readable and (later) structured report for downstream consumers.

---

## Boundaries with sibling repositories

| Concern | Owner |
|---------|-------|
| Corpus validation (spec tree, registries, links) | `veritypay-tooling` |
| Typed specification loading (`vp-spec-model`) | `veritypay-tooling` |
| Claim/evidence/outcome semantics | `veritypay-spec` |
| Reference evaluation implementation | **this repository** |
| VP-CS scenario text | `veritypay-spec` |
| Running VP-CS suites, pass/fail aggregation | `veritypay-conformance` |

---

## What this architecture deliberately omits

| Topic | Status |
|-------|--------|
| Implementation language | Deferred to first code milestone ADR |
| CLI surface | Deferred to ROADMAP milestones |
| Persistence, networking, blockchain adapters | Out of scope for reference interpreter |
| Performance tuning | Explicitly subordinate to correctness and readability |
| Production deployment topology | Not applicable |

---

## Related documents

| Document | Relationship |
|----------|--------------|
| [README.md](README.md) | Purpose and ecosystem boundaries |
| [ROADMAP.md](ROADMAP.md) | Milestone delivery order |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Specification boundary for contributors |
| [veritypay-spec — CONFORMANCE_MODEL](https://github.com/veritypay/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md) | Verification outcomes and VP-CS |
| [veritypay-tooling — SPECIFICATION_MODEL](https://github.com/veritypay/veritypay-tooling/blob/main/docs/SPECIFICATION_MODEL.md) | Shared typed input layer |
| [veritypay-spec — Phase II Platform Plan](https://github.com/veritypay/veritypay-spec/blob/main/docs/05-governance/PHASE_II_PLATFORM_PLAN.md) | Platform context |

---

*The specification defines meaning. The reference interpreter makes that meaning runnable—for education, review, and conformance comparison.*
