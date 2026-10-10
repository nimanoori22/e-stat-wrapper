# e-stat-wrapper

Async Rust client for Japan's e-Stat API v3 with typed national CPI series,
pagination, retries, and shared rate and concurrency limits. Clones share
request orchestration and a metadata cache through `rustopus`.

## Installation

Add the Git dependency to your application's `Cargo.toml`:

```toml
[dependencies]
e-stat-wrapper = { git = "https://github.com/nimanoori22/e-stat-wrapper.git", branch = "main" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Use a current stable Rust toolchain with support for edition 2024. `rustopus` 0.1.0 is fetched from
crates.io; no sibling checkout is needed.

## Quick start

Register an application through [e-Stat's My Page](https://www.e-stat.go.jp/mypage/)
and set its application ID in your environment:

```sh
export ESTAT_APP_ID="your-application-id"
```

The crate reads the ID supplied to the client and does not automatically load
`.env` files. Local `.env` files are git-ignored.

```rust
use e_stat_wrapper::{Client, CpiBase, CpiMeasure, CpiQuery, Month};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder(std::env::var("ESTAT_APP_ID")?)
        .concurrency(4)
        .max_attempts(5)
        .build()?;

    let query = CpiQuery::new(CpiBase::Base2025, CpiMeasure::YearOnYear)
        .between(Month::new(2026, 1)?, Month::new(2026, 6)?);
    let [headline, core, core_core] = client.cpi_indicators(&query).await?;
    // Individual methods: headline_cpi, cpi_ex_fresh_food,
    // cpi_ex_fresh_food_and_energy. Each returns a CpiSeries.
    println!("Headline CPI: {:?}", headline.observations);
    Ok(())
}
```

To run the included example:

```sh
git clone https://github.com/nimanoori22/e-stat-wrapper.git
cd e-stat-wrapper
cargo run --example cpi
```

## CPI series

Use `CpiMeasure::Index` for index levels or `YearOnYear` for the officially
published year-on-year percentage. These are nationwide, monthly, unadjusted
series. The fresh-food-and-energy exclusion is distinct from excluding all
food and energy. Returned series record the table, base year, selected codes,
labels and unit. Missing/suppressed cells have `value: None` and retain their
raw value and annotation. Observations are sorted oldest to newest.

`CpiBase::Base2025` uses table `0004052037`; `Base2020` uses `0003427113`.
e-Stat announced the 2025-base switch for August 2026; 2020-base updates end
after December 2026. Item, area and measure codes are resolved from the table's
Japanese metadata, cached across clones. Headline is explicitly restricted to
the primary item `0001` because the goods/services classification also has an
item named "All items" (`0201`), at the same hierarchy level. Its label is still
validated against metadata. Only monthly time labels are selected;
annual, fiscal-year and quarterly records are excluded. Exact time filters are
split into batches of at most 100, and each batch follows `NEXT_KEY` pagination.
An empty/unavailable time range returns an empty series.

## Request orchestration

Defaults: evenly spaced quota of 2 requests/second, 4 concurrent requests,
5 attempts including the first, exponential backoff starting at 500ms with
jitter and a 30s cap. Each HTTP attempt has a 30s timeout; each operation has a
180s total deadline covering queueing and retries. These are configurable
client defaults, not published e-Stat quotas.

HTTP 429 pauses the shared arm, honoring `Retry-After` seconds or HTTP dates,
capped at 60s. Without a usable header, it pauses the arm using exponential
backoff. Other transient HTTP/transport failures use rustopus's status policy.
Documented e-Stat database/internal error codes `200..203` and `299` also retry;
authentication and parameter errors fail immediately. Cancellation drops the
caller future. A deadline applies to each page, not the entire multi-page fetch.

Construct one client per application ID and share clones of it.
Creating separate clients creates separate limits. For a custom proxy or HTTP
configuration, inject `rustopus::reqwest::Client` with `.http_client(...)`.
Transport/request errors strip their URL to avoid logging the application ID.
Do not log raw API responses (e-Stat may echo request parameters).

The HTTP types use rustopus's re-exported reqwest 0.13.5. Optional `query`,
`gzip` and `socks` features are enabled on the same version through Cargo feature
unification, including support for SOCKS proxies configured in the environment.

## Tests

Offline mock tests cover all three exact CPI selections, shared metadata,
pagination/singleton JSON, missing values, official YoY selection, retries,
authentication failures, bounded attempts, concurrency across clones, shared
429 pauses, deadlines, duplicate periods and time-code batching.

```sh
cargo test
```

Live tests are ignored by default and require `ESTAT_APP_ID`. They retrieve
all 12 monthly observations for each 2025-base indicator in 2025 and verify
the base-year annual mean is approximately 100, plus check all three official
YoY series for January–June 2026. They make real API requests:

```sh
cargo test --test live_cpi -- --ignored --test-threads=1 --nocapture
cargo run --example cpi
```

Serial execution keeps the separately constructed live-test clients from
combining their per-client request quotas. Mock CPI values are synthetic and
do not claim to reproduce published statistics. Live tests do not run during
compilation checks.

## Sources and attribution

- [API v3 specification](https://www.e-stat.go.jp/api/api-info/e-stat-manual3-0)
- [2025-base table and migration announcement](https://www.e-stat.go.jp/api/info-cat/news/httpswwwe-statgojpapiinfo-catnewscpi-info202607)
- [2025-base CPI table](https://www.e-stat.go.jp/en/stat-search/database?statdisp_id=0004052037)

Display the following credit in your application where users can see it (`CREDIT` is
also exported by this crate):

> This service uses API functions from e-Stat, however its contents are not guaranteed by government.

## License

The wrapper is licensed under the [MIT License](LICENSE). Reference documents
in `e-stat/` are copied from e-Stat and retain their original source terms.
