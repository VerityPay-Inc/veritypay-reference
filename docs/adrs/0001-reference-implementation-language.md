---
id: ADR-0001
title: Implementation Language for veritypay-reference
status: accepted
version: 1.0.0
authors:
  - VerityPay Core Team
reviewers: []
related_docs:
  - README.md
  - ARCHITECTURE.md
  - ROADMAP.md
  - CONTRIBUTING.md
decision_date: 2026-06-30
superseded_by: null
---

# ADR-0001 — Implementation Language for veritypay-reference

**Status:** Accepted · **Version:** 1.0.0 · **Date:** 2026-06-30

**Related:** [README.md](../../README.md) · [ARCHITECTURE.md](../../ARCHITECTURE.md) · [ROADMAP.md](../../ROADMAP.md) · [veritypay-tooling — ADR-0001](https://github.com/veritypay/veritypay-tooling/blob/main/docs/adrs/0001-tooling-implementation-language.md) · [veritypay-spec — ADR Guide](https://github.com/veritypay/veritypay-spec/blob/main/docs/05-governance/ADR_GUIDE.md)

---

## Purpose

Choose the **implementation language** for the VerityPay reference interpreter.

---

## Context

Milestone A established `veritypay-reference` as a **documentation-only scaffold**: purpose, architecture, roadmap, and contribution rules. Milestone B will load validated specification input through **`vp-spec-model`** in [`veritypay-tooling`](https://github.com/veritypay/veritypay-tooling).

Before implementation begins, the project must choose an implementation language. This is an **engineering decision**—not a protocol decision. It does not bind independent VerityPay implementers. It does bind how this repository evolves.

`veritypay-reference` is the **executable specification** for VerityPay. It must:

- Consume the validated specification model from `veritypay-tooling` / `vp-spec-model`
- Prioritize **correctness**, **readability**, and **traceability** over production performance
- Produce verification **outcomes** and **traces** suitable for education, conformance, and review

The language must model claims, evidence, outcomes, and traces **explicitly**—and remain legible to reviewers who treat this code as an oracle, not a product codebase.

---

## Decision

**`veritypay-reference` will be implemented in Rust.**

The primary reason is **not** raw performance. The reference interpreter is not a throughput benchmark; evaluation clarity matters more than microseconds.

The primary reasons are:

- **Same ecosystem as `veritypay-tooling`** — shared engineering conventions, CI patterns, and contributor context
- **Consume `vp-spec-model` with least friction** — direct dependency on stable typed structures ([ADR-0007](https://github.com/veritypay/veritypay-tooling/blob/main/docs/adrs/0007-specification-model-stability.md))
- **Strong typing** for claims, evidence, outcomes, and traces — explicit models that document intent
- **Reliable CLI and test tooling** — reproducible local and CI runs for conformance hooks
- **Long-term maintainability** — a codebase that remains readable as semantics expand across milestones B–G
- **Alignment with reference/conformance tooling direction** — institutional infrastructure, not a script

---

## Alternatives considered

Each option was evaluated for **reference interpreter fit**, not general language popularity.

### Rust

**Advantages**

- Native consumption of `vp-spec-model` in the same workspace or as a path/git dependency
- Strong static types for claim, evidence, outcome, and trace structures
- Excellent error modeling for parse and evaluation failures
- Mature test and fixture tooling (`cargo test`, golden files)
- Readable explicit control flow for trace production
- Same language as validators—reduces context switching for platform contributors

**Tradeoffs**

- Steeper learning curve for contributors unfamiliar with ownership and lifetimes
- Smaller contributor pool than JavaScript or Python

**Assessment:** Best match for correctness, explicit modeling, traceability, and `vp-spec-model` reuse.

---

### Go

**Advantages**

- Simple language with fast compilation
- Approachable for many backend engineers
- Good standard library for CLI and file I/O
- Single-binary deployment story

**Tradeoffs**

- Less expressive type system for modeling claim/evidence/outcome domains and rich traces
- No direct reuse of `vp-spec-model`—requires FFI, regeneration, or duplicate types
- Weaker compile-time enforcement as semantic surface area grows

**Assessment:** Viable for a thin CLI; weaker fit for typed integration with `vp-spec-model` and trace-heavy evaluation.

---

### TypeScript

**Advantages**

- Very large contributor ecosystem
- Rapid iteration during early prototyping
- Familiar to many application developers

**Tradeoffs**

- Requires Node.js (or bundled runtime) at execution time
- No native `vp-spec-model` consumption—duplicate types or separate binding layer
- Weaker deployment story for a pinned conformance oracle in CI
- Runtime type drift unless carefully disciplined

**Assessment:** Strong for examples and docs tooling; weaker as the canonical reference interpreter runtime.

---

### Python

**Advantages**

- Enormous ecosystem and parsing libraries
- Rapid development for experiments and notebooks

**Tradeoffs**

- Packaging and dependency friction for reproducible CI
- No native `vp-spec-model` consumption
- Harder to ship one portable, pinned oracle artifact
- Interpreter version drift across contributors

**Assessment:** Excellent for ad hoc analysis; poor fit as the long-lived executable specification core.

---

## Rationale

The reference interpreter exists so specification semantics can be **run**, **compared**, and **explained**—not so this repository wins performance benchmarks.

Rust provides:

1. **Direct reuse of `vp-spec-model`** — `RegistrySet`, `DocumentCorpus`, and `ReferenceGraph` without re-parsing or type duplication
2. **Explicit domain modeling** — claims, evidence, outcomes (`satisfied`, `not_satisfied`, `indeterminate`), and traces as types reviewers can follow
3. **Traceability** — structured evaluation steps and rule references without sacrificing readability
4. **Platform coherence** — same toolchain and conventions as `veritypay-tooling`, reducing institutional overhead

Go, TypeScript, and Python remain reasonable choices for **adjacent** work (documentation sites, one-off scripts, product SDKs). They are not rejected globally—only for **this** repository's core mission.

---

## Consequences

### Positive

- **Shared specification types** — evaluation loads the same model validators already use
- **Explicit semantics** — strong types document claim/evidence/outcome boundaries
- **Reliable oracle** — reproducible builds for conformance and audit evidence
- **Readable evaluation** — explicit control flow supports trace production (Milestone F)
- **Platform alignment** — contributors moving between tooling and reference work in one ecosystem

### Negative

- **Rust learning curve** — some contributors need onboarding before first merged evaluation PR
- **Coupling to Rust `vp-spec-model`** — non-Rust implementers cannot embed the model without bindings (acceptable; they implement from spec text)
- **Slower initial velocity** — workspace bootstrap and idiomatic modeling take time upfront

**Why these tradeoffs are acceptable**

The reference interpreter is **institutional infrastructure**—an executable specification readers must trust. Slower start with explicit models and shared types beats fast iteration that produces an oracle divergent from validated specification input.

---

## Future reconsideration

This decision should **only** be revisited if:

- Rust becomes a **blocker to ecosystem sustainability** (e.g. no maintainers able to steward the codebase over a documented period)
- **`vp-spec-model` integration** cannot reasonably be achieved in Rust without disproportionate cost
- A **future ADR supersedes** this one with new evidence

Changing the implementation language **requires a new ADR**. Partial rewrites without ADR are not acceptable for the core interpreter.

---

## Related decisions

| Document | Relationship |
|----------|--------------|
| [ARCHITECTURE.md](../../ARCHITECTURE.md) | Component model; language deferred until this ADR |
| [ROADMAP.md](../../ROADMAP.md) | Milestone B proceeds after this ADR |
| [CONTRIBUTING.md](../../CONTRIBUTING.md) | Correctness and readability over performance |
| [veritypay-tooling — ADR-0001](https://github.com/veritypay/veritypay-tooling/blob/main/docs/adrs/0001-tooling-implementation-language.md) | Tooling language choice (same ecosystem) |
| [veritypay-tooling — ADR-0007](https://github.com/veritypay/veritypay-tooling/blob/main/docs/adrs/0007-specification-model-stability.md) | Stable `vp-spec-model` for consumers |
| [veritypay-spec — ADR Guide](https://github.com/veritypay/veritypay-spec/blob/main/docs/05-governance/ADR_GUIDE.md) | ADR process |

---

## Follow-up

- [ ] Bootstrap `cargo` workspace (Milestone B implementation—separate PR)
- [ ] Add `vp-spec-model` dependency path when workspace exists
- [ ] Document local build path in README when code lands

---

## Conclusion

The reference interpreter values **correctness, explicit modeling, and traceability** over raw performance. Rust best supports consumption of **`vp-spec-model`**, readable evaluation, and long-term maintainability as the executable specification grows.

---

*Accepted ADRs are historical records. Supersede with a new ADR; do not silently rewrite this decision.*
