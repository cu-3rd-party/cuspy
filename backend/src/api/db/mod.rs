use crate::api::models::{ApiError, db_uuid};
use serde_json::{Map, Value};
use sqlx::AnyPool;
use uuid::Uuid;

fn profile_data_object(payload: &Value) -> Result<&Map<String, Value>, ApiError> {
    payload.as_object().ok_or(ApiError::BadRequest(
        "profile data must be a JSON object".into(),
    ))
}

fn optional_string(map: &Map<String, Value>, key: &str) -> Option<String> {
    map.get(key)?.as_str().map(str::to_owned)
}

fn optional_bool(map: &Map<String, Value>, key: &str) -> Option<bool> {
    map.get(key)?.as_bool()
}

fn optional_i64(map: &Map<String, Value>, key: &str) -> Option<i64> {
    map.get(key)?.as_i64()
}

#[allow(dead_code)]
pub async fn insert_agent_data_from_profile(
    db: &AnyPool,
    payload: &Value,
) -> Result<Uuid, ApiError> {
    let payload = profile_data_object(payload)?;

    let agent_data_id = sqlx::query_scalar::<_, String>(
        r#"
        insert into "agent_data" (
            agent_data_id,
            codename,
            academic_group,
            academic_level,
            course_number,
            bachelor_track,
            identification_name,
            physical_contact_allowed,
            hugs_close_proximity_allowed
        )
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        returning cast(agent_data_id as text)
        "#,
    )
    .bind(db_uuid(Uuid::now_v7()))
    .bind(optional_string(payload, "codename"))
    .bind(optional_string(payload, "academic_group"))
    .bind(optional_string(payload, "academic_level"))
    .bind(optional_i64(payload, "course_number"))
    .bind(optional_string(payload, "bachelor_track"))
    .bind(optional_string(payload, "identification_name"))
    .bind(optional_bool(payload, "physical_contact_allowed").unwrap_or(false))
    .bind(optional_bool(payload, "hugs_close_proximity_allowed").unwrap_or(false))
    .fetch_one(db)
    .await?;

    Uuid::parse_str(&agent_data_id)
        .map_err(|error| ApiError::Internal(format!("invalid agent_data_id returned: {error}")))
}

pub async fn update_agent_data_from_profile(
    db: &AnyPool,
    agent_data_id: Uuid,
    payload: &Value,
) -> Result<(), ApiError> {
    let payload = profile_data_object(payload)?;

    let result = sqlx::query(
        r#"
        update "agent_data"
        set
            codename = coalesce($2, codename),
            academic_group = coalesce($3, academic_group),
            academic_level = coalesce($4, academic_level),
            course_number = coalesce($5, course_number),
            bachelor_track = coalesce($6, bachelor_track),
            identification_name = coalesce($7, identification_name),
            physical_contact_allowed = coalesce($8, physical_contact_allowed),
            hugs_close_proximity_allowed = coalesce($9, hugs_close_proximity_allowed)
        where agent_data_id = cast($1 as uuid)
        "#,
    )
    .bind(db_uuid(agent_data_id))
    .bind(optional_string(payload, "codename"))
    .bind(optional_string(payload, "academic_group"))
    .bind(optional_string(payload, "academic_level"))
    .bind(optional_i64(payload, "course_number"))
    .bind(optional_string(payload, "bachelor_track"))
    .bind(optional_string(payload, "identification_name"))
    .bind(optional_bool(payload, "physical_contact_allowed"))
    .bind(optional_bool(payload, "hugs_close_proximity_allowed"))
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(())
}
