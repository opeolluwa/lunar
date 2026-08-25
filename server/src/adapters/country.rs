use serde::{Deserialize, Serialize};

use lunar::entities::country;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchCountriesResponse {
    pub records: Vec<country::Model>,
}
