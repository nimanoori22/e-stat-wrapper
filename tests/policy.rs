use e_stat_wrapper::rustopus::{
    Arm, Backoff, Decision, Outcome, Policy, Transport, TransportError, reqwest,
};
use e_stat_wrapper::{EstatPolicy, PolicyError};
use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::time::Instant;

fn outcome(status: u16, retry_after: Option<&str>) -> Outcome {
    let mut headers = reqwest::header::HeaderMap::new();
    if let Some(value) = retry_after {
        headers.insert("retry-after", value.parse().unwrap());
    }
    Outcome {
        status: reqwest::StatusCode::from_u16(status).unwrap(),
        headers,
        body: Vec::new().into(),
    }
}

#[test]
fn rate_limits_pause_the_arm_even_without_retry_after() {
    let policy = EstatPolicy::new(
        Backoff::exponential(Duration::from_secs(1)).jitter(false),
        Duration::from_secs(60),
    );
    for (attempt, expected) in [(1, 1), (2, 2), (3, 4)] {
        assert!(
            matches!(policy.decide(attempt, &Ok(outcome(429, None))), Decision::Retry { after: Some(d) } if d == Duration::from_secs(expected))
        );
    }
    assert!(
        matches!(policy.decide(1, &Ok(outcome(429, Some("120")))), Decision::Retry { after: Some(d) } if d == Duration::from_secs(60))
    );
}

#[test]
fn retry_after_http_dates_are_supported() {
    let policy = EstatPolicy::new(
        Backoff::constant(Duration::from_secs(1)),
        Duration::from_secs(60),
    );
    let future = httpdate::fmt_http_date(std::time::SystemTime::now() + Duration::from_secs(30));
    assert!(
        matches!(policy.decide(1, &Ok(outcome(429, Some(&future)))), Decision::Retry { after: Some(d) } if d >= Duration::from_secs(28) && d <= Duration::from_secs(30))
    );
}

#[test]
fn nonretryable_http_errors_fail() {
    let policy = EstatPolicy::new(Backoff::default(), Duration::from_secs(60));
    assert!(matches!(
        policy.decide(1, &Ok(outcome(403, None))),
        Decision::Fail(PolicyError::Http(403))
    ));
}

struct Scripted {
    calls: AtomicUsize,
    starts: Arc<Mutex<Vec<Instant>>>,
}

impl Transport for Scripted {
    async fn send(&self, _: reqwest::Request) -> Result<Outcome, TransportError> {
        self.starts.lock().unwrap().push(Instant::now());
        let call = self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(outcome(if call == 0 { 429 } else { 200 }, None))
    }
}

#[tokio::test(start_paused = true)]
async fn one_429_pauses_other_requests_on_the_same_arm() {
    let starts = Arc::new(Mutex::new(Vec::new()));
    let backoff = Backoff::constant(Duration::from_secs(5));
    let arm = Arm::builder()
        .suckers(1)
        .max_attempts(2)
        .backoff(backoff.clone())
        .policy(EstatPolicy::new(backoff, Duration::from_secs(60)))
        .build_with(Scripted {
            calls: AtomicUsize::new(0),
            starts: starts.clone(),
        });
    let http = reqwest::Client::new();
    let request1 = http.get("https://example.invalid/first").build().unwrap();
    let request2 = http.get("https://example.invalid/second").build().unwrap();
    let clone = arm.clone();
    let (first, second) = tokio::join!(arm.send(request1), clone.send(request2));
    first.unwrap();
    second.unwrap();
    let starts = starts.lock().unwrap();
    assert_eq!(starts.len(), 3);
    assert!(starts[1].duration_since(starts[0]) >= Duration::from_secs(5));
    assert!(starts[2].duration_since(starts[0]) >= Duration::from_secs(5));
}
