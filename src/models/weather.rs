use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Weather {
    pub humidity: f32,
    pub temperature: f32,
    pub created_at: Option<DateTime<Utc>>,
}
