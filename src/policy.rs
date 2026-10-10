use crate::models::integer;
use rustopus::{Backoff, Decision, Outcome, Policy, StatusPolicy, TransportError};
use serde_json::Value;
use std::time::{Duration, SystemTime};

#[derive(Debug, thiserror::Error)]
pub enum PolicyError {
    #[error("HTTP {0}")]
    Http(u16),
    #[error("e-Stat API error {status}: {message}")]
    Api { status: u64, message: String },
}

/// HTTP handling plus e-Stat's JSON RESULT errors. A 429 always pauses the arm.
#[derive(Debug, Clone)]
pub struct EstatPolicy {
    status: StatusPolicy,
    backoff: Backoff,
    max_retry_after: Duration,
}

impl EstatPolicy {
    pub fn new(backoff: Backoff, max_retry_after: Duration) -> Self {
        Self {
            status: StatusPolicy::new().max_retry_after(max_retry_after),
            backoff,
            max_retry_after,
        }
    }
}

impl Policy for EstatPolicy {
    type Error = PolicyError;

    fn decide(
        &self,
        attempt: u32,
        result: &std::result::Result<Outcome, TransportError>,
    ) -> Decision<Self::Error> {
        if let Ok(outcome) = result {
            if outcome.status.as_u16() == 429 {
                let after = outcome
                    .headers
                    .get("retry-after")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|h| {
                        h.trim()
                            .parse::<u64>()
                            .ok()
                            .map(Duration::from_secs)
                            .or_else(|| {
                                httpdate::parse_http_date(h).ok().map(|date| {
                                    date.duration_since(SystemTime::now()).unwrap_or_default()
                                })
                            })
                    })
                    .unwrap_or_else(|| self.backoff.delay(attempt))
                    .min(self.max_retry_after);
                return Decision::Retry { after: Some(after) };
            }
            // Also inspect the body on HTTP errors so callers get the API code.
            if let Ok(Value::Object(root)) = serde_json::from_slice::<Value>(&outcome.body) {
                for payload in root.values() {
                    if let Some(status) = payload
                        .get("RESULT")
                        .and_then(|r| r.get("STATUS"))
                        .and_then(integer)
                    {
                        if matches!(status, 200..=203 | 299) {
                            return Decision::Retry { after: None };
                        }
                        if status >= 100 {
                            return Decision::Fail(PolicyError::Api {
                                status,
                                message: payload["RESULT"]["ERROR_MSG"]
                                    .as_str()
                                    .unwrap_or("unknown error")
                                    .to_owned(),
                            });
                        }
                    }
                }
            }
        }
        match self.status.decide(attempt, result) {
            Decision::Accept => Decision::Accept,
            Decision::Retry { after } => Decision::Retry { after },
            Decision::Fail(error) => Decision::Fail(PolicyError::Http(error.status.as_u16())),
            _ => Decision::Fail(PolicyError::Http(0)),
        }
    }
}
