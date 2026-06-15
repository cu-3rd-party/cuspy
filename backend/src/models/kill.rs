use crate::models::resource::Resource;
use crate::models::user::{User, UserResponse};
use crate::models::{ApiError, db_optional_timestamp, db_optional_uuid, db_uuid};
use crate::rest::helpers;
use s3::Bucket;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgArgumentBuffer, PgValueRef};
use sqlx::{Decode, Encode, Executor, FromRow, PgConnection, Postgres, Row, Type, postgres::PgRow};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct ReportKillRequest {
    pub kill_event_id: Uuid,
    pub evidence_url: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct ConfirmKillRequest {
    pub kill_event_id: Uuid,
    pub confirmed: bool,
    pub note: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum KillEventStatus {
    Pending,
    Reported,
    Confirmed,
    AdminApproved,
    Rejected,
}

impl Type<Postgres> for KillEventStatus {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        <&str as Type<Postgres>>::type_info()
    }
}

impl<'r> Decode<'r, Postgres> for KillEventStatus {
    fn decode(
        value: PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        match value {
            "pending" => Ok(KillEventStatus::Pending),
            "reported" => Ok(KillEventStatus::Reported),
            "confirmed" => Ok(KillEventStatus::Confirmed),
            "admin_approved" => Ok(KillEventStatus::AdminApproved),
            "rejected" => Ok(KillEventStatus::Rejected),
            _ => Err(format!("Invalid value for KillEventStatus: {}", value).into()),
        }
    }
}

impl Encode<'_, Postgres> for KillEventStatus {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        let s = match self {
            KillEventStatus::Pending => "PENDING",
            KillEventStatus::Reported => "REPORTED",
            KillEventStatus::Confirmed => "CONFIRMED",
            KillEventStatus::AdminApproved => "ADMIN_APPROVED",
            KillEventStatus::Rejected => "REJECTED",
        };
        <&str as Encode<Postgres>>::encode(s, buf)
    }
}

#[derive(Serialize, ToSchema)]
pub struct KillEventResponse {
    pub killer: UserResponse,
    pub victim: UserResponse,
    pub status: KillEventStatus,
    pub evidence_url: Option<String>,
    pub killer_confirmed_at: Option<String>,
    pub victim_confirmed_at: Option<String>,
    pub moderation_reason: Option<String>,
    pub reported_at: String,
    pub confirmed_at: Option<String>,
    pub moderated_at: Option<String>,
    pub moderator_id: Option<Uuid>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct RankingEntry {
    pub rank: i64,
    pub user_id: Uuid,
    pub username: Option<String>,
    pub rating: i64,
    pub approved_kills: i64,
    pub approved_deaths: i64,
}

#[derive(Serialize, ToSchema)]
pub struct UserStatsResponse {
    pub user_id: Uuid,
    pub rating: i64,
    pub approved_kills: i64,
    pub approved_deaths: i64,
    pub pending_kills: i64,
}

impl<'r> FromRow<'r, PgRow> for RankingEntry {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        use crate::models::parse_uuid;

        Ok(Self {
            rank: row.get("rank"),
            user_id: parse_uuid(row, "user_id")?,
            username: row.try_get("username").ok(),
            rating: row.get("rating"),
            approved_kills: row.get("approved_kills"),
            approved_deaths: row.get("approved_deaths"),
        })
    }
}

impl<'r> FromRow<'r, PgRow> for UserStatsResponse {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        use crate::models::parse_uuid;

        Ok(Self {
            user_id: parse_uuid(row, "user_id")?,
            rating: row.get("rating"),
            approved_kills: row.get("approved_kills"),
            approved_deaths: row.get("approved_deaths"),
            pending_kills: row.get("pending_kills"),
        })
    }
}

pub struct KillEventRecord {
    pub kill_event_id: Uuid,
    pub killer_id: Uuid,
    pub victim_id: Uuid,
    pub status: KillEventStatus,
    pub evidence_resource_id: Option<Uuid>,
    pub killer_confirmed_at: Option<time::OffsetDateTime>,
    pub victim_confirmed_at: Option<time::OffsetDateTime>,
    pub confirmed_at: Option<time::OffsetDateTime>,
    pub moderated_at: Option<time::OffsetDateTime>,
    pub moderator_id: Option<Uuid>,
    pub moderation_reason: Option<String>,
    pub rating_applied_at: Option<time::OffsetDateTime>,
    pub created_at: time::OffsetDateTime,
    pub updated_at: Option<time::OffsetDateTime>,
}

