//! Requires a personal e-Stat application ID. Never runs in the default suite.
use e_stat_wrapper::{Client, CpiBase, CpiIndicator, CpiMeasure, CpiQuery, Month};

async fn verify(indicator: CpiIndicator) {
    let app_id =
        std::env::var("ESTAT_APP_ID").expect("set ESTAT_APP_ID before running ignored live tests");
    let client = Client::new(app_id).unwrap();
    let query = CpiQuery::new(CpiBase::Base2025, CpiMeasure::Index)
        .between(Month::new(2025, 1).unwrap(), Month::new(2025, 12).unwrap());
    let series = client
        .cpi(indicator, &query)
        .await
        .expect("fetch national monthly CPI");
    assert_eq!(series.indicator, indicator);
    assert_eq!(series.table_id, "0004052037");
    assert_eq!(series.base_year, 2025);
    assert!(series.area_name.trim().ends_with("全国"));
    assert_eq!(
        series.observations.len(),
        12,
        "expected every month in the base year"
    );
    for (i, row) in series.observations.iter().enumerate() {
        assert_eq!(row.month, Month::new(2025, i as u8 + 1).unwrap());
        let value = row.value.expect("base-year CPI should be numeric");
        assert!(value > 0.0 && value.is_finite());
    }
    // Independent normalization invariant: each base-year annual mean is 100.
    // Monthly source values are rounded to one decimal, so allow 0.1 points.
    let mean = series
        .observations
        .iter()
        .map(|o| o.value.unwrap())
        .sum::<f64>()
        / 12.0;
    assert!((mean - 100.0).abs() <= 0.1, "base-year mean was {mean}");
    println!(
        "{}: {} monthly values, base-year mean={mean:.3}, item={}",
        indicator.slug(),
        series.observations.len(),
        series.item_code
    );
}

#[tokio::test]
#[ignore = "requires ESTAT_APP_ID and network access"]
async fn headline_cpi() {
    verify(CpiIndicator::HeadlineCpi).await;
}

#[tokio::test]
#[ignore = "requires ESTAT_APP_ID and network access"]
async fn cpi_ex_fresh_food() {
    verify(CpiIndicator::CpiExFreshFood).await;
}

#[tokio::test]
#[ignore = "requires ESTAT_APP_ID and network access"]
async fn cpi_ex_fresh_food_and_energy() {
    verify(CpiIndicator::CpiExFreshFoodAndEnergy).await;
}

#[tokio::test]
#[ignore = "requires ESTAT_APP_ID and network access"]
async fn all_three_published_year_on_year_series() {
    let client = Client::new(std::env::var("ESTAT_APP_ID").expect("set ESTAT_APP_ID")).unwrap();
    let query = CpiQuery::new(CpiBase::Base2025, CpiMeasure::YearOnYear)
        .between(Month::new(2026, 1).unwrap(), Month::new(2026, 6).unwrap());
    let series = client.cpi_indicators(&query).await.unwrap();
    for (i, series) in series.iter().enumerate() {
        assert_eq!(series.indicator, CpiIndicator::ALL[i]);
        assert_eq!(series.measure, CpiMeasure::YearOnYear);
        assert_eq!(series.unit, "%");
        assert_eq!(series.observations.len(), 6);
        for (i, row) in series.observations.iter().enumerate() {
            assert_eq!(row.month, Month::new(2026, i as u8 + 1).unwrap());
            assert!(
                row.value
                    .expect("published YoY should be numeric")
                    .is_finite()
            );
        }
    }
}
