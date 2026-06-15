use crate::models::{ApiError, db_uuid, parse_timestamp, parse_uuid};
use crate::rest::helpers::format_timestamp;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow, Postgres, Row, postgres::PgRow};
use utoipa::ToSchema;
use uuid::Uuid;

pub struct RatingHistoryRecord {
    pub rating_history_id: Uuid,
    pub user_id: Uuid,
    pub rating: i64,
    pub change: i64,
    pub reason: Option<String>,
    pub created_at: time::OffsetDateTime,
}

#[derive(Serialize, ToSchema)]
pub struct RatingHistoryResponse {
    pub rating_history_id: Uuid,
    pub user_id: Uuid,
    pub rating: i64,
    pub change: i64,
    pub reason: Option<String>,
    pub created_at: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateRatingChangeRequest {
    pub user_id: Uuid,
    pub change: i64,
    pub reason: Option<String>,
}

impl RatingHistoryRecord {
    pub async fn create<'c, E>(
        executor: E,
        user_id: Uuid,
        change: i64,
        reason: Option<String>,
    ) -> Result<Self, ApiError>
    where
        E: Executor<'c, Database = Postgres>,
    {
        Ok(sqlx::query_as(
            r#"
                insert into rating_history (user_id, rating, change, reason)
                select cast($1 as uuid), rating + $2, $2, $3
                from "user"
                where user_id = cast($1 as uuid)
                returning
                    cast(rating_history_id as text) as rating_history_id,
                    cast(user_id as text) as user_id,
                    rating,
                    change,
                    reason,
                    cast(created_at as text) as created_at
            "#,
        )
        .bind(db_uuid(user_id))
        .bind(change)
        .bind(reason)
        .fetch_one(executor)
        .await?)
    }

    pub async fn get_by_id<'c, E>(executor: E, rating_history_id: Uuid) -> Option<Self>
    where
        E: Executor<'c, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
                select
                    cast(rating_history_id as text) as rating_history_id,
                    cast(user_id as text) as user_id,
                    rating,
                    change,
                    reason,
                    cast(created_at as text) as created_at
                from rating_history
                where rating_history_id = cast($1 as uuid)
                limit 1
            "#,
        )
        .bind(db_uuid(rating_history_id))
        .fetch_optional(executor)
        .await
        .ok()
        .flatten()
    }

    pub async fn get_by_user_id<'c, E>(executor: E, user_id: Uuid) -> Result<Vec<Self>, ApiError>
    where
        E: Executor<'c, Database = Postgres>,
    {
        Ok(sqlx::query_as(
            r#"
                select
                    cast(rating_history_id as text) as rating_history_id,
                    cast(user_id as text) as user_id,
                    rating,
                    change,
                    reason,
                    cast(created_at as text) as created_at
                from rating_history
                where user_id = cast($1 as uuid)
                order by created_at desc
            "#,
        )
        .bind(db_uuid(user_id))
        .fetch_all(executor)
        .await?)
    }

    pub async fn get_all<'c, E>(executor: E) -> Result<Vec<Self>, ApiError>
    where
        E: Executor<'c, Database = Postgres>,
    {
        Ok(sqlx::query_as(
            r#"
                select
                    cast(rating_history_id as text) as rating_history_id,
                    cast(user_id as text) as user_id,
                    rating,
                    change,
                    reason,
                    cast(created_at as text) as created_at
                from rating_history
                order by created_at desc
            "#,
        )
        .fetch_all(executor)
        .await?)
    }

    pub async fn into_response(self) -> RatingHistoryResponse {
        RatingHistoryResponse {
            rating_history_id: self.rating_history_id,
            user_id: self.user_id,
            rating: self.rating,
            change: self.change,
            reason: self.reason,
            created_at: format_timestamp(self.created_at),
        }
    }
}

impl<'r> FromRow<'r, PgRow> for RatingHistoryRecord {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            rating_history_id: parse_uuid(row, "rating_history_id")?,
            user_id: parse_uuid(row, "user_id")?,
            rating: row.get("rating"),
            change: row.get("change"),
            reason: row.try_get("reason").ok(),
            created_at: parse_timestamp(row, "created_at")?,
        })
    }
}
