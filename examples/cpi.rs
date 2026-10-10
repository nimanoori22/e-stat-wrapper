use e_stat_wrapper::{Client, CpiBase, CpiMeasure, CpiQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(std::env::var("ESTAT_APP_ID")?)?;
    let query = CpiQuery::new(CpiBase::Base2025, CpiMeasure::YearOnYear);
    for series in client.cpi_indicators(&query).await? {
        match series.observations.last() {
            Some(latest) => println!(
                "{} {}: {:?} {} ({} base)",
                series.indicator.slug(),
                latest.month,
                latest.value,
                series.unit,
                series.base_year
            ),
            None => println!("{}: no monthly observations", series.indicator.slug()),
        }
    }
    Ok(())
}
