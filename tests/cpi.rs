//! Synthetic e-Stat-shaped responses: these test behavior, not live CPI values.
use e_stat_wrapper::rustopus::{Backoff, Quota, reqwest};
use e_stat_wrapper::{Client, CpiBase, CpiIndicator, CpiMeasure, CpiQuery, Error, Month};
use serde_json::{Value, json};
use std::{
    num::NonZeroU32,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use wiremock::{
    Mock, MockServer, Request, Respond, ResponseTemplate,
    matchers::{method, path, query_param},
};

async fn client(server: &MockServer, concurrency: usize) -> Client {
    Client::builder("test-only-app-id")
        .base_url(format!("{}/", server.uri()))
        .http_client(reqwest::Client::builder().no_proxy().build().unwrap())
        .rate(Quota::per_second(NonZeroU32::new(1000).unwrap()))
        .concurrency(concurrency)
        .backoff(Backoff::constant(Duration::from_millis(10)))
        .max_attempts(3)
        .total_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

fn metadata() -> Value {
    json!({"GET_META_INFO": {
        "RESULT": {"STATUS": 0, "ERROR_MSG": "正常に終了しました。"},
        "METADATA_INF": {
            "TABLE_INF": {"@id": "0004052037", "TITLE": "CPI synthetic fixture"},
            "CLASS_INF": {"CLASS_OBJ": [
                {"@id": "tab", "@name": "表章項目", "CLASS": [
                    {"@code": "index", "@name": "指数"},
                    {"@code": "yoy", "@name": "前年同月比"}
                ]},
                {"@id": "cat01", "@name": "品目", "CLASS": [
                    {"@code": "0201", "@name": "総合", "@level": "1"},
                    {"@code": "0001", "@name": "0001 総合", "@level": "1"},
                    {"@code": "core", "@name": "生鮮食品を除く総合"},
                    {"@code": "core-core", "@name": "生鮮食品及びエネルギーを除く総合"},
                    {"@code": "wrong-food", "@name": "食料（酒類を除く）及びエネルギーを除く総合"},
                    {"@code": "adjusted", "@name": "総合（季節調整値）"}
                ]},
                {"@id": "area", "@name": "地域", "CLASS": [
                    {"@code": "japan", "@name": "全国"},
                    {"@code": "tokyo", "@name": "東京都区部"}
                ]},
                {"@id": "time", "@name": "時間", "CLASS": [
                    {"@code": "2025-02", "@name": "2025年2月"},
                    {"@code": "2025-01", "@name": "2025年1月"},
                    {"@code": "2025-annual", "@name": "2025年"},
                    {"@code": "2025-fiscal", "@name": "2025年度"},
                    {"@code": "2025-quarter", "@name": "2025年1～3月"}
                ]}
            ]}
        }
    }})
}

fn page(rows: Value, next: Value) -> Value {
    json!({"GET_STATS_DATA": {
        "RESULT": {"STATUS": 0},
        "STATISTICAL_DATA": {
            "TABLE_INF": {"@id": "0004052037"},
            "RESULT_INF": {"TOTAL_NUMBER": "2", "NEXT_KEY": next},
            "DATA_INF": {"VALUE": rows}
        }
    }})
}

fn row(item: &str, tab: &str, month: &str, value: &str) -> Value {
    json!({"@cat01": item, "@tab": tab, "@area": "japan", "@time": month, "$": value})
}

fn query() -> CpiQuery {
    CpiQuery::new(CpiBase::Base2025, CpiMeasure::Index)
        .between(Month::new(2025, 1).unwrap(), Month::new(2025, 2).unwrap())
}

async fn mount_metadata(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/getMetaInfo"))
        .and(query_param("statsDataId", "0004052037"))
        .and(query_param("lang", "J"))
        .respond_with(ResponseTemplate::new(200).set_body_json(metadata()))
        .expect(1)
        .mount(server)
        .await;
}

#[tokio::test]
async fn all_three_indicators_select_exact_national_monthly_categories_and_cache_metadata() {
    let server = MockServer::start().await;
    mount_metadata(&server).await;
    for (item, value) in [("0001", "100.1"), ("core", "100.2"), ("core-core", "100.3")] {
        Mock::given(method("GET"))
            .and(path("/getStatsData"))
            .and(query_param("appId", "test-only-app-id"))
            .and(query_param("statsDataId", "0004052037"))
            .and(query_param("cdCat01", item))
            .and(query_param("cdArea", "japan"))
            .and(query_param("cdTab", "index"))
            .and(query_param("cdTime", "2025-01,2025-02"))
            .respond_with(ResponseTemplate::new(200).set_body_json(page(
                json!([
                    row(item, "index", "2025-02", value),
                    row(item, "index", "2025-01", "100")
                ]),
                Value::Null,
            )))
            .expect(1)
            .mount(&server)
            .await;
    }
    let series = client(&server, 2)
        .await
        .cpi_indicators(&query())
        .await
        .unwrap();
    for (i, series) in series.iter().enumerate() {
        assert_eq!(series.indicator, CpiIndicator::ALL[i]);
        assert_eq!(series.base_year, 2025);
        assert_eq!(series.area_name, "全国");
        assert_eq!(series.unit, "2025=100");
        assert_eq!(series.observations.len(), 2);
        assert_eq!(series.observations[0].month, Month::new(2025, 1).unwrap());
        assert_eq!(series.observations[0].value, Some(100.0));
        let expected = [100.1, 100.2, 100.3][i];
        assert!((series.observations[1].value.unwrap() - expected).abs() < 1e-9);
    }
}

#[tokio::test]
async fn paginates_singleton_values_and_preserves_missing_data_and_annotations() {
    let server = MockServer::start().await;
    mount_metadata(&server).await;
    let mut first = row("0001", "index", "2025-01", "***");
    first["@annotation"] = json!("suppressed");
    for (start, rows, next) in [
        ("1", first, json!("2")),
        ("2", row("0001", "index", "2025-02", "101.5"), json!("")),
    ] {
        Mock::given(path("/getStatsData"))
            .and(query_param("startPosition", start))
            .respond_with(ResponseTemplate::new(200).set_body_json(page(rows, next)))
            .expect(1)
            .mount(&server)
            .await;
    }
    let series = client(&server, 1)
        .await
        .headline_cpi(&query())
        .await
        .unwrap();
    assert_eq!(series.observations.len(), 2);
    assert_eq!(series.observations[0].value, None);
    assert_eq!(series.observations[0].raw_value, "***");
    assert_eq!(
        series.observations[0].annotation.as_deref(),
        Some("suppressed")
    );
    assert_eq!(series.observations[1].value, Some(101.5));
}

#[tokio::test]
async fn year_on_year_uses_official_tab_and_accepts_negative_values() {
    let server = MockServer::start().await;
    mount_metadata(&server).await;
    Mock::given(path("/getStatsData"))
        .and(query_param("cdTab", "yoy"))
        .respond_with(ResponseTemplate::new(200).set_body_json(page(
            json!([
                row("core", "yoy", "2025-01", "-0.2"),
                row("core", "yoy", "2025-02", "0")
            ]),
            Value::Null,
        )))
        .expect(1)
        .mount(&server)
        .await;
    let mut query = query();
    query.measure = CpiMeasure::YearOnYear;
    let series = client(&server, 1)
        .await
        .cpi_ex_fresh_food(&query)
        .await
        .unwrap();
    assert_eq!(series.unit, "%");
    assert_eq!(series.observations[0].value, Some(-0.2));
}

struct RetryThenSucceed {
    calls: Arc<AtomicUsize>,
    first: ResponseTemplate,
}

impl Respond for RetryThenSucceed {
    fn respond(&self, _: &Request) -> ResponseTemplate {
        if self.calls.fetch_add(1, Ordering::SeqCst) == 0 {
            self.first.clone()
        } else {
            ResponseTemplate::new(200).set_body_json(metadata())
        }
    }
}

#[tokio::test]
async fn retries_http_429_http_503_and_body_level_database_errors() {
    for first in [
        ResponseTemplate::new(429).insert_header("Retry-After", "0"),
        ResponseTemplate::new(503),
        ResponseTemplate::new(200).set_body_json(json!({"GET_META_INFO": {"RESULT": {"STATUS": 200, "ERROR_MSG": "database unavailable"}}})),
    ] {
        let server = MockServer::start().await;
        let calls = Arc::new(AtomicUsize::new(0));
        Mock::given(path("/getMetaInfo"))
            .respond_with(RetryThenSucceed { calls: calls.clone(), first })
            .expect(2).mount(&server).await;
        client(&server, 1).await.metadata("0004052037").await.unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }
}

#[tokio::test]
async fn authentication_errors_fail_immediately_even_with_http_200() {
    let server = MockServer::start().await;
    Mock::given(path("/getMetaInfo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"GET_META_INFO": {"RESULT": {"STATUS": "100", "ERROR_MSG": "invalid application ID"}}})))
        .expect(1).mount(&server).await;
    let error = client(&server, 1)
        .await
        .metadata("0004052037")
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        Error::Orchestration(e_stat_wrapper::rustopus::Error::Policy(
            e_stat_wrapper::PolicyError::Api { status: 100, .. }
        ))
    ));
}

#[tokio::test]
async fn exhausted_retries_are_bounded() {
    let server = MockServer::start().await;
    Mock::given(path("/getMetaInfo"))
        .respond_with(ResponseTemplate::new(503))
        .expect(3)
        .mount(&server)
        .await;
    let error = client(&server, 1)
        .await
        .metadata("0004052037")
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        Error::Orchestration(e_stat_wrapper::rustopus::Error::Exhausted { attempts: 3, .. })
    ));
}

