use crate::{Class, Client, Error, Metadata, Result, StatsDataRequest};
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CpiBase {
    Base2020,
    Base2025,
}

impl CpiBase {
    pub const fn table_id(self) -> &'static str {
        match self {
            Self::Base2020 => "0003427113",
            Self::Base2025 => "0004052037",
        }
    }
    pub const fn year(self) -> u16 {
        match self {
            Self::Base2020 => 2020,
            Self::Base2025 => 2025,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CpiIndicator {
    HeadlineCpi,
    CpiExFreshFood,
    CpiExFreshFoodAndEnergy,
}

impl CpiIndicator {
    pub const ALL: [Self; 3] = [
        Self::HeadlineCpi,
        Self::CpiExFreshFood,
        Self::CpiExFreshFoodAndEnergy,
    ];
    pub const fn slug(self) -> &'static str {
        match self {
            Self::HeadlineCpi => "headline_cpi",
            Self::CpiExFreshFood => "cpi_ex_fresh_food",
            Self::CpiExFreshFoodAndEnergy => "cpi_ex_fresh_food_and_energy",
        }
    }
    fn names(self) -> &'static [&'static str] {
        match self {
            Self::HeadlineCpi => &["総合", "All items"],
            Self::CpiExFreshFood => &["生鮮食品を除く総合", "All items, less fresh food"],
            Self::CpiExFreshFoodAndEnergy => &[
                "生鮮食品及びエネルギーを除く総合",
                "All items, less fresh food and energy",
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CpiMeasure {
    /// Unadjusted index with the selected base year's annual mean equal to 100.
    Index,
    /// Official percentage change from the same month in the previous year.
    YearOnYear,
}

impl CpiMeasure {
    fn names(self) -> &'static [&'static str] {
        match self {
            Self::Index => &["指数", "Index"],
            Self::YearOnYear => &[
                "前年同月比",
                "前年同月比（％）",
                "前年同月比(%)",
                "前年（同月）比",
                "前年(同月)比",
                "Change over the year",
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Month {
    year: u16,
    month: u8,
}

impl Month {
    pub fn new(year: u16, month: u8) -> Result<Self> {
        if !(1..=9999).contains(&year) || !(1..=12).contains(&month) {
            return Err(Error::Invalid(
                "month must have year 1..=9999 and month 1..=12".into(),
            ));
        }
        Ok(Self { year, month })
    }
    pub const fn year(self) -> u16 {
        self.year
    }
    pub const fn month(self) -> u8 {
        self.month
    }

    // Requests use lang=J. Class labels distinguish monthly from annual,
    // fiscal-year and quarterly observations without guessing time-code offsets.
    fn from_label(label: &str) -> Option<Self> {
        let label = label.trim();
        let (year, rest) = label.split_once('年')?;
        let month = rest.strip_suffix('月')?.trim();
        Self::new(year.trim().parse().ok()?, month.parse().ok()?).ok()
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}", self.year, self.month)
    }
}

#[derive(Debug, Clone)]
pub struct CpiQuery {
    pub base: CpiBase,
    pub measure: CpiMeasure,
    pub from: Option<Month>,
    pub through: Option<Month>,
}

impl CpiQuery {
    pub fn new(base: CpiBase, measure: CpiMeasure) -> Self {
        Self {
            base,
            measure,
            from: None,
            through: None,
        }
    }
    pub fn between(mut self, from: Month, through: Month) -> Self {
        self.from = Some(from);
        self.through = Some(through);
        self
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CpiObservation {
    pub month: Month,
    pub time_code: String,
    pub time_label: String,
    /// None for suppressed/missing/non-numeric cells; inspect raw_value.
    pub value: Option<f64>,
    pub raw_value: String,
    pub annotation: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CpiSeries {
    pub indicator: CpiIndicator,
    pub measure: CpiMeasure,
    pub base_year: u16,
    pub table_id: String,
    pub item_dimension: String,
    pub item_code: String,
    pub item_name: String,
    pub area_dimension: String,
    pub area_code: String,
    pub area_name: String,
    pub measure_code: String,
    pub unit: String,
    /// Ascending by month, with no duplicate periods.
    pub observations: Vec<CpiObservation>,
}

impl Client {
    pub async fn headline_cpi(&self, query: &CpiQuery) -> Result<CpiSeries> {
        self.cpi(CpiIndicator::HeadlineCpi, query).await
    }
    pub async fn cpi_ex_fresh_food(&self, query: &CpiQuery) -> Result<CpiSeries> {
        self.cpi(CpiIndicator::CpiExFreshFood, query).await
    }
    pub async fn cpi_ex_fresh_food_and_energy(&self, query: &CpiQuery) -> Result<CpiSeries> {
        self.cpi(CpiIndicator::CpiExFreshFoodAndEnergy, query).await
    }

    /// Fetch all three series concurrently through the same rustopus arm.
    pub async fn cpi_indicators(&self, query: &CpiQuery) -> Result<[CpiSeries; 3]> {
        let (headline, core, core_core) = tokio::try_join!(
            self.headline_cpi(query),
            self.cpi_ex_fresh_food(query),
            self.cpi_ex_fresh_food_and_energy(query)
        )?;
        Ok([headline, core, core_core])
    }

    pub async fn cpi(&self, indicator: CpiIndicator, query: &CpiQuery) -> Result<CpiSeries> {
        if matches!((query.from, query.through), (Some(from), Some(to)) if from > to) {
            return Err(Error::Invalid("CPI range starts after it ends".into()));
        }
        let table_id = query.base.table_id();
        let meta = self.metadata(table_id).await?;
        // Both bases use 0001 for headline. The goods/services classification
        // also contains "総合" (0201), at the same level; names/levels cannot
        // distinguish them. Still verify the canonical item's metadata label.
        let (item_dimension, item) = resolve(&meta, indicator.names(), |id, class| {
            id.starts_with("cat")
                && (indicator != CpiIndicator::HeadlineCpi || class.code == "0001")
        })?;
        let (area_dimension, area) = resolve(&meta, &["全国", "All Japan"], |id, _| {
            id == "area" || id.starts_with("cat")
        })?;
        let (_, measure) = resolve(&meta, query.measure.names(), |id, _| id == "tab")?;
        if item_dimension == area_dimension {
            return Err(Error::Response("CPI item and region share an axis".into()));
        }
        let time = meta
            .dimensions
            .iter()
            .find(|d| d.id == "time")
            .ok_or_else(|| Error::Response("CPI table has no time dimension".into()))?;
        let mut months = BTreeMap::new();
        let mut unique_months = BTreeSet::new();
        let mut has_monthly_labels = false;
        for class in &time.classes {
            if let Some(month) = Month::from_label(&class.name) {
                has_monthly_labels = true;
                if query.from.is_some_and(|from| month < from)
                    || query.through.is_some_and(|to| month > to)
                {
                    continue;
                }
                if !unique_months.insert(month)
                    || months
                        .insert(class.code.clone(), (month, class.name.clone()))
                        .is_some()
                {
                    return Err(Error::Response("ambiguous monthly CPI time codes".into()));
                }
            }
        }
        if !has_monthly_labels {
            return Err(Error::Response(
                "CPI time metadata contains no recognized monthly labels".into(),
            ));
        }
        let mut request = StatsDataRequest::new(table_id)
            .filter(code_filter(item_dimension)?, &item.code)
            .filter(code_filter(area_dimension)?, &area.code)
            .filter("cdTab", &measure.code);
        // Never silently mix households or another unselected dimension.
        for dimension in &meta.dimensions {
            if [item_dimension, area_dimension, "tab", "time"].contains(&dimension.id.as_str()) {
                continue;
            }
            if dimension.classes.len() != 1 {
                return Err(Error::Response(format!(
                    "CPI table has unselected axis {}",
                    dimension.id
                )));
            }
            request = request.filter(code_filter(&dimension.id)?, &dimension.classes[0].code);
        }
        let codes: Vec<_> = months.keys().cloned().collect();
        let mut observations = BTreeMap::new();
        // e-Stat accepts at most 100 exact codes per dimension per request.
        for chunk in codes.chunks(100) {
            let request = request.clone().filter("cdTime", chunk.join(","));
            let allowed: BTreeSet<_> = chunk.iter().map(String::as_str).collect();
            for row in self.stats_data_all(&request).await? {
                for (filter, expected) in &request.filters {
                    if filter == "cdTime" {
                        continue;
                    }
                    let id = filter.strip_prefix("cd").unwrap().to_ascii_lowercase();
                    if row.dimension(&id) != Some(expected.as_str()) {
                        return Err(Error::Response(format!(
                            "CPI response does not match selected {id}"
                        )));
                    }
                }
                let code = row
                    .dimension("time")
                    .ok_or_else(|| Error::Response("CPI row has no time code".into()))?;
                if !allowed.contains(code) {
                    return Err(Error::Response(
                        "CPI response contains an unrequested time code".into(),
                    ));
                }
                let (month, label) = &months[code];
                let observation = CpiObservation {
                    month: *month,
                    time_code: code.into(),
                    time_label: label.clone(),
                    value: row.numeric_value(),
                    raw_value: row.raw_value,
                    annotation: row.annotation,
                };
                if observations.insert(*month, observation).is_some() {
                    return Err(Error::Response("duplicate monthly CPI observation".into()));
                }
            }
        }
        Ok(CpiSeries {
            indicator,
            measure: query.measure,
            base_year: query.base.year(),
            table_id: table_id.into(),
            item_dimension: item_dimension.into(),
            item_code: item.code.clone(),
            item_name: item.name.clone(),
            area_dimension: area_dimension.into(),
            area_code: area.code.clone(),
            area_name: area.name.clone(),
            measure_code: measure.code.clone(),
            unit: match query.measure {
                CpiMeasure::Index => format!("{}=100", query.base.year()),
                CpiMeasure::YearOnYear => "%".into(),
            },
            observations: observations.into_values().collect(),
        })
    }
}

fn resolve<'a>(
    meta: &'a Metadata,
    names: &[&str],
    eligible: impl Fn(&str, &Class) -> bool,
) -> Result<(&'a str, &'a Class)> {
    let mut matches = meta
        .dimensions
        .iter()
        .flat_map(|d| d.classes.iter().map(move |class| (d.id.as_str(), class)))
        .filter(|(id, class)| eligible(id, class))
        .filter(|(_, class)| {
            // Some CPI class labels include their numeric code as a prefix.
            let label = class
                .name
                .trim()
                .trim_start_matches(|c: char| c.is_ascii_digit())
                .trim();
            names.contains(&label)
        });
    let found = matches
        .next()
        .ok_or_else(|| Error::Response(format!("CPI metadata missing {}", names[0])))?;
    if let Some(other) = matches.next() {
        return Err(Error::Response(format!(
            "ambiguous CPI category {}: {}:{} (level {:?}), {}:{} (level {:?})",
            names[0], found.0, found.1.code, found.1.level, other.0, other.1.code, other.1.level
        )));
    }
    Ok(found)
}

fn code_filter(id: &str) -> Result<String> {
    match id {
        "tab" => Ok("cdTab".into()),
        "area" => Ok("cdArea".into()),
        "time" => Ok("cdTime".into()),
        id if (1..=15).any(|i| id == format!("cat{i:02}")) => Ok(format!("cdC{}", &id[1..])),
        _ => Err(Error::Response(format!("unsupported CPI dimension {id}"))),
    }
}