impl KillEventRecord {
    pub async fn create<'c, E>(
        executor: E,
        killer_id: Uuid,
        victim_id: Uuid,
        reporter_is_killer: bool,
    ) -> Result<Self, ApiError>
    where
        E: Executor<'c, Database = Postgres>,
    {
        Ok(sqlx::query_as(
            r#"
                insert into kill_event (
                    killer_id,
                    victim_id,
                    status,
                    killer_confirmed_at,
                    victim_confirmed_at
                )
                values (
                    cast($1 as uuid),
                    cast($2 as uuid),
                    'REPORTED',
                    case when $4 then now() end,
                    case when $5 then null else now() end
                )
                returning *
            "#,
        )
        .bind(db_uuid(killer_id))
        .bind(db_uuid(victim_id))
        .bind(reporter_is_killer)
        .fetch_one(executor)
        .await?)
    }

    pub async fn get_by_id<'c, E>(executor: E, kill_event_id: Uuid) -> Result<Self, ApiError>
    where
        E: Executor<'c, Database = Postgres>,
    {
        sqlx::query_as::<_, Self>(
            r#"
                select *
                from kill_event
                where kill_event_id = cast($1 as uuid)
            "#,
        )
        .bind(db_uuid(kill_event_id))
        .fetch_optional(executor)
        .await?
        .ok_or(ApiError::NotFound)
    }

    pub async fn get_by_user_id<'c, E>(executor: E, user_id: Uuid) -> Result<Vec<Self>, ApiError>
    where
        E: Executor<'c, Database = Postgres>,
    {
        sqlx::query_as::<_, Self>(
            r#"
                select *
                from kill_event
                where cast(killer_id as text) = $1
                   or cast(victim_id as text) = $1
                order by created_at desc
            "#,
        )
        .bind(db_uuid(user_id))
        .fetch_all(executor)
        .await
        .map_err(ApiError::from)
    }

    pub async fn update<'c, E>(self, executor: E) -> Result<Self, ApiError>
    where
        E: Executor<'c, Database = Postgres>,
    {
        sqlx::query_as::<_, Self>(
            r#"
                update kill_event
                set
                    status = $2,
                    evidence_resource_id = cast($3 as uuid),
                    killer_confirmed_at = $4,
                    victim_confirmed_at = $5,
                    confirmed_at = $6,
                    moderated_at = $7,
                    moderator_id = cast($8 as uuid),
                    moderation_reason = $9,
                    rating_applied_at = $10
                where kill_event_id = cast($1 as uuid)
                returning *
            "#,
        )
        .bind(db_uuid(self.kill_event_id))
        .bind(&self.status)
        .bind(db_optional_uuid(self.evidence_resource_id))
        .bind(db_optional_timestamp(self.killer_confirmed_at))
        .bind(db_optional_timestamp(self.victim_confirmed_at))
        .bind(db_optional_timestamp(self.confirmed_at))
        .bind(db_optional_timestamp(self.moderated_at))
        .bind(db_optional_uuid(self.moderator_id))
        .bind(&self.moderation_reason)
        .bind(db_optional_timestamp(self.rating_applied_at))
        .fetch_one(executor)
        .await
        .map_err(ApiError::from)
    }

    pub async fn delete<'c, E>(executor: E, kill_event_id: Uuid) -> Result<bool, ApiError>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let result = sqlx::query(
            r#"
                delete from kill_event
                where kill_event_id = cast($1 as uuid)
            "#,
        )
        .bind(db_uuid(kill_event_id))
        .execute(executor)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}

impl<'r> FromRow<'r, PgRow> for KillEventRecord {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        Ok(Self {
            kill_event_id: row.get("kill_event_id"),
            killer_id: row.get("killer_id"),
            victim_id: row.get("victim_id"),
            status: row.get("status"),
            evidence_resource_id: row.try_get("evidence_resource_id")?,
            killer_confirmed_at: row.try_get("killer_confirmed_at")?,
            victim_confirmed_at: row.try_get("victim_confirmed_at")?,
            confirmed_at: row.try_get("confirmed_at")?,
            moderated_at: row.try_get("moderated_at")?,
            moderator_id: row.try_get("moderator_id")?,
            moderation_reason: row.try_get("moderation_reason")?,
            rating_applied_at: row.try_get("rating_applied_at")?,
            created_at: row.get("created_at"),
            updated_at: row.try_get("updated_at")?,
        })
    }
}

impl KillEventRecord {
    pub async fn into_response(
        self,
        executor: &mut PgConnection,
        bucket: Arc<Box<Bucket>>,
    ) -> Result<KillEventResponse, ApiError> {
        let killer = User::get_by_id(&mut *executor, self.killer_id)
            .await
            .ok_or(ApiError::NotFound)?;
        let victim = User::get_by_id(&mut *executor, self.killer_id)
            .await
            .ok_or(ApiError::NotFound)?;
        Ok(KillEventResponse {
            killer: killer.into_response(&mut *executor).await?,
            victim: victim.into_response(&mut *executor).await?,
            status: self.status,
            evidence_url: Resource::get_url_by_option_id(
                &mut *executor,
                bucket,
                self.evidence_resource_id,
            )
            .await,
            killer_confirmed_at: self.killer_confirmed_at.map(helpers::format_timestamp),
            victim_confirmed_at: self.victim_confirmed_at.map(helpers::format_timestamp),
            moderation_reason: self.moderation_reason,
            reported_at: helpers::format_timestamp(self.created_at),
            confirmed_at: self.confirmed_at.map(helpers::format_timestamp),
            moderated_at: self.moderated_at.map(helpers::format_timestamp),
            moderator_id: self.moderator_id,
            created_at: helpers::format_timestamp(self.created_at),
            updated_at: self.updated_at.map(helpers::format_timestamp),
        })
    }
}