#[tokio::test]
async fn rejects_nonadvancing_pagination_and_wrong_region() {
    for rows in [row("0001", "index", "2025-01", "100"), {
        let mut row = row("0001", "index", "2025-01", "100");
        row["@area"] = json!("tokyo");
        row
    }] {
        let server = MockServer::start().await;
        mount_metadata(&server).await;
        let next = if rows["@area"] == "tokyo" {
            Value::Null
        } else {
            json!(1)
        };
        Mock::given(path("/getStatsData"))
            .respond_with(ResponseTemplate::new(200).set_body_json(page(rows, next)))
            .expect(1)
            .mount(&server)
            .await;
        assert!(matches!(
            client(&server, 1).await.headline_cpi(&query()).await,
            Err(Error::Response(_))
        ));
    }
}

#[tokio::test]
async fn duplicate_periods_and_invalid_ranges_are_rejected() {
    let server = MockServer::start().await;
    mount_metadata(&server).await;
    Mock::given(path("/getStatsData"))
        .respond_with(ResponseTemplate::new(200).set_body_json(page(
            json!([
                row("0001", "index", "2025-01", "100"),
                row("0001", "index", "2025-01", "101")
            ]),
            Value::Null,
        )))
        .expect(1)
        .mount(&server)
        .await;
    let client = client(&server, 1).await;
    assert!(matches!(
        client.headline_cpi(&query()).await,
        Err(Error::Response(_))
    ));
    let invalid = query().between(Month::new(2025, 2).unwrap(), Month::new(2025, 1).unwrap());
    assert!(matches!(
        client.headline_cpi(&invalid).await,
        Err(Error::Invalid(_))
    ));
}

