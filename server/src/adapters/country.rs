use serde::{Deserialize, Serialize};
use ts_rs::TS;

use lunar::entities::country;

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "country.ts")]
pub struct FetchCountriesResponse {
    pub records: Vec<country::Model>,
}
