use crate::models::{ApiError, parse_optional_timestamp, parse_timestamp, parse_uuid, db_uuid};
use base64::Engine;
use sha2::{Digest, Sha256};
use sqlx::{Error, Executor, FromRow, Postgres, Row, postgres::PgConnection};
use std::io::Cursor;
use std::sync::Arc;
use s3::Bucket;
use sqlx::postgres::PgRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct Resource {
    pub id: Uuid,
    pub file_location: String,
    pub file_size: i64,
    pub mime_type: Option<String>,
    pub checksum: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: time::OffsetDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: Option<time::OffsetDateTime>,
}

impl Resource {
    async fn create<'c, E>(
        executor: E,
        bucket: Arc<Box<Bucket>>,
        content: bytes::Bytes,
        checksum: String,
        mime_type: Option<String>,
    ) -> Result<Self, ApiError>
    where
        E: Executor<'c, Database = Postgres>
    {
        let resource_id = Uuid::new_v4();
        let location =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(resource_id.as_bytes());
        let file_size = i64::try_from(content.len())
            .map_err(|_| ApiError::BadRequest("resource is too large".to_string()))?;
        let mut reader = Cursor::new(content.as_ref());

        if let Some(mime_type) = mime_type.as_deref() {
            bucket
                .put_object_stream_with_content_type(&mut reader, &location, mime_type)
                .await?;
        } else {
            bucket.put_object_stream(&mut reader, &location).await?;
        }

        let resource: Resource = sqlx::query_as(
            r#"
                insert into "resource" (file_location, file_size, mime_type, checksum)
                values ($1, $2, $3, $4)
                returning
                    cast(resource_id as text) as resource_id,
                    file_location,
                    file_size,
                    mime_type,
                    checksum,
                    cast(created_at as text) as created_at,
                    cast(updated_at as text) as updated_at
            "#,
        )
            .bind(&location)
            .bind(file_size)
            .bind(&mime_type)
            .bind(&checksum)
            .fetch_one(executor)
            .await?;

        Ok(resource)
    }
    pub async fn new(
        executor: &mut PgConnection,
        bucket: Arc<Box<Bucket>>,
        content: bytes::Bytes,
        mime_type: Option<String>,
    ) -> Result<Self, ApiError> {
        let checksum = Self::calculate_checksum(&content);

        let existing_resource = Self::get_by_checksum(&mut *executor, &checksum).await;
        if let Some(resource) = existing_resource {
            return Ok(resource);
        }

        Ok(Self::create(&mut *executor, bucket, content, checksum, mime_type).await?)
    }

    pub async fn get_by_id<'c, E>(
        executor: E,
        resource_id: Uuid,
    ) -> Option<Resource>
    where
        E: Executor<'c, Database = Postgres>
    {
        sqlx::query_as(
            r#"
            select
                cast(resource_id as text) as resource_id,
                file_location,
                file_size,
                mime_type,
                checksum,
                cast(created_at as text) as created_at,
                cast(updated_at as text) as updated_at
            from "resource"
            where resource_id = cast($1 as uuid)
            limit 1
        "#,
        )
            .bind(db_uuid(resource_id))
            .fetch_optional(executor)
            .await
            .ok().flatten()
    }

    pub async fn get_by_checksum<'c, E>(
        executor: E,
        checksum: &String,
    ) -> Option<Resource>
    where
        E: Executor<'c, Database = Postgres>
    {
        sqlx::query_as(
            r#"
            select
                cast(resource_id as text) as resource_id,
                file_location,
                file_size,
                mime_type,
                checksum,
                cast(created_at as text) as created_at,
                cast(updated_at as text) as updated_at
            from "resource"
            where checksum = $1
            limit 1
        "#,
        )
            .bind(&checksum)
            .fetch_optional(executor)
            .await
            .ok().flatten()
    }

    pub fn calculate_checksum(
        content: &bytes::Bytes,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_ref());
        format!("{:x}", hasher.finalize())
    }

    const PRESIGN_EXPIRY_SECS: u32 = 1 * 60;
    pub async fn presign_get(
        &self,
        bucket: Arc<Box<Bucket>>,
    ) -> Result<String, ApiError> {
        Ok(bucket
            .presign_get(&self.file_location, Self::PRESIGN_EXPIRY_SECS, None)
            .await?)
    }
}

impl<'r> FromRow<'r, PgRow> for Resource {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: parse_uuid(row, "resource_id")?,
            file_location: row.get("file_location"),
            file_size: row.get("file_size"),
            mime_type: row.try_get("mime_type").ok(),
            checksum: row.try_get("checksum").ok(),
            created_at: parse_timestamp(row, "created_at")?,
            updated_at: parse_optional_timestamp(row, "updated_at").ok().flatten(),
        })
    }
}
