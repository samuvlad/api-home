use async_trait::async_trait;
use chrono::Utc;
use std::sync::PoisonError;
use thiserror::Error;

use crate::models::weather::Weather;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("PoisonError: `{0}`")]
    LockError(String),
    #[error("This entity already exists")]
    AlreadyExists,
    #[error("This entity does not exist")]
    DoesNotExist,
    #[error("The id format is not valid")]
    InvalidId,
}

impl<T> From<PoisonError<T>> for RepositoryError {
    fn from(poison_error: PoisonError<T>) -> Self {
        RepositoryError::LockError(poison_error.to_string())
    }
}

#[async_trait]
pub trait Repository: Send + Sync + 'static {
    async fn create_register_weather(&self, weather: &Weather) -> Result<Weather, RepositoryError>;
    async fn get(&self) -> Result<Vec<Weather>, RepositoryError>;
}

pub struct BdRepository {
    pool: sqlx::PgPool,
}

impl BdRepository {
    pub async fn from_env() -> sqlx::Result<Self> {
        let conn_str =
            std::env::var("DATABASE_URL").map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;
        let pool = sqlx::PgPool::connect(&conn_str).await?;
        Ok(Self { pool })
    }
}
#[async_trait]
impl Repository for BdRepository {
    async fn create_register_weather(&self, weather: &Weather) -> Result<Weather, RepositoryError> {
        let result = sqlx::query_as::<_, Weather>(
            r#"
            INSERT INTO weather (humidity, temperature, created_at)
            VALUES ($1, $2, $3)
            RETURNING humidity, temperature, created_at"#,
        )
        .bind(&weather.humidity)
        .bind(&weather.temperature)
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await;

        result.map_err(|e| {
            println!("{}", e);
            RepositoryError::AlreadyExists
        })
    }

    async fn get(&self) -> Result<Vec<Weather>, RepositoryError> {
        let result = sqlx::query_as::<_, Weather>(
            r#"
            SELECT * FROM weather;
            "#,
        )
        .fetch_all(&self.pool)
        .await;

        result.map_err(|e| {
            print!("{}", e);
            RepositoryError::DoesNotExist
        })
    }
}
