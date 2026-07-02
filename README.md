# veritypay-reference

**Executable specification for the VerityPay protocol.**

This repository is part of the **Verity Specification Platform**. It implements **readable reference semantics** for claims, evidence, and verification—so that protocol behavior can be studied, tested, and compared. It does **not** define protocol meaning.

**Repository maturity:** **Specification loading** — Cargo workspace per [ADR-0002](docs/adrs/0002-workspace-architecture.md); validated spec input loads through `vp-spec-model` (Milestone B). Claim parsing and verification not yet implemented.

---

## Documentation

| Document | Description |
|----------|-------------|
| [README.md](README.md) | Purpose, boundaries, and ecosystem links *(this file)* |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Long-term component model—conceptual, not executable code |
| [ROADMAP.md](ROADMAP.md) | Capability milestones A–G with success criteria |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How to contribute to the reference interpreter |
| [docs/adrs/README.md](docs/adrs/README.md) | Architecture Decision Records |
| [docs/adrs/0001-reference-implementation-language.md](docs/adrs/0001-reference-implementation-language.md) | ADR-0001 — Implementation language (Rust) |
| [docs/adrs/0002-workspace-architecture.md](docs/adrs/0002-workspace-architecture.md) | ADR-0002 — Cargo workspace architecture |
| [docs/adrs/0007-reference-interpreter-public-contract.md](docs/adrs/0007-reference-interpreter-public-contract.md) | ADR-0007 — Public interpreter contract (`evaluate`) |
| [LICENSE](LICENSE) | License terms for this repository |

---

## What is the reference interpreter?

