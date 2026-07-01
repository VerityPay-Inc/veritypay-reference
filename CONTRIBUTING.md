# Contributing to veritypay-reference

**Handbook for contributors implementing the VerityPay reference interpreter.**

You are not defining the VerityPay protocol here. You are building **readable executable semantics**—code that demonstrates accepted specification behavior so others can study, test, and compare implementations.

Read this before opening a pull request.

---

## Specification boundary

These four statements govern every contribution to this repository:

1. **Contributors implement specification behavior; they do not invent protocol behavior.** The interpreter applies rules from accepted documents and RFCs in [`veritypay-spec`](https://github.com/VerityPay-Inc/veritypay-spec). When code and spec disagree on meaning, the specification wins and the interpreter is updated.

2. **Protocol changes belong in `veritypay-spec` through RFCs.** Normative changes flow through [VP-RFC-0000](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0000-rfc-process.md) and governance in `veritypay-spec`. Implement resulting semantics here only after they are accepted upstream.

3. **The interpreter follows the spec.** This repository does not assign new meaning to VP-TERM IDs, introduce verification outcomes, or alter Architecture Alpha. It executes accepted semantics—not drafts, not product preferences.

4. **Correctness and readability matter more than performance.** The reference interpreter exists to be **read, reviewed, and trusted** as an oracle. Clever optimizations that obscure behavior are out of scope unless an ADR justifies them without sacrificing traceability.

---

## Welcome

Contributing to `veritypay-reference` means strengthening **public, executable semantics** for the VerityPay ecosystem.

This repository exists because:

- **Conformance** requires a default oracle implementations can compare against
- **Educators and auditors** need runnable evidence of specification behavior
- **Implementers** deserve code aligned with validated upstream text—not a parallel protocol
- **VP-CS scenarios** need an interpreter that reproduces normative outcomes

We welcome engineers, specification readers, and conformance authors. You do not need permission to read, propose issues, or draft ADRs. You **do** need to respect the boundary: **the interpreter follows the specification; it never defines it.**

---

## Before you start

| Order | Document | Why |
|-------|----------|-----|
| 1 | [README.md](README.md) | Purpose, boundaries, maturity |
| 2 | [ARCHITECTURE.md](ARCHITECTURE.md) | Component model and data flow |
| 3 | [ROADMAP.md](ROADMAP.md) | Current milestone and success criteria |
| 4 | [veritypay-spec — CONFORMANCE_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md) | Verification outcomes and VP-CS |
| 5 | [veritypay-spec — SPECIFICATION_STATUS](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/SPECIFICATION_STATUS.md) | Ecosystem maturity |
| 6 | [veritypay-tooling — SPECIFICATION_MODEL](https://github.com/VerityPay-Inc/veritypay-tooling/blob/main/docs/SPECIFICATION_MODEL.md) | Shared typed input layer |
| 7 | [veritypay-spec — Phase II Platform Plan](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/05-governance/PHASE_II_PLATFORM_PLAN.md) | Where the reference interpreter sits |

For protocol and governance context, read [veritypay-spec — CONTRIBUTING](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/CONTRIBUTING.md) when your work touches semantics defined there.

---

## The golden rule

> **Changes to protocol semantics belong in `veritypay-spec` through RFCs.**  
> **The reference interpreter implements accepted specification behavior—it never invents the protocol.**

| If you want to… | Do this |
|-----------------|---------|
| Change what a claim **means** | RFC or architecture change in `veritypay-spec` |
| Add a new verification outcome label | Governance or RFC in `veritypay-spec`, then implement here |
| Implement `verify(claim, evidence, spec_version)` for accepted rules | Pull request in **this** repository |
| Validate registry YAML or fix broken links | Pull request in **`veritypay-tooling`** |
| Run VP-CS suites and aggregate pass/fail | **`veritypay-conformance`** (future) |
| Change Architecture Alpha | RFC in `veritypay-spec` ([GOVERNANCE](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/05-governance/GOVERNANCE.md)) |

When unsure, **default to spec governance first.**

---

## What belongs in this repository

| In scope | Examples |
|----------|----------|
| Specification loading | Consume `vp-spec-model` or validated spec input |
| Claim and evidence parsing | Minimal fixtures expanding toward full DATA_MODEL |
| Interpreter and verification | Readable evaluation of accepted rules |
| Outcomes and traces | `satisfied`, `not_satisfied`, `indeterminate` + explainability |
| Reference fixtures | Golden claim/evidence/outcome examples |
| ADRs | Language choice, module boundaries, public oracle API |
| Tests | Scenario fixtures aligned with VP-CS where available |

| Out of scope | Examples |
|--------------|----------|
| Normative spec edits | Wording, new claim types, registry schema policy |
| Corpus validation | Registry validators, cross-reference checks |
| Production features | Wallets, payroll UI, merchant onboarding |
| SDKs and HTTP APIs | Integrator-facing product surfaces |
| Performance tuning as primary goal | Micro-optimizations without readability justification |

---

## Implementation values

When code milestones begin, prefer:

| Value | Practice |
|-------|----------|
| **Readability** | Explicit steps; name rules after spec concepts where possible |
| **Traceability** | Every outcome should be explainable via trace (Milestone F+) |
| **Spec fidelity** | Load rules from specification input—avoid copied prose constants |
| **Minimal scope** | Smallest milestone that proves one semantic path end-to-end |
| **Test fixtures** | Golden files with expected outcomes—not only happy paths |

Avoid:

- Hard-coding normative text that belongs in loaded specification context
- Introducing behavior "because it seems right" without upstream acceptance
- Optimizing before correctness and explainability are proven

---

## Pull request expectations

Before requesting review:

1. Confirm the change aligns with the **current ROADMAP milestone**
2. Confirm no normative spec change is smuggled in without an upstream RFC
3. Add or update fixtures demonstrating expected behavior
4. Prefer citations to spec sections, VP-TERM IDs, or VP-CS IDs in comments and traces where helpful
5. If the change affects public oracle contracts, propose or reference an ADR

---

## Related repositories

| Repository | Relationship |
|------------|--------------|
| [`veritypay-spec`](https://github.com/VerityPay-Inc/veritypay-spec) | Source of truth — defines meaning |
| [`veritypay-tooling`](https://github.com/VerityPay-Inc/veritypay-tooling) | Validates corpus; supplies `vp-spec-model` |
| [`veritypay-conformance`](https://github.com/VerityPay-Inc/veritypay-conformance) | VP-CS runners (future) — consumes reference outcomes |

---

*Implement what the specification says. Make it readable enough that others can trust the oracle.*
