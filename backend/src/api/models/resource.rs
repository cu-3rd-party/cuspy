use crate::ApiContext;
use crate::api::models::{ApiError, parse_optional_timestamp, parse_timestamp, parse_uuid};
use base64::Engine;
use sha2::{Digest, Sha256};
use sqlx::any::AnyRow;
use sqlx::{Error, FromRow, Row};
use std::io::Cursor;
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
    pub async fn new(
        state: &ApiContext,
        content: bytes::Bytes,
        mime_type: Option<String>,
    ) -> Result<Self, ApiError> {
        let mut hasher = Sha256::new();
        hasher.update(content.as_ref());
        let checksum = format!("{:x}", hasher.finalize());

        let existing_resource = sqlx::query_as::<_, Resource>(
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
                "#,
        )
        .bind(&checksum)
        .fetch_optional(&state.db)
        .await?;

        if let Some(resource) = existing_resource {
            return Ok(resource);
        }

        let resource_id = Uuid::new_v4();
        let location =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(resource_id.as_bytes());
        let file_size = i64::try_from(content.len())
            .map_err(|_| ApiError::BadRequest("resource is too large".to_string()))?;
        let mut reader = Cursor::new(content.as_ref());

        if let Some(mime_type) = mime_type.as_deref() {
            state
                .bucket
                .put_object_stream_with_content_type(&mut reader, &location, mime_type)
                .await?;
        } else {
            state
                .bucket
                .put_object_stream(&mut reader, &location)
                .await?;
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
        .fetch_one(&state.db)
        .await?;

        Ok(resource)
    }
}

impl<'r> FromRow<'r, AnyRow> for Resource {
    fn from_row(row: &'r AnyRow) -> Result<Self, Error> {
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