The **reference interpreter** ([VP-TERM-027](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/00-overview/GLOSSARY.md#reference-interpreter)) is an **executable artifact** that evaluates claims and verification rules for education and conformance. It makes normative semantics **testable** without mandating one production stack.

This repository is the **reference interpreter**, not a [reference implementation (VP-TERM-026)](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/00-overview/GLOSSARY.md#reference-implementation) in the production-pattern sense. VP-TERM-026 describes software that demonstrates accepted behavior broadly; this repo **executes** specification semantics for interpretation, verification, and review.

`veritypay-reference` is **not**:

- a production implementation
- an SDK or integrator library
- a payment application or wallet
- the owner of protocol truth

It is **readable code that executes the behavior defined by [`veritypay-spec`](https://github.com/VerityPay-Inc/veritypay-spec)**—a **reference interpreter** for education, conformance, and review, not a canonical production pattern.

The interpreter **follows** the specification. It never **defines** it.

**Public contract** ([ADR-0007](docs/adrs/0007-reference-interpreter-public-contract.md)): `EvaluationContext` → `Interpreter::evaluate` → `VerificationResult` for Platform 1.1 single-evidence evaluation. Platform 1.2 multi-evidence evaluation uses `EvaluationInput` → `Interpreter::evaluate_input` → `VerificationResult`. Both entrypoints are supported; `evaluate` is unchanged.

The first implemented protocol rules for **Platform 1.1** are **VP-RULE-0002** (*Evidence Claim Binding*) and **VP-RULE-0001** (*Assertion Body Evidence Match*) from [`veritypay-spec`](https://github.com/VerityPay-Inc/veritypay-spec) ([VP-RFC-0002](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0002-claim-identity-binding.md), [VP-RFC-0001](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0001-minimal-claim-evidence-semantics.md)).

**Platform 1.2** ([VP-RFC-0003](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0003-multiple-evidence.md), [VP-RFC-0004](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0004-evidence-evaluation-policies.md)) is **accepted in the specification**. This repository implements **`ALL_REQUIRED`** multi-evidence execution via `Interpreter::evaluate_input`. The Platform 1.1 `EvaluationContext` contract remains supported unchanged.

---

## Why does it exist?

Phase I established *what VerityPay is*: domain model, behavior, state, data representation, and conformance philosophy in prose.

Phase II requires semantics to become **executable**—so that independent implementations can ask *"Am I conforming?"* and receive an answer grounded in the same normative rules.

Manual reading of architecture documents does not scale for:

- **Conformance scenarios** ([VP-CS](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md)) that require reproducible outcomes
- **Grant and audit audiences** who need runnable evidence of specification behavior
- **Implementers** who need an oracle to compare against—not a hidden source of truth

`veritypay-reference` exists so that:

- Verification semantics in the spec can be **run**, not only read
- `veritypay-conformance` has a **default oracle** for expected outcomes
- Educators and reviewers can trace **claim → evidence → outcome** in code aligned with accepted documents

See [Phase II Platform Plan](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/05-governance/PHASE_II_PLATFORM_PLAN.md) in `veritypay-spec`.

---

## Relationship to veritypay-spec

```
veritypay-spec          ← source of truth (normative text, registries, RFCs, VP-CS)
       ↓ consumed by
veritypay-tooling         ← validates corpus integrity
       ↓ enables
veritypay-reference       ← executable semantics (this repo)
       ↓ enables
veritypay-conformance     ← VP-CS runners (future)
```

| Responsibility | `veritypay-spec` | `veritypay-reference` |
|----------------|------------------|------------------------|
| Protocol meaning | Yes | No — implements accepted meaning |
| Claim and evidence schemas | Yes | Reads; does not author normatively |
| Verification rules | Documented in architecture and RFCs | Executed here by the reference interpreter |
| VP-CS scenario text | Authoritative prose | Executes scenarios when wired by conformance |
| Edition / specification version binding | Governed in spec | Honors pinned version at evaluation time |

When the interpreter and specification disagree on **what the protocol means**, the specification wins. The interpreter is updated—not the protocol.

Normative sources include [DOMAIN_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/01-architecture/DOMAIN_MODEL.md), [BEHAVIOR_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/01-architecture/BEHAVIOR_MODEL.md), [STATE_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/01-architecture/STATE_MODEL.md), [DATA_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/01-architecture/DATA_MODEL.md), and accepted RFCs.

---

## Relationship to veritypay-tooling and vp-spec-model

[`veritypay-tooling`](https://github.com/VerityPay-Inc/veritypay-tooling) validates that the specification corpus is **internally consistent** before semantics are executed. [`vp-spec-model`](https://github.com/VerityPay-Inc/veritypay-tooling/blob/main/docs/SPECIFICATION_MODEL.md) provides a **stable typed representation** of registries, documents, and references ([ADR-0007](https://github.com/VerityPay-Inc/veritypay-tooling/blob/main/docs/adrs/0007-specification-model-stability.md)).

| Layer | Role for this repository |
|-------|--------------------------|
| **`veritypay-spec`** | Defines *what* must be true |
| **`veritypay-tooling`** | Confirms the corpus *is well-formed* |
| **`vp-spec-model`** | Supplies shared structures for loading specification input |
| **`veritypay-reference`** | Evaluates claims *according to* loaded specification |

The reference interpreter should **consume validated specification input**—typically through `vp-spec-model` or a sibling dependency—rather than re-parsing registries and documents ad hoc. Tooling answers *"Is the spec coherent?"* The interpreter answers *"What outcome does this claim produce under that spec?"*

Validation rules and diagnostic policy remain in tooling. Verification semantics and outcome production remain here.

---

## What this repository intentionally does NOT do

| Out of scope | Where it belongs |
|--------------|------------------|
| Normative specification edits | `veritypay-spec` via RFC |
| Registry and link validation | `veritypay-tooling` |
| VP-CS scenario authoring (normative text) | `veritypay-spec` |
| Conformance suite orchestration and reporting | `veritypay-conformance` |
| Production performance, scaling, or HA | Product implementations |
| SDKs, HTTP APIs, chain adapters | Future product or SDK repos |
| Payroll UI, wallets, or merchant apps | Product repositories |
| Defining new verification outcomes or claim types | RFC in `veritypay-spec` |

If a change alters **what the protocol means**, it belongs in an RFC—not in this repository.

---

## Planned capabilities

Capabilities are delivered **capability-based** per [ROADMAP.md](ROADMAP.md)—not on a fixed calendar.

| Capability | Description | Milestone |
|------------|-------------|-----------|
| Repository scaffold | Purpose, architecture, contribution rules | A |
| Load specification model | Consume typed spec via `vp-spec-model` | B |
| Parse minimal claim | Accept a minimal claim input | C |
| Evaluate minimal claim | Run interpreter against loaded spec | D |
| Platform 1.2 model groundwork | `EvidenceSet`, `EvaluationPolicy`, `EvaluationInput` | D.5 |
| Platform 1.2 multi-evidence execution | `Interpreter::evaluate_input`, `ALL_REQUIRED` aggregation | D.6 |
| Produce verification outcome | `satisfied` / `not_satisfied` / `indeterminate` | E |
| Produce trace | Explainable evaluation steps | F |
| Conformance integration | Hooks for VP-CS runners | G |

Long-term structure: [ARCHITECTURE.md](ARCHITECTURE.md).

---

## Repository layout

```
veritypay-reference/
├── Cargo.toml                 ← Workspace manifest
├── rust-toolchain.toml        ← Pinned stable Rust
├── rustfmt.toml
├── README.md                  ← You are here
├── ARCHITECTURE.md
├── ROADMAP.md
├── CONTRIBUTING.md
├── LICENSE
├── scripts/
│   └── readiness-gate.sh      ← local fmt, clippy, test, CLI smoke
├── .github/workflows/ci.yml   ← fmt, clippy, test
├── docs/
│   └── adrs/
├── crates/
│   ├── vp-reference-cli/      ← `vp-reference` binary
│   ├── vp-reference-core/     ← contexts, errors, contracts
│   ├── vp-reference-spec/     ← specification input (Milestone B)
│   ├── vp-reference-model/    ← pure domain types
│   ├── vp-reference-interpreter/ ← verification logic (Milestone D+)
│   └── vp-reference-report/   ← human and JSON output (Milestone F+)
├── src/lib.rs                 ← workspace root (integration tests)
└── tests/                     ← workspace integration tests
```

Build and run:

```bash
cargo build -p vp-reference-cli
cargo run -p vp-reference-cli --bin vp-reference
```

Load a validated `veritypay-spec` checkout (sibling layout assumed):

```bash
cargo run -p vp-reference-cli -- load-spec --spec ../veritypay-spec
```

Example output:

```
Specification loaded

Terms: 40
RFCs: 1
Documents: 27
References: 1305
```

This loads registries, documents, and the reference graph through [`vp-spec-model`](https://github.com/VerityPay-Inc/veritypay-tooling) and exposes a path-free `SpecificationContext` to downstream crates. It does not run tooling validators, parse claims, or evaluate verification logic.

**Readiness gate** (from repository root):

```bash
./scripts/readiness-gate.sh
```

Runs `cargo fmt --check`, `cargo clippy`, `cargo test`, a CLI smoke run, and `load-spec` against sibling `../veritypay-spec` when present. Skips spec loading with a clear message if the checkout is absent.

CI runs `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace`.

---

## Links to sibling repositories

| Resource | Location |
|----------|----------|
| Specification home | [veritypay-spec](https://github.com/VerityPay-Inc/veritypay-spec) |
| Conformance model | [CONFORMANCE_MODEL.md](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md) |
| Phase II platform plan | [PHASE_II_PLATFORM_PLAN.md](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/05-governance/PHASE_II_PLATFORM_PLAN.md) |
| Platform releases | [PLATFORM_RELEASES.md](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/PLATFORM_RELEASES.md) |
| Specification tooling | [veritypay-tooling](https://github.com/VerityPay-Inc/veritypay-tooling) |
| Specification model | [SPECIFICATION_MODEL.md](https://github.com/VerityPay-Inc/veritypay-tooling/blob/main/docs/SPECIFICATION_MODEL.md) |
| VP-TERM: Reference Interpreter | [GLOSSARY — VP-TERM-027](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/00-overview/GLOSSARY.md#reference-interpreter) |

---

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md). You are implementing **specification behavior in readable code**, not inventing protocol behavior.

---

## License

See [LICENSE](LICENSE).
