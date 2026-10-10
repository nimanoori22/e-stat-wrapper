use crate::models::{Classes, Values, integer, one_or_many};
use crate::{
    API_BASE_URL, EstatPolicy, Metadata, Observation, PolicyError, StatsPage, Table, TablePage,
};
use rustopus::{Arm, Backoff, Quota, Transport, TransportError, reqwest};
use serde::Deserialize;
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashMap},
    num::NonZeroU32,
    sync::Arc,
    time::Duration,
};
use tokio::sync::{Mutex, OnceCell};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid configuration or request: {0}")]
    Invalid(String),
    #[error("request failed: {0}")]
    Request(#[source] reqwest::Error),
    #[error(transparent)]
    Orchestration(#[from] rustopus::Error<PolicyError>),
    #[error("invalid e-Stat JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("unexpected e-Stat response: {0}")]
    Response(String),
}

// reqwest errors can contain appId in the URL. Strip it before rustopus stores
// a transport error (including the final error in an exhausted retry sequence).
struct RedactedTransport(reqwest::Client);

impl Transport for RedactedTransport {
    async fn send(
        &self,
        request: reqwest::Request,
    ) -> std::result::Result<rustopus::Outcome, TransportError> {
        let response = self
            .0
            .execute(request)
            .await
            .map_err(|e| TransportError::from(e.without_url()))?;
        let status = response.status();
        let headers = response.headers().clone();
        let body = response
            .bytes()
            .await
            .map_err(|e| TransportError::from(e.without_url()))?;
        Ok(rustopus::Outcome {
            status,
            headers,
            body,
        })
    }
}

type MetadataCache = Mutex<HashMap<String, Arc<OnceCell<Arc<Metadata>>>>>;

/// Clones share the same arm and metadata cache. Create once per e-Stat app ID.
#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    arm: Arm<EstatPolicy, RedactedTransport>,
    app_id: Arc<str>,
    base_url: reqwest::Url,
    metadata_cache: Arc<MetadataCache>,
}

pub struct ClientBuilder {
    app_id: String,
    base_url: String,
    quota: Option<Quota>,
    concurrency: usize,
    max_attempts: u32,
    backoff: Backoff,
    request_timeout: Duration,
    total_timeout: Duration,
    max_retry_after: Duration,
    http: Option<reqwest::Client>,
}

