//! Async e-Stat API v3 client, with shared request orchestration through rustopus.
//! Clone one client across Winter tasks to share concurrency and rate limits.
//! CPI methods return national, unadjusted monthly series; choose index levels
//! or the officially published year-on-year percentage explicitly.

mod client;
mod cpi;
mod models;
mod policy;

pub use client::{Client, ClientBuilder, Error, Result, StatsDataRequest, TableSearch};
pub use cpi::{CpiBase, CpiIndicator, CpiMeasure, CpiObservation, CpiQuery, CpiSeries, Month};
pub use models::{Class, Dimension, Metadata, Observation, StatsPage, Table, TablePage};
pub use policy::{EstatPolicy, PolicyError};
pub use rustopus;

pub const API_BASE_URL: &str = "https://api.e-stat.go.jp/rest/3.0/app/json/";
pub const CREDIT: &str = "This service uses API functions from e-Stat, however its contents are not guaranteed by government.";
