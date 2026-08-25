use std::sync::Arc;

use axum::extract::{Path, State};

use crate::{
    adapters::country::FetchCountriesResponse,
    errors::service_error::ServiceError,
    response::ApiResponse,
    services::country_service::CountryServiceExt,
    states::AppState,
};

use lunar::entities::country;

pub async fn fetch_all_countries(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<FetchCountriesResponse>, ServiceError> {
    let countries = state.services.country_service.get_all_countries().await?;

    Ok(ApiResponse::builder()
        .message("Countries fetched successfully")
        .data(countries)
        .build())
}

pub async fn fetch_country_by_identifier(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<Option<country::Model>>, ServiceError> {
    let country = state
        .services
        .country_service
        .get_country_by_identifier(&identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("Country fetched successfully")
        .data(country)
        .build())
}

pub async fn fetch_countries_by_currency_code(
    State(state): State<Arc<AppState>>,
    Path(currency_code): Path<String>,
) -> Result<ApiResponse<FetchCountriesResponse>, ServiceError> {
    let countries = state
        .services
        .country_service
        .get_countries_by_currency_code(&currency_code)
        .await?;

    Ok(ApiResponse::builder()
        .message("Countries fetched successfully")
        .data(countries)
        .build())
}
