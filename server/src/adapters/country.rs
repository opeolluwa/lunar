use lunar::entities::country;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "country.ts")]
pub struct FetchCountriesResponse {
    pub records: Vec<country::Model>,
}
