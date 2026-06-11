use crate::api::extractor::AdminUser;
use crate::api::models::profile::{
    AdminUpdateProfileRequest, ProfileRequestRecord, ProfileRequestResponse,
};
use crate::api::models::{ApiError, db_optional_timestamp, db_uuid};
use crate::api::{extractor, helpers};
use crate::{ApiContext, notifier};
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use http::{HeaderMap, StatusCode};
use uuid::Uuid;

pub fn profile_request_router() -> Router<ApiContext> {
    Router::new()
        .route("/", get(admin_list_profile_requests))
        .route(
            "/{request_id}",
            get(admin_get_profile_request)
                .patch(admin_update_profile_request)
                .delete(admin_delete_profile_request),
        )
}

#[utoipa::path(
    get,
    path = "/admin/profile-requests",
    tag = "admin",
    responses(
        (status = 200, description = "List all profile requests", body = [ProfileRequestResponse]),
        (status = 401, description = "Unauthorized", body = crate::api::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_list_profile_requests(
    State(state): State<ApiContext>,
    AdminUser(_user): AdminUser,
) -> Result<Json<Vec<ProfileRequestResponse>>, ApiError> {
    let requests = sqlx::query_as::<_, ProfileRequestRecord>(
        r#"
        select
            profile_request_id,
            user_id,
            requested_profile_data_id,
            status,
            reviewer_note,
            reviewed_at,
            created_at,
            updated_at
        from profile_request
        order by created_at desc
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(
        requests
            .into_iter()
            .map(helpers::to_profile_request_response)
            .collect(),
    ))
}

#[utoipa::path(
    get,
    path = "/admin/profile-requests/{request_id}",
    tag = "admin",
    params(("request_id" = Uuid, Path, description = "Profile request id")),
    responses(
        (status = 200, description = "Profile request details", body = ProfileRequestResponse),
        (status = 404, description = "Profile request not found", body = crate::api::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_get_profile_request(
    State(state): State<ApiContext>,
    AdminUser(_user): AdminUser,
    Path(request_id): Path<Uuid>,
) -> Result<Json<ProfileRequestResponse>, ApiError> {
    let request = sqlx::query_as::<_, ProfileRequestRecord>(
        r#"
        select
            profile_request_id,
            user_id,
            requested_profile_data_id,
            status,
            reviewer_note,
            reviewed_at,
            created_at,
            updated_at
        from profile_request
        where profile_request_id = cast($1 as uuid)
        "#,
    )
    .bind(db_uuid(request_id))
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(helpers::to_profile_request_response(request)))
}

#[utoipa::path(
    patch,
    path = "/admin/profile-requests/{request_id}",
    tag = "admin",
    params(("request_id" = Uuid, Path, description = "Profile request id")),
    request_body = AdminUpdateProfileRequest,
    responses(
        (status = 200, description = "Profile request updated", body = ProfileRequestResponse),
        (status = 400, description = "Bad request", body = crate::api::models::ErrorResponse),
        (status = 404, description = "Profile request not found", body = crate::api::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_update_profile_request(
    State(state): State<ApiContext>,
    AdminUser(_user): AdminUser,
    Path(request_id): Path<Uuid>,
    Json(payload): Json<AdminUpdateProfileRequest>,
) -> Result<Json<ProfileRequestResponse>, ApiError> {
    if let Some(status) = payload.status.as_deref()
        && !matches!(status, "sent" | "confirmed" | "rejected")
    {
        return Err(ApiError::BadRequest("invalid status".into()));
    }

    let reviewed_at = payload
        .status
        .as_deref()
        .filter(|status| matches!(*status, "confirmed" | "rejected"))
        .map(|_| sqlx::types::time::OffsetDateTime::now_utc());

    let mut tx = state.db.begin().await?;

    let request = sqlx::query_as::<_, ProfileRequestRecord>(
        r#"
        update profile_request
        set
            requested_profile_data_id = coalesce($1, requested_profile_data_id),
            status = coalesce($2, status),
            reviewer_note = coalesce($3, reviewer_note),
            reviewed_at = coalesce($4, reviewed_at)
        where profile_request_id = cast($1 as uuid)
        returning
            profile_request_id,
            user_id,
            requested_profile_data_id,
            status,
            reviewer_note,
            reviewed_at,
            created_at,
            updated_at
        "#,
    )
    .bind(db_uuid(request_id))
    .bind(payload.status.clone())
    .bind(payload.reviewer_note)
    .bind(db_optional_timestamp(reviewed_at))
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(ApiError::NotFound)?;

    if payload.status.as_deref() == Some("confirmed") {
        sqlx::query(
            r#"
            update "user"
            set agent_data_id = $2
            where user_id = $1
            "#,
        )
        .bind(db_uuid(request.user_id))
        .bind(db_uuid(request.requested_profile_data_id))
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    if request.status == "confirmed" {
        notifier::notify_user(
            &state,
            request.user_id,
            "Profile approved. Full operative access live. Open target intel and resume hunt.",
        )
        .await;
    } else if request.status == "rejected" {
        let note = request
            .reviewer_note
            .as_deref()
            .unwrap_or("No reviewer note attached.");
        notifier::notify_user(
            &state,
            request.user_id,
            format!("Profile rejected. Edit dossier and resend. Reviewer note: {note}"),
        )
        .await;
    }

    Ok(Json(helpers::to_profile_request_response(request)))
}

#[utoipa::path(
    delete,
    path = "/admin/profile-requests/{request_id}",
    tag = "admin",
    params(("request_id" = Uuid, Path, description = "Profile request id")),
    responses(
        (status = 204, description = "Profile request deleted"),
        (status = 404, description = "Profile request not found", body = crate::api::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_delete_profile_request(
    State(state): State<ApiContext>,
    AdminUser(_user): AdminUser,
    Path(request_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let result = sqlx::query(
        r#"
            delete from
                profile_request
            where profile_request_id = $1
            "#,
    )
    .bind(db_uuid(request_id))
    .execute(&state.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
