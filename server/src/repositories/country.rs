use std::sync::Arc;

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use lunar::entities::{country, country::Column, prelude::Country};
use crate::errors::database_error::DatabaseError;
use crate::repositories::base::Repository;


#[derive(Clone)]
pub struct CountryRepository {
    db_conn: Arc<DatabaseConnection>,
}

pub(crate) trait CountryRepositoryExt {
    async fn fetch_all_countries(&self) -> Result<Vec<country::Model>, DatabaseError>;

    async fn fetch_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<country::Model>, DatabaseError>;

    async fn fetch_countries_by_currency_code(
        &self,
        currency_code: &str,
    ) -> Result<Vec<country::Model>, DatabaseError>;
}

impl Repository for CountryRepository {
    fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            db_conn: db_conn.to_owned(),
        }
    }
}

impl CountryRepositoryExt for CountryRepository {
    async fn fetch_all_countries(&self) -> Result<Vec<country::Model>, DatabaseError> {
        let countries: Vec<country::Model> = Country::find()
            .all(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;

        Ok(countries)
    }

    async fn fetch_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<country::Model>, DatabaseError> {
        let country = Country::find_by_id(identifier)
            .one(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;

        Ok(country)
    }

    async fn fetch_countries_by_currency_code(
        &self,
        currency_code: &str,
    ) -> Result<Vec<country::Model>, DatabaseError> {
        let countries = Country::find()
            .filter(Column::CurrencyCode.eq(currency_code))
            .all(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;

        Ok(countries)
    }
}
