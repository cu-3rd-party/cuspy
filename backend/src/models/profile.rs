use crate::models::ApiError;
use crate::models::agent_data::{AgentData};
use crate::models::{
    db_optional_timestamp, db_uuid, parse_optional_timestamp, parse_timestamp, parse_uuid,
};
use crate::rest::helpers::format_timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Executor, FromRow, PgConnection, Postgres, Row, postgres::PgRow};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct ProfileRequestEvent {
    pub profile_request_id: Uuid,
    pub user_id: Uuid,
    pub status: String,
    pub reviewer_note: String,
    pub created_at: String,
    pub updated_at: String,
}

pub struct ProfileRequestRecord {
    pub profile_request_id: Uuid,
    pub user_id: Uuid,
    pub requested_profile_data_id: Uuid,
    pub status: String,
    pub reviewer_note: Option<String>,
    pub reviewed_at: Option<time::OffsetDateTime>,
    pub created_at: time::OffsetDateTime,
    pub updated_at: Option<time::OffsetDateTime>,
}

impl ProfileRequestRecord {
    pub async fn create<'c, E>(
        executor: E,
        user_id: Uuid,
        requested_profile_data_id: Uuid,
        status: String,
    ) -> Result<Self, ApiError>
    where
        E: Executor<'c, Database = Postgres>,
    {
        Ok(sqlx::query_as(
            r#"
                insert into profile_request (
                    user_id,
                    requested_profile_data_id,
                    status
                )
                values (cast($2 as uuid), cast($3 as uuid), $4)
                returning *
            "#,
        )
        .bind(db_uuid(user_id))
        .bind(db_uuid(requested_profile_data_id))
        .bind(status)
        .fetch_one(executor)
        .await?)
    }

    pub async fn get_by_id<'c, E>(executor: E, profile_request_id: Uuid) -> Option<Self>
    where
        E: Executor<'c, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
                select *
                from profile_request
                where profile_request_id = cast($1 as uuid)
                limit 1
            "#,
        )
        .bind(db_uuid(profile_request_id))
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
                select *
                from profile_request
                where cast(user_id as text) = $1
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
                select *
                from profile_request
                order by created_at desc
            "#,
        )
        .fetch_all(executor)
        .await?)
    }

    pub async fn update_timestamp<'c, E>(self, executor: E) -> Result<Self, ApiError>
    where
        E: Executor<'c, Database = Postgres>,
    {
        Ok(sqlx::query_as(
            r#"
                update profile_request
                set updated_at = now()
                where profile_request_id = cast($1 as uuid)
                returning *
            "#,
        )
        .bind(db_uuid(self.profile_request_id))
        .fetch_one(executor)
        .await?)
    }

    pub async fn update<'c, E>(
        self,
        executor: E,
        status: Option<String>,
        reviewer_note: Option<String>,
    ) -> Result<Self, ApiError>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let reviewed_at = Some(time::OffsetDateTime::now_utc());
        Ok(sqlx::query_as(
            r#"
                update profile_request
                set
                    status = coalesce($2, status),
                    reviewer_note = coalesce($3, reviewer_note),
                    reviewed_at = coalesce(cast($4 as timestamptz), reviewed_at)
                where profile_request_id = cast($1 as uuid)
                returning *
            "#,
        )
        .bind(db_uuid(self.profile_request_id))
        .bind(status)
        .bind(reviewer_note)
        .bind(db_optional_timestamp(reviewed_at))
        .fetch_one(executor)
        .await?)
    }

    pub async fn delete<'c, E>(self, executor: E) -> Result<bool, ApiError>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let result = sqlx::query(
            r#"
                delete from profile_request
                where profile_request_id = cast($1 as uuid)
            "#,
        )
        .bind(db_uuid(self.profile_request_id))
        .execute(executor)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_requested_profile_data_id<'c, E>(&self, executor: E) -> Result<Uuid, ApiError>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let value: String = sqlx::query_scalar(
            r#"
                select cast(requested_profile_data_id as text)
                from profile_request
                where profile_request_id = cast($1 as uuid)
            "#,
        )
        .bind(db_uuid(self.profile_request_id))
        .fetch_optional(executor)
        .await?
        .ok_or(ApiError::NotFound)?;

        Uuid::parse_str(&value).map_err(|error| {
            ApiError::Internal(format!("invalid requested profile data id: {error}"))
        })
    }
}

impl ProfileRequestRecord {
    pub async fn into_response(
        self,
        executor: &mut PgConnection,
    ) -> Result<ProfileRequestResponse, ApiError> {
        Ok(ProfileRequestResponse {
            profile_request_id: self.profile_request_id,
            user_id: self.user_id,
            requested_profile_data: AgentData::get_by_id(
                &mut *executor,
                self.requested_profile_data_id,
            )
            .await,
            status: self.status,
            reviewer_note: self.reviewer_note,
            reviewed_at: self.reviewed_at.map(format_timestamp),
            created_at: format_timestamp(self.created_at),
            updated_at: self.updated_at.map(format_timestamp),
        })
    }
}

impl<'r> FromRow<'r, PgRow> for ProfileRequestRecord {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            profile_request_id: parse_uuid(row, "profile_request_id")?,
            user_id: parse_uuid(row, "user_id")?,
            requested_profile_data_id: parse_uuid(row, "requested_profile_data_id")?,
            status: row.get("status"),
            reviewer_note: row.try_get("reviewer_note").ok(),
            reviewed_at: parse_optional_timestamp(row, "reviewed_at").ok().flatten(),
            created_at: parse_timestamp(row, "created_at")?,
            updated_at: parse_optional_timestamp(row, "updated_at")?,
        })
    }
}

/// блять че за нейминг говна сука
#[derive(Serialize, ToSchema)]
pub struct ProfileRequestResponse {
    pub profile_request_id: Uuid,
    pub user_id: Uuid,
    pub requested_profile_data: Option<AgentData>,
    pub status: String,
    pub reviewer_note: Option<String>,
    pub reviewed_at: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateProfileRequest {
    pub agent_data_id: Uuid,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateProfileRequest {
    pub status: Option<String>,
    pub reviewer_note: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct AdminUpdateProfileRequest {
    pub requested_profile_data: Option<Value>,
    pub status: Option<String>,
    pub reviewer_note: Option<String>,
}
