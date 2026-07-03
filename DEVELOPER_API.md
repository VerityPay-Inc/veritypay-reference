# Developer API

Developer-facing surfaces for running verification against the VerityPay reference interpreter.

These interfaces are **convenience layers** over the interpreter in `vp-reference-interpreter`. They do **not** define protocol meaning. Normative semantics live in [`veritypay-spec`](https://github.com/VerityPay-Inc/veritypay-spec).

---

## Stability

| Surface | Status |
|---------|--------|
| **Interpreter library** (`Interpreter::evaluate`, `Interpreter::evaluate_input`) | Stable public contract per [ADR-0007](docs/adrs/0007-reference-interpreter-public-contract.md) |
| **CLI** (`vp-reference verify`, `vp-reference serve`) | **Experimental** — flags, JSON shapes, and HTTP paths may change |
| **HTTP server** | **Experimental** — same as CLI |

Protocol outcomes (`satisfied`, `not_satisfied`, `indeterminate`) and rule behavior are governed by accepted RFCs in `veritypay-spec`, not by this document.

When this repository and the specification disagree, **the specification wins**.

---

## 1. CLI

Build the binary:

```bash
cargo build -p vp-reference-cli
```

### `vp-reference verify`

Verify one claim against one evidence file.

```bash
vp-reference verify \
  --claim <path-to-claim.json> \
  --evidence <path-to-evidence.json> \
  [--format human|json] \
  [--explain]
```

| Flag | Default | Description |
|------|---------|-------------|
| `--claim` | *(required)* | Path to claim JSON |
| `--evidence` | *(required)* | Path to evidence JSON |
| `--format` | `human` | Output format: `human` or `json` |
| `--explain` | off | Include step-by-step explanation derived from the evaluation trace |

**Exit codes**

| Code | Meaning |
|------|---------|
| `0` | Verification ran successfully (outcome may be `satisfied`, `not_satisfied`, or `indeterminate`) |
| `2` | User error — missing file, invalid JSON, or invalid claim/evidence shape |

Example using repository fixtures:

```bash
cargo run -p vp-reference-cli --bin vp-reference -- verify \
  --claim examples/claim.normalized_text.json \
  --evidence examples/evidence.normalized_text.json \
  --format human
```

Compact human output:

```text
✓ satisfied claim-001
Reason: All 1 applicable evidence envelope(s) satisfied (ALL_REQUIRED)
```

With `--explain`, human output adds assertion type, evidence id, policy, applied rules, numbered explanation steps, and a reason block. With `--format json`, output matches the JSON shapes in [Examples](#examples) below.

### Input JSON shapes

**Claim file**

```json
{
  "claim_id": "claim-001",
  "subject": "example-subject",
  "assertion": {
    "assertion_type": "normalized_text",
    "body": "Hello World"
  }
}
```

**Evidence file**

```json
{
  "evidence_id": "evidence-001",
  "claim_id": "claim-001",
  "evidence_type": "document",
  "content": {
    "content_type": "text/plain",
    "body": "  Hello   World  "
  }
}
```

Supported assertion types today: `body_equality`, `minimal` (alias), `normalized_text`. See [README.md](README.md) for the platform matrix.

---

## 2. HTTP server

### `vp-reference serve`

Expose the same verify pipeline over HTTP.

```bash
vp-reference serve [--host <host>] [--port <port>]
```

| Flag | Default | Description |
|------|---------|-------------|
| `--host` | `127.0.0.1` | Listen address |
| `--port` | `8787` | Listen port |

Example:

```bash
cargo run -p vp-reference-cli --bin vp-reference -- serve --host 127.0.0.1 --port 8787
```

### `GET /health`

Liveness check.

**Response** `200 OK`

```json
{
  "status": "ok",
  "service": "vp-reference",
  "version": "platform-1.3-dev"
}
```

Example:

```bash
curl -s http://127.0.0.1:8787/health
```

### `POST /verify`

Run verification for one claim and one evidence object.

**Request** `Content-Type: application/json`

```json
{
  "claim": {
    "claim_id": "claim-001",
    "subject": "example-subject",
    "assertion": {
      "assertion_type": "normalized_text",
      "body": "Hello World"
    }
  },
  "evidence": {
    "evidence_id": "evidence-001",
    "claim_id": "claim-001",
    "evidence_type": "document",
    "content": {
      "content_type": "text/plain",
      "body": "  Hello   World  "
    }
  },
  "explain": false
}
```

| Field | Required | Default | Description |
|-------|----------|---------|-------------|
| `claim` | yes | — | Same shape as CLI claim file |
| `evidence` | yes | — | Same shape as CLI evidence file |
| `explain` | no | `false` | When `true`, response matches `verify --format json --explain` |

**Response** `200 OK` — JSON verification result (see [Examples](#examples)).

**Error responses**

| Status | When |
|--------|------|
| `400 Bad Request` | Malformed JSON, missing fields, or invalid claim/evidence shape |
| `500 Internal Server Error` | Unexpected server failure (bind errors surface at process start) |

Error body:

```json
{
  "error": "description of the problem"
}
```

Example:

```bash
curl -s http://127.0.0.1:8787/verify \
  -H 'content-type: application/json' \
  -d @- <<'EOF'
{
  "claim": {
    "claim_id": "claim-001",
    "subject": "example-subject",
    "assertion": {
      "assertion_type": "normalized_text",
      "body": "Hello World"
    }
  },
  "evidence": {
    "evidence_id": "evidence-001",
    "claim_id": "claim-001",
    "evidence_type": "document",
    "content": {
      "content_type": "text/plain",
      "body": "  Hello   World  "
    }
  },
  "explain": true
}
EOF
```

The HTTP handler reuses `run_verify_documents` and the same JSON renderer as the CLI. No interpreter logic is duplicated in the server layer.

---

## 3. Library API

For embedders, integrators, and `veritypay-conformance`, use the interpreter crates directly.

**Crates**

| Crate | Role |
|-------|------|
| `vp-reference-model` | Domain types: `Claim`, `Evidence`, `Outcome`, `VerificationResult` |
| `vp-reference-core` | `EvaluationContext`, `EvaluationInput`, `SpecificationContext` |
| `vp-reference-interpreter` | `Interpreter` — executes rules and returns `VerificationResult` |

### `Interpreter::evaluate`

```text
EvaluationContext → Interpreter::evaluate(&EvaluationContext) → VerificationResult
```

**Use when:**

- Evaluating **one claim** against **one evidence** envelope
- Matching the Platform 1.1 public contract ([ADR-0007](docs/adrs/0007-reference-interpreter-public-contract.md))
- Building a conformance oracle or custom harness with explicit per-envelope control

**Inputs:** `EvaluationContext` with `specification`, `claim`, `evidence`, and `options`.

### `Interpreter::evaluate_input`

```text
EvaluationInput → Interpreter::evaluate_input(&EvaluationInput) → VerificationResult
```

**Use when:**

- Evaluating **one claim** against an **evidence set** (zero or more envelopes)
- Applying an **evaluation policy** — today `ALL_REQUIRED` per [VP-RFC-0004](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0004-evidence-evaluation-policies.md)
- Matching what the CLI and HTTP server use internally (single evidence in an `EvidenceSet`)

**Inputs:** `EvaluationInput` with `specification`, `claim`, `evidence_set`, `evaluation_policy`, and `options`.

### Choosing between them

| Situation | Entrypoint |
|-----------|------------|
| Single evidence, legacy Platform 1.1 path | `evaluate` |
| Multi-evidence or `ALL_REQUIRED` aggregation | `evaluate_input` |
| CLI / HTTP verify command | `evaluate_input` (one envelope, `ALL_REQUIRED`) |
| `veritypay-conformance` reference oracle | `evaluate` via `EvaluationContext` built from scenario fixtures |

Both entrypoints dispatch by `assertion_type` through the assertion evaluator registry ([ADR-0009](docs/adrs/0009-assertion-evaluator-architecture.md)). Outcome vocabulary is always `satisfied`, `not_satisfied`, or `indeterminate`.

---

## 4. Examples

All examples use `assertion_type: "normalized_text"` and **VP-RULE-0011** semantics (draft [VP-RFC-0011](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0011-normalized-text-assertion.md)).

### Satisfied — trim and whitespace collapse

**Claim body:** `"Hello World"`  
**Evidence body:** `"  Hello   World  "`

After normalization both bodies match.

```bash
vp-reference verify \
  --claim examples/claim.normalized_text.json \
  --evidence examples/evidence.normalized_text.json \
  --format json
```

```json
{
  "claim_id": "claim-001",
  "outcome": "satisfied",
  "reason": "All 1 applicable evidence envelope(s) satisfied (ALL_REQUIRED)",
  "trace": [ "..."]
}
```

### Not satisfied — case-sensitive mismatch

**Claim body:** `"Hello"`  
**Evidence body:** `"hello"`

```json
{
  "claim": {
    "claim_id": "claim-001",
    "subject": "example-subject",
    "assertion": {
      "assertion_type": "normalized_text",
      "body": "Hello"
    }
  },
  "evidence": {
    "evidence_id": "evidence-001",
    "claim_id": "claim-001",
    "evidence_type": "document",
    "content": {
      "content_type": "text/plain",
      "body": "hello"
    }
  }
}
```

```json
{
  "claim_id": "claim-001",
  "outcome": "not_satisfied",
  "reason": "At least one applicable evidence envelope is not_satisfied (ALL_REQUIRED)",
  "trace": [ "..." ]
}
```

### Indeterminate — whitespace-only evidence

**Claim body:** `"Hello"`  
**Evidence body:** `"     "` (whitespace only)

```json
{
  "claim": {
    "claim_id": "claim-001",
    "subject": "example-subject",
    "assertion": {
      "assertion_type": "normalized_text",
      "body": "Hello"
    }
  },
  "evidence": {
    "evidence_id": "evidence-001",
    "claim_id": "claim-001",
    "evidence_type": "document",
    "content": {
      "content_type": "text/plain",
      "body": "     "
    }
  }
}
```

```json
{
  "claim_id": "claim-001",
  "outcome": "indeterminate",
  "reason": "At least one applicable evidence envelope is indeterminate with no not_satisfied (ALL_REQUIRED)",
  "trace": [ "..." ]
}
```

### With `--explain` / `"explain": true`

Adds informative fields (not normative protocol objects):

```json
{
  "claim_id": "claim-001",
  "outcome": "satisfied",
  "reason": "All 1 applicable evidence envelope(s) satisfied (ALL_REQUIRED)",
  "assertion_type": "normalized_text",
  "evidence": [
    {
      "evidence_id": "evidence-001",
      "claim_id": "claim-001",
      "content_type": "text/plain"
    }
  ],
  "policy": "ALL_REQUIRED",
  "applied_rules": ["VP-RULE-0002", "VP-RULE-0011"],
  "explanation": [
    "Evidence claim_id matched claim claim_id.",
    "Evidence text was normalized.",
    "Normalized assertion body matched normalized evidence body.",
    "ALL_REQUIRED aggregation returned satisfied."
  ],
  "trace": [ "..." ]
}
```

Explanation text is **informative only**. It is derived from `assertion_type`, outcome, evidence shape, and trace `rule_reference` values.

---

## Related documents

| Document | Role |
|----------|------|
| [README.md](README.md) | Repository overview and quick-start commands |
| [ADR-0007](docs/adrs/0007-reference-interpreter-public-contract.md) | Stable interpreter contract |
| [ADR-0009](docs/adrs/0009-assertion-evaluator-architecture.md) | Assertion type dispatch |
| [CONFORMANCE_MODEL.md](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md) | VP-CS and oracle expectations |
| [examples/](examples/) | Sample claim and evidence JSON files |
