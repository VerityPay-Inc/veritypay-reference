---
id: ADR-0002
title: Cargo Workspace Architecture
status: accepted
version: 1.0.0
authors:
  - VerityPay Core Team
reviewers: []
related_docs:
  - docs/adrs/0001-reference-implementation-language.md
  - ARCHITECTURE.md
  - ROADMAP.md
decision_date: 2026-07-01
superseded_by: null
---

# ADR-0002 — Cargo Workspace Architecture

**Status:** Accepted · **Version:** 1.0.0 · **Date:** 2026-07-01

**Related:** [ADR-0001](0001-reference-implementation-language.md) · [ARCHITECTURE.md](../../ARCHITECTURE.md) · [ROADMAP.md](../../ROADMAP.md) · [veritypay-tooling — ADR-0002](https://github.com/VerityPay-Inc/veritypay-tooling/blob/main/docs/adrs/0002-workspace-architecture.md)

---

## Purpose

Define the **Rust workspace decomposition** for `veritypay-reference` before implementation begins.

---

## Context

Milestone A established the reference interpreter scaffold: purpose, architecture, roadmap, and contribution rules.

[ADR-0001](0001-reference-implementation-language.md) chose **Rust** as the implementation language, primarily for correctness, traceability, and consumption of **`vp-spec-model`** from [`veritypay-tooling`](https://github.com/VerityPay-Inc/veritypay-tooling).

Milestone B will load specification input. Before source files land, the project must record **how the codebase decomposes** into crates.

[ARCHITECTURE.md](../../ARCHITECTURE.md) defines the evaluation pipeline:

```
Specification input → Claim input + Evidence input → Interpreter → Verification → Outcome → Trace → Report
```

A single binary crate would tempt:

- Monolithic modules mixing CLI, specification loading, domain types, evaluation, and reporting
- Hidden coupling between specification input and verification rules
- Circular imports as trace and conformance features arrive
- A catch-all `utils` or `common` crate that becomes unmaintainable

This ADR records crate boundaries, dependency direction, and expansion rules. It does **not** create `Cargo.toml` files or code—that follows in Milestone B implementation PRs.

---

## Decision

**Implement `veritypay-reference` as a Cargo workspace of focused crates**, not a single binary package.

Initial workspace members:

| Crate | Role |
|-------|------|
| **`vp-reference-cli`** | Binary entrypoint; arguments; output modes; exit codes |
| **`vp-reference-core`** | Shared context, errors, traits, execution contracts |
| **`vp-reference-spec`** | Load specification input through `vp-spec-model` |
| **`vp-reference-model`** | Claim, Evidence, Outcome, Trace domain types |
| **`vp-reference-interpreter`** | Verification logic and evaluation flow |
| **`vp-reference-report`** | Human and machine-readable reporting |

**Composition rule:** The CLI invokes the interpreter through **contracts defined in `vp-reference-core`**. The interpreter loads specification context from **`vp-reference-spec`** and produces **`vp-reference-model`** outcomes and traces. **`vp-reference-report`** formats results—it does not alter verification semantics.

---

## Workspace rules

| Rule | Detail |
|------|--------|
| **No protocol meaning invented here** | Semantics derive from `veritypay-spec`; crates implement accepted rules only |
| **No validator behavior from `veritypay-tooling`** | Registry and cross-reference validation remain in tooling—not reimplemented here |
| **No production app behavior** | No wallets, payroll UI, or product workflows |
| **No network or blockchain adapters** | No chain RPC, HTTP servers, or live evidence fetch in core milestones |
| **No cyclic dependencies** | Dependency graph is acyclic; cycles require ADR supersession |
| **No `utils` or `common` crates** | Shared types belong in `vp-reference-core` or `vp-reference-model` with documented ownership |

---

## Crate responsibilities

### `vp-reference-cli`

| Field | Definition |
|-------|------------|
| **Purpose** | User-facing **CLI** for local development, education, and conformance hooks |
| **Responsibilities** | Parse arguments; build evaluation context; invoke interpreter pipeline; dispatch to human or machine-readable report output; map results to process exit codes; `--help` and version output |
| **Does not belong** | Verification rules; claim/evidence parsing logic; specification loading internals; protocol semantics; validator behavior |
| **Depends on** | `vp-reference-core`, `vp-reference-interpreter`, `vp-reference-report`, `vp-reference-spec` (wiring only) |

The CLI is a **thin shell**. Substantive logic belongs in interpreter, spec, model, or report crates.

---

### `vp-reference-core`

| Field | Definition |
|-------|------------|
| **Purpose** | **Stable contracts** between CLI, specification loading, interpreter, and reporting |
| **Responsibilities** | `EvaluationContext` (spec binding, version pin, input paths); shared error types; interpreter and reporter traits; execution lifecycle types; specification-version binding helpers that are crate-neutral |
| **Does not belong** | Verification rule implementations; CLI argument structs (`clap`); report formatting; `vp-spec-model` loading; claim/evidence field definitions |
| **Depends on** | `vp-reference-model` (outcome/trace types in trait signatures) — keep minimal |

`vp-reference-core` stays **small**. Resist expanding it with every shared helper—prefer placing logic in the crate that owns the behavior.

---

### `vp-reference-spec`

| Field | Definition |
|-------|------------|
| **Purpose** | **Specification input** — load validated corpus data for evaluation |
| **Responsibilities** | Load `RegistrySet`, `DocumentCorpus`, and related structures through **`vp-spec-model`**; bind loaded context to a specification or Edition identifier; fail clearly when required normative material is missing; expose read-only specification context to the interpreter |
| **Does not belong** | Claim or evidence parsing; verification rules; CLI; report formatting; corpus validation (that is `veritypay-tooling`); defining registry or document schema |
| **Depends on** | `vp-reference-core`, `vp-spec-model` (path, git, or crates.io dependency from `veritypay-tooling`) |

This crate is the **bridge** to validated upstream specification representation—not a second parser.

---

### `vp-reference-model`

| Field | Definition |
|-------|------------|
| **Purpose** | **Domain types** for interpreter inputs and outputs |
| **Responsibilities** | `Claim`, `Evidence`, `Outcome` (`satisfied`, `not_satisfied`, `indeterminate`); `Trace` and trace events; parse-error types at the domain boundary; stable identifiers for conformance comparison |
| **Does not belong** | Verification logic; specification loading; CLI; report rendering; normative schema invention beyond accepted spec documents |
| **Depends on** | Standard library and serialization crates only — **leaf crate** (no workspace interpreter dependencies) |

Domain types are the **lingua franca** between parsing, evaluation, and reporting.

---

### `vp-reference-interpreter`

| Field | Definition |
|-------|------------|
| **Purpose** | **Verification logic and evaluation flow** |
| **Responsibilities** | Parse or accept claim and evidence (minimal fixtures in early milestones); apply normative rules from loaded specification context; produce verification decisions; emit trace events during evaluation; implement `verify(claim, evidence, spec_version)` semantics per [CONFORMANCE_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md) |
| **Does not belong** | CLI; human/JSON report formatting; `vp-spec-model` loading (delegate to `vp-reference-spec`); registry or link validation; network I/O; product business logic |
| **Depends on** | `vp-reference-core`, `vp-reference-model`, `vp-reference-spec` — **not** `vp-reference-cli` or `vp-reference-report` |

All verification rule knowledge stays **inside this crate** (or private modules within it).

---

### `vp-reference-report`

| Field | Definition |
|-------|------------|
| **Purpose** | **Human and machine-readable reporting** |
| **Responsibilities** | Render outcome + trace + specification metadata for reviewers; support human-readable output for local dev; support structured export for conformance runners (Milestone G); present clear summaries—not stack traces alone |
| **Does not belong** | Verification logic; specification loading; CLI argument parsing; altering outcomes or traces |
| **Depends on** | `vp-reference-core`, `vp-reference-model` — **not** `vp-reference-interpreter` internals |

Reporting is **downstream** of evaluation. Formatters read model types; they do not re-run verification.

---

## Dependency graph

```
                 ┌─────────────────────┐
                 │  vp-reference-cli   │  (binary)
                 └──────────┬──────────┘
                            │
         ┌──────────────────┼──────────────────┐
         ▼                  ▼                  ▼
┌─────────────────┐ ┌──────────────┐ ┌─────────────────┐
│vp-reference-    │ │vp-reference- │ │vp-reference-    │
│interpreter      │ │report        │ │spec             │
└────────┬────────┘ └──────┬───────┘ └────────┬────────┘
         │                   │                  │
         └─────────┬─────────┘                  │
                   ▼                            │
          ┌─────────────────┐                   │
          │ vp-reference-   │◄──────────────────┘
          │ core            │
          └────────┬────────┘
                   ▼
          ┌─────────────────┐         ┌─────────────────┐
          │ vp-reference-   │         │  vp-spec-model  │  (external)
          │ model           │         │  (veritypay-    │
          └─────────────────┘         │   tooling)      │
                                        └─────────────────┘
```

**Allowed dependency direction (summary):**

| From | May depend on |
|------|----------------|
| `vp-reference-model` | *(none in workspace)* |
| `vp-reference-core` | `vp-reference-model` |
| `vp-reference-spec` | `vp-reference-core`, `vp-spec-model` |
| `vp-reference-interpreter` | `vp-reference-core`, `vp-reference-model`, `vp-reference-spec` |
| `vp-reference-report` | `vp-reference-core`, `vp-reference-model` |
| `vp-reference-cli` | all workspace crates above (wiring) |

**Forbidden:**

- Interpreter → `vp-reference-cli` or `vp-reference-report`
- Report → `vp-reference-interpreter`
- Spec → interpreter or report
- Any cycle in the graph
- Reimplementing `veritypay-tooling` validators inside reference crates

---

## Cyclic dependencies are forbidden

| Reason | Explanation |
|--------|-------------|
| **Architectural honesty** | Cycles hide boundaries this ADR and [ARCHITECTURE.md](../../ARCHITECTURE.md) exist to preserve |
| **Readable evaluation** | Interpreter logic must remain followable without circular type dependencies |
| **Testability** | Interpreter tests against fixtures without loading CLI or report formatters |
| **Conformance clarity** | Outcome types flow one direction: model → report |

If two crates appear to need each other, **extract a smaller type or trait downward** (usually into `vp-reference-core` or `vp-reference-model`)—never introduce a mutual dependency.

---

## No `utils` or `common` crates

Intentionally **avoid** workspace members named `vp-reference-utils`, `vp-reference-common`, or `shared`.

| Problem | Consequence |
|---------|-------------|
| **Magnet for ambiguity** | Every helper lands there; ownership disappears |
| **Hidden coupling** | Evaluation and reporting share implicit state through grab-bag modules |
| **Review fatigue** | PRs touch `common` for unrelated reasons |

**Instead:**

- **Cross-cutting contracts** → `vp-reference-core` (minimal)
- **Domain types** → `vp-reference-model`
- **Specification loading** → `vp-reference-spec`
- **Verification rules** → `vp-reference-interpreter` (private modules)
- **Output formatting** → `vp-reference-report`
- **CLI concerns** → `vp-reference-cli`

If `vp-reference-core` grows too large, split by **conceptual boundary** with a documented ADR—not into `common`.

---

## Future expansion

| Crate | Milestone | Purpose |
|-------|-----------|---------|
| **`vp-reference-fixtures`** | B–C | Shared golden claim/evidence/outcome fixtures for tests and docs |
| **`vp-reference-conformance`** | G | VP-CS scenario wiring and structured export for `veritypay-conformance` |

**`vp-reference-fixtures`** (optional):

- Holds test data aligned with VP-CS scenarios
- Depends on `vp-reference-model` only
- Does not ship in the default CLI binary unless explicitly invoked

**`vp-reference-conformance`** (optional):

- Stable entrypoints for external conformance runners
- Depends on `vp-reference-interpreter`, `vp-reference-report`, `vp-reference-spec`
- Does not author VP-CS normative text (that remains in `veritypay-spec`)

Adding either crate requires the same acyclic rules and an ADR update if boundaries change materially.

---

## Rationale

| Factor | Workspace decomposition |
|--------|-------------------------|
| **ADR-0001 (Rust)** | Cargo workspaces are idiomatic; enables `vp-spec-model` dependency |
| **ARCHITECTURE.md** | Pipeline stages map to crate boundaries |
| **Milestone delivery** | B loads spec; C–E add claim/evidence/outcome incrementally without monolith refactor |
| **Contributor clarity** | "Fix outcome type" → `vp-reference-model`; "Fix rule evaluation" → `vp-reference-interpreter` |
| **Tooling separation** | Spec loading reuses `vp-spec-model`; validation stays in `veritypay-tooling` |
| **Readability** | Interpreter crate remains the primary reading surface for semantics |

A monolith would compile faster initially and fail structurally when trace and conformance milestones arrive.

---

## Consequences

### Positive

- Compile-time enforcement of evaluation pipeline boundaries
- Clear ownership per milestone and per domain concern
- CLI and report crates remain stable as verification rules expand
- `vp-reference-model` reusable by fixtures, conformance, and tests
- Aligns with [ARCHITECTURE.md](../../ARCHITECTURE.md) — readable semantics at the interpreter edge

### Negative

- More crates to bootstrap before first green CI
- Contributors must learn workspace layout and dependency rules
- Initial PRs touch multiple `Cargo.toml` files when workspace is created
- External dependency on `vp-spec-model` versioning must be managed explicitly

**Acceptable** because boundaries are cheaper to establish **before** Milestone B code than to extract after evaluation logic entangles.

---

## Future reconsideration

Revisit this ADR only if:

- Workspace member count creates measurable maintainer burden **without** corresponding isolation benefit
- A proposed crate violates acyclic rules and cannot be resolved by trait extraction
- `vp-spec-model` integration requires a layout change that cannot be accommodated within `vp-reference-spec`
- An accepted ADR supersedes this layout

Splitting or merging crates requires **ADR-0003** (or successor)—not drive-by refactors.

---

## Related decisions

| Document | Relationship |
|----------|--------------|
| [ADR-0001](0001-reference-implementation-language.md) | Rust + workspace recommendation |
| [ARCHITECTURE.md](../../ARCHITECTURE.md) | Component model this ADR implements |
| [ROADMAP.md](../../ROADMAP.md) | Milestone B follows this ADR |
| [veritypay-tooling — ADR-0007](https://github.com/VerityPay-Inc/veritypay-tooling/blob/main/docs/adrs/0007-specification-model-stability.md) | Stable `vp-spec-model` consumer contract |
| [veritypay-spec — CONFORMANCE_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md) | Outcome vocabulary and verification flow |

---

## Follow-up

- [ ] Add workspace `Cargo.toml` and crate manifests (Milestone B — separate PR)
- [ ] Document workspace layout in README when directories exist
- [ ] Declare `vp-spec-model` dependency path in workspace manifest

---

## Conclusion

The reference interpreter treats **structure as a feature**. Crate boundaries make the evaluation pipeline visible: spec loads, the interpreter evaluates, the model carries meaning, the report explains.

Implementation may begin once the workspace is scaffolded according to this ADR.

---

*Accepted ADRs are historical records. Supersede with a new ADR; do not silently rewrite this decision.*
