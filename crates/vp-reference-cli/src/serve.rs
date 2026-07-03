//! HTTP server exposing the verify pipeline.

use std::net::SocketAddr;

use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::net::TcpListener;

use crate::error::VerifyError;
use crate::input::{ClaimFile, EvidenceFile};
use crate::output::OutputFormat;
use crate::verify::run_verify_documents;

/// Service version reported by `/health`.
pub const SERVICE_VERSION: &str = "platform-1.3-dev";

/// HTTP server bind options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServeOptions {
    host: String,
    port: u16,
}

impl ServeOptions {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn socket_addr(&self) -> Result<SocketAddr, ServeError> {
        format!("{}:{}", self.host, self.port)
            .parse()
            .map_err(|error| ServeError::bind(format!("invalid listen address: {error}")))
    }
}

/// Server startup failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServeError {
    message: String,
}

impl ServeError {
    fn bind(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    fn internal(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ServeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ServeError {}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub version: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct VerifyRequest {
    pub claim: ClaimFile,
    pub evidence: EvidenceFile,
    #[serde(default)]
    pub explain: bool,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

/// Builds the HTTP router for health and verify endpoints.
pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/verify", post(verify))
}

pub async fn run_serve(options: ServeOptions) -> Result<(), ServeError> {
    let address = options.socket_addr()?;
    let listener = TcpListener::bind(address)
        .await
        .map_err(|error| ServeError::bind(format!("failed to bind {address}: {error}")))?;

    axum::serve(listener, app())
        .await
        .map_err(|error| ServeError::internal(format!("server error: {error}")))
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "vp-reference",
        version: SERVICE_VERSION,
    })
}

pub async fn verify(
    body: Result<Json<VerifyRequest>, axum::extract::rejection::JsonRejection>,
) -> Response {
    let request = match body {
        Ok(Json(request)) => request,
        Err(rejection) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: rejection.to_string(),
                }),
            )
                .into_response();
        }
    };

    match verify_json(request) {
        Ok(body) => (StatusCode::OK, body).into_response(),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: error.message().to_owned(),
            }),
        )
            .into_response(),
    }
}

fn verify_json(request: VerifyRequest) -> Result<Json<Value>, VerifyError> {
    let output = run_verify_documents(
        request.claim,
        request.evidence,
        OutputFormat::Json,
        request.explain,
    )?;
    let value = serde_json::from_str(output.rendered())
        .map_err(|error| VerifyError::user(format!("failed to encode JSON output: {error}")))?;
    Ok(Json(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    fn sample_verify_request(explain: bool) -> VerifyRequest {
        VerifyRequest {
            claim: ClaimFile {
                claim_id: "claim-001".to_owned(),
                subject: "example-subject".to_owned(),
                assertion: crate::input::AssertionFile {
                    assertion_type: "normalized_text".to_owned(),
                    body: "Hello World".to_owned(),
                },
            },
            evidence: EvidenceFile {
                evidence_id: "evidence-001".to_owned(),
                claim_id: "claim-001".to_owned(),
                evidence_type: "document".to_owned(),
                content: crate::input::EvidenceContentFile {
                    content_type: "text/plain".to_owned(),
                    body: "  Hello   World  ".to_owned(),
                },
            },
            explain,
        }
    }

    async fn response_json(response: Response) -> Value {
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("response body");
        serde_json::from_slice(&body).expect("json body")
    }

    #[tokio::test]
    async fn health_route_returns_ok_payload() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), StatusCode::OK);
        let value = response_json(response).await;
        assert_eq!(value["status"], "ok");
        assert_eq!(value["service"], "vp-reference");
        assert_eq!(value["version"], SERVICE_VERSION);
    }

    #[tokio::test]
    async fn verify_route_returns_satisfied_json() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/verify")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&sample_verify_request(false)).expect("request json"),
                    ))
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), StatusCode::OK);
        let value = response_json(response).await;
        assert_eq!(value["claim_id"], "claim-001");
        assert_eq!(value["outcome"], "satisfied");
        assert!(value.get("assertion_type").is_none());
        assert!(value["trace"].is_array());
    }

    #[tokio::test]
    async fn verify_route_with_explain_includes_explanation_fields() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/verify")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&sample_verify_request(true)).expect("request json"),
                    ))
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), StatusCode::OK);
        let value = response_json(response).await;
        assert_eq!(value["assertion_type"], "normalized_text");
        assert_eq!(value["policy"], "ALL_REQUIRED");
        assert!(value["applied_rules"].is_array());
        assert!(value["explanation"].is_array());
    }

    #[tokio::test]
    async fn verify_route_returns_bad_request_for_malformed_input() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/verify")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"claim":{"claim_id":"claim-001","subject":"x"},"evidence":{}}"#,
                    ))
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let value = response_json(response).await;
        assert!(value["error"].is_string());
    }
}