impl ClientBuilder {
    /// Override for mocks or an e-Stat-compatible gateway. Include a trailing slash.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }
    pub fn rate(mut self, quota: Quota) -> Self {
        self.quota = Some(quota);
        self
    }
    pub fn concurrency(mut self, n: usize) -> Self {
        self.concurrency = n;
        self
    }
    /// Includes the first attempt.
    pub fn max_attempts(mut self, n: u32) -> Self {
        self.max_attempts = n;
        self
    }
    pub fn backoff(mut self, backoff: Backoff) -> Self {
        self.backoff = backoff;
        self
    }
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }
    /// Applies to each HTTP operation, including queue time and all retries.
    pub fn total_timeout(mut self, timeout: Duration) -> Self {
        self.total_timeout = timeout;
        self
    }
    pub fn max_retry_after(mut self, timeout: Duration) -> Self {
        self.max_retry_after = timeout;
        self
    }
    /// Inject a client configured by Winter (proxy, TLS, etc.). Its timeout wins.
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http = Some(client);
        self
    }

    pub fn build(self) -> Result<Client> {
        if self.app_id.trim().is_empty() || self.concurrency == 0 || self.max_attempts == 0 {
            return Err(Error::Invalid(
                "app ID, concurrency and attempts must be nonzero".into(),
            ));
        }
        if self.request_timeout.is_zero()
            || self.total_timeout.is_zero()
            || self.max_retry_after.is_zero()
        {
            return Err(Error::Invalid("timeouts must be positive".into()));
        }
        let base_url = reqwest::Url::parse(&self.base_url)
            .map_err(|_| Error::Invalid("invalid base URL".into()))?;
        if !matches!(base_url.scheme(), "http" | "https")
            || !base_url.path().ends_with('/')
            || base_url.query().is_some()
            || base_url.fragment().is_some()
            || !base_url.username().is_empty()
            || base_url.password().is_some()
        {
            return Err(Error::Invalid(
                "base URL must be HTTP(S), end in / and contain no credentials, query or fragment"
                    .into(),
            ));
        }
        let http = match self.http {
            Some(http) => http,
            None => reqwest::Client::builder()
                .timeout(self.request_timeout)
                .user_agent(concat!("e-stat-wrapper/", env!("CARGO_PKG_VERSION")))
                .build()
                .map_err(|e| Error::Request(e.without_url()))?,
        };
        let mut arm = Arm::builder()
            .name("e-stat")
            .suckers(self.concurrency)
            .max_attempts(self.max_attempts)
            .backoff(self.backoff.clone())
            .total_timeout(self.total_timeout)
            .policy(EstatPolicy::new(self.backoff, self.max_retry_after));
        if let Some(quota) = self.quota {
            arm = arm.rate(quota);
        }
        Ok(Client {
            arm: arm.build_with(RedactedTransport(http.clone())),
            http,
            app_id: self.app_id.into(),
            base_url,
            metadata_cache: Arc::new(Mutex::new(HashMap::new())),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct TableSearch {
    pub search_word: Option<String>,
    pub stats_code: Option<String>,
    pub updated_date: Option<String>,
    pub start_position: Option<u64>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct StatsDataRequest {
    pub stats_data_id: String,
    pub filters: BTreeMap<String, String>,
    pub start_position: u64,
    pub limit: u32,
}

impl StatsDataRequest {
    pub fn new(table_id: impl Into<String>) -> Self {
        Self {
            stats_data_id: table_id.into(),
            filters: BTreeMap::new(),
            start_position: 1,
            limit: 100_000,
        }
    }
    /// e.g. `filter("cdArea", "00000")` or `filter("cdCat01", "0001")`.
    pub fn filter(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.filters.insert(name.into(), value.into());
        self
    }
    pub fn page_size(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }
}

impl Client {
    pub fn new(app_id: impl Into<String>) -> Result<Self> {
        Self::builder(app_id).build()
    }

    pub fn builder(app_id: impl Into<String>) -> ClientBuilder {
        ClientBuilder {
            app_id: app_id.into(),
            base_url: API_BASE_URL.into(),
            quota: Some(
                Quota::per_second(NonZeroU32::new(2).unwrap()).allow_burst(NonZeroU32::MIN),
            ),
            concurrency: 4,
            max_attempts: 5,
            backoff: Backoff::default(),
            request_timeout: Duration::from_secs(30),
            total_timeout: Duration::from_secs(180),
            max_retry_after: Duration::from_secs(60),
            http: None,
        }
    }

    pub(crate) async fn get(
        &self,
        endpoint: &str,
        root: &str,
        params: &[(String, String)],
    ) -> Result<Value> {
        let url = self
            .base_url
            .join(endpoint)
            .map_err(|_| Error::Invalid("invalid endpoint".into()))?;
        let request = self
            .http
            .get(url)
            .query(&[("appId", self.app_id.as_ref()), ("lang", "J")])
            .query(params)
            .build()
            .map_err(|e| Error::Request(e.without_url()))?;
        let outcome = self.arm.send(request).await?;
        let mut envelope: Value = serde_json::from_slice(&outcome.body)?;
        let payload = envelope
            .get_mut(root)
            .ok_or_else(|| Error::Response(format!("missing {root}")))?
            .take();
        let status = payload
            .get("RESULT")
            .and_then(|r| r.get("STATUS"))
            .and_then(integer)
            .ok_or_else(|| Error::Response("missing or invalid RESULT.STATUS".into()))?;
        if status > 2 {
            return Err(Error::Response(format!(
                "unrecognized RESULT.STATUS {status}"
            )));
        }
        Ok(payload)
    }

    pub async fn search_tables(&self, search: &TableSearch) -> Result<TablePage> {
        let mut params = Vec::new();
        for (key, value) in [
            ("searchWord", &search.search_word),
            ("statsCode", &search.stats_code),
            ("updatedDate", &search.updated_date),
        ] {
            if let Some(value) = value {
                params.push((key.into(), value.clone()));
            }
        }
        if let Some(start) = search.start_position {
            if start == 0 {
                return Err(Error::Invalid("start position is 1-based".into()));
            }
            params.push(("startPosition".into(), start.to_string()));
        }
        if let Some(limit) = search.limit {
            validate_limit(limit)?;
            params.push(("limit".into(), limit.to_string()));
        }
        let payload = self.get("getStatsList", "GET_STATS_LIST", &params).await?;
        if integer(&payload["RESULT"]["STATUS"]) == Some(1) {
            return Ok(TablePage {
                tables: vec![],
                total: 0,
                next_position: None,
            });
        }
        #[derive(Deserialize)]
        struct Tables {
            #[serde(rename = "TABLE_INF", default, deserialize_with = "one_or_many")]
            tables: Vec<Table>,
        }
        let data = &payload["DATALIST_INF"];
        let tables: Tables = serde_json::from_value(data.clone())?;
        Ok(TablePage {
            tables: tables.tables,
            total: required_integer(&data["NUMBER"], "NUMBER")?,
            next_position: next_key(&data["RESULT_INF"])?,
        })
    }

    /// Cached across clones; simultaneous callers share one metadata request.
    pub async fn metadata(&self, table_id: &str) -> Result<Arc<Metadata>> {
        let cell = self
            .metadata_cache
            .lock()
            .await
            .entry(table_id.into())
            .or_default()
            .clone();
        cell.get_or_try_init(|| async {
            let payload = self
                .get(
                    "getMetaInfo",
                    "GET_META_INFO",
                    &[("statsDataId".into(), table_id.into())],
                )
                .await?;
            let data = &payload["METADATA_INF"];
            let table: Table = serde_json::from_value(data["TABLE_INF"].clone())?;
            if table.id != table_id {
                return Err(Error::Response(
                    "metadata table ID differs from request".into(),
                ));
            }
            let classes: Classes = serde_json::from_value(data["CLASS_INF"].clone())?;
            Ok(Arc::new(Metadata {
                table,
                dimensions: classes.dimensions,
            }))
        })
        .await
        .cloned()
    }

    pub async fn stats_data(&self, request: &StatsDataRequest) -> Result<StatsPage> {
        validate_limit(request.limit)?;
        if request.stats_data_id.is_empty() || request.start_position == 0 {
            return Err(Error::Invalid(
                "table ID is required and start position is 1-based".into(),
            ));
        }
        for name in request.filters.keys() {
            if !valid_filter(name) {
                return Err(Error::Invalid(format!(
                    "unsupported dimension filter {name}"
                )));
            }
        }
        let mut params = vec![
            ("statsDataId".into(), request.stats_data_id.clone()),
            ("startPosition".into(), request.start_position.to_string()),
            ("limit".into(), request.limit.to_string()),
            ("metaGetFlg".into(), "N".into()),
            ("annotationGetFlg".into(), "Y".into()),
        ];
        params.extend(request.filters.iter().map(|(k, v)| (k.clone(), v.clone())));
        let payload = self.get("getStatsData", "GET_STATS_DATA", &params).await?;
        if integer(&payload["RESULT"]["STATUS"]) == Some(1) {
            return Ok(StatsPage {
                table: None,
                observations: vec![],
                total: 0,
                next_position: None,
            });
        }
        let data = &payload["STATISTICAL_DATA"];
        let values: Values = serde_json::from_value(data["DATA_INF"].clone())?;
        let table: Option<Table> = data
            .get("TABLE_INF")
            .map(|v| serde_json::from_value(v.clone()))
            .transpose()?;
        if table
            .as_ref()
            .is_some_and(|t| t.id != request.stats_data_id)
        {
            return Err(Error::Response("data table ID differs from request".into()));
        }
        Ok(StatsPage {
            table,
            observations: values.observations,
            total: required_integer(&data["RESULT_INF"]["TOTAL_NUMBER"], "TOTAL_NUMBER")?,
            next_position: next_key(&data["RESULT_INF"])?,
        })
    }

    pub async fn stats_data_all(&self, request: &StatsDataRequest) -> Result<Vec<Observation>> {
        let mut request = request.clone();
        let mut values = Vec::new();
        loop {
            let page = self.stats_data(&request).await?;
            let empty = page.observations.is_empty();
            values.extend(page.observations);
            match page.next_position {
                None => return Ok(values),
                Some(next) if next <= request.start_position || empty => {
                    return Err(Error::Response("pagination made no progress".into()));
                }
                Some(next) => request.start_position = next,
            }
        }
    }
}

fn validate_limit(limit: u32) -> Result<()> {
    if !(1..=100_000).contains(&limit) {
        return Err(Error::Invalid("page size must be 1..=100000".into()));
    }
    Ok(())
}

fn valid_filter(name: &str) -> bool {
    let prefix = if name.starts_with("cd") {
        "cd"
    } else if name.starts_with("lv") {
        "lv"
    } else {
        return false;
    };
    let mut axis = &name[prefix.len()..];
    if prefix == "cd" {
        axis = axis
            .strip_suffix("From")
            .or_else(|| axis.strip_suffix("To"))
            .unwrap_or(axis);
    }
    matches!(axis, "Tab" | "Time" | "Area") || (1..=15).any(|i| axis == format!("Cat{i:02}"))
}

fn required_integer(value: &Value, name: &str) -> Result<u64> {
    integer(value).ok_or_else(|| Error::Response(format!("missing or invalid {name}")))
}

fn next_key(info: &Value) -> Result<Option<u64>> {
    match info.get("NEXT_KEY") {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) if s.is_empty() => Ok(None),
        Some(value) => Ok(Some(required_integer(value, "NEXT_KEY")?).filter(|v| *v != 0)),
    }
}