struct TimedResponses {
    starts: Arc<std::sync::Mutex<Vec<std::time::Instant>>>,
}

impl Respond for TimedResponses {
    fn respond(&self, _: &Request) -> ResponseTemplate {
        self.starts.lock().unwrap().push(std::time::Instant::now());
        ResponseTemplate::new(200)
            .set_body_json(json!({"GET_STATS_DATA": {"RESULT": {"STATUS": 1}}}))
            .set_delay(Duration::from_millis(250))
    }
}

#[tokio::test]
async fn concurrency_limit_is_shared_across_client_clones() {
    let server = MockServer::start().await;
    let starts = Arc::new(std::sync::Mutex::new(Vec::new()));
    Mock::given(path("/getStatsData"))
        .respond_with(TimedResponses {
            starts: starts.clone(),
        })
        .expect(6)
        .mount(&server)
        .await;
    let client = client(&server, 2).await;
    let mut tasks = Vec::new();
    for _ in 0..6 {
        let client = client.clone();
        tasks.push(tokio::spawn(async move {
            client
                .stats_data(&e_stat_wrapper::StatsDataRequest::new("0004052037"))
                .await
                .unwrap()
        }));
    }
    for task in tasks {
        assert!(task.await.unwrap().observations.is_empty());
    }
    let starts = starts.lock().unwrap();
    assert_eq!(starts.len(), 6);
    // With 250ms bodies, every third arrival must wait for a preceding slot.
    for i in 0..4 {
        assert!(
            starts[i + 2].duration_since(starts[i]) >= Duration::from_millis(200),
            "more than two requests were in flight"
        );
    }
}

#[tokio::test]
async fn total_deadline_stops_slow_requests() {
    let server = MockServer::start().await;
    Mock::given(path("/getMetaInfo"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(metadata())
                .set_delay(Duration::from_secs(1)),
        )
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder("test-only-app-id")
        .base_url(format!("{}/", server.uri()))
        .http_client(reqwest::Client::builder().no_proxy().build().unwrap())
        .total_timeout(Duration::from_millis(30))
        .build()
        .unwrap();
    assert!(matches!(
        client.metadata("0004052037").await,
        Err(Error::Orchestration(
            e_stat_wrapper::rustopus::Error::Deadline
        ))
    ));
}

#[tokio::test]
async fn more_than_100_months_are_split_into_exact_code_batches() {
    let server = MockServer::start().await;
    let mut meta = metadata();
    let classes: Vec<Value> = (0..101).map(|i| {
        json!({"@code": format!("t{i:03}"), "@name": format!("{}年{}月", 2000 + i / 12, i % 12 + 1)})
    }).collect();
    meta["GET_META_INFO"]["METADATA_INF"]["CLASS_INF"]["CLASS_OBJ"][3]["CLASS"] = json!(classes);
    Mock::given(path("/getMetaInfo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(meta))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(path("/getStatsData"))
        .respond_with(|request: &Request| {
            let time = request
                .url
                .query_pairs()
                .find(|(k, _)| k == "cdTime")
                .unwrap()
                .1
                .into_owned();
            let codes: Vec<_> = time.split(',').collect();
            assert!(codes.len() <= 100);
            ResponseTemplate::new(200).set_body_json(page(
                json!(
                    codes
                        .iter()
                        .map(|c| row("0001", "index", c, "100"))
                        .collect::<Vec<_>>()
                ),
                Value::Null,
            ))
        })
        .expect(2)
        .mount(&server)
        .await;
    let series = client(&server, 1)
        .await
        .headline_cpi(&CpiQuery::new(CpiBase::Base2025, CpiMeasure::Index))
        .await
        .unwrap();
    assert_eq!(series.observations.len(), 101);
}
