use crate::ApiContext;
use crate::api::models::kill::{KillEventRecord, KillEventResponse};
use crate::api::models::{ApiError, db_uuid, kill};
use crate::api::{extractor, helpers};
use axum::Json;
use axum::extract::{Query, State};
use http::HeaderMap;
use serde::Deserialize;
use sqlx::any::AnyArguments;
use sqlx::query::QueryAs;
use sqlx::{Any, QueryBuilder};
use uuid::Uuid;
use crate::api::extractor::AuthUser;

#[derive(Deserialize)]
pub struct ListParams {
    status: Option<Vec<String>>,
    killer_user_id: Option<Uuid>,
    victim_user_id: Option<Uuid>,
}

impl ListParams {
    pub fn build_query(self) -> String {
        let mut builder = QueryBuilder::<Any>::new(
            r#"
            select
                kill_event_id,
                killer_id,
                victim_id,
                status,
                evidence_resource_id,
                details,
                killer_confirmed_at,
                victim_confirmed_at,
                confirmed_at,
                moderated_at,
                moderator_id,
                moderation_reason,
                rating_applied_at,
                created_at,
                updated_at
            from kill_event
        "#,
        );
        let mut conditions = vec![];
        let mut bind_values = vec![];

        if let Some(user_id) = self.killer_user_id {
            conditions.push("killer_user_id=".to_string());
            bind_values.push(db_uuid(user_id));
        }
        if let Some(user_id) = self.victim_user_id {
            conditions.push("victim_user_id=".to_string());
            bind_values.push(db_uuid(user_id));
        }
        if let Some(statuses) = self.status {
            for status in statuses {
                conditions.push("status=".to_string());
                bind_values.push(status);
            }
        }

        if !conditions.is_empty() && !bind_values.is_empty() {
            builder.push(" where ");
            let mut separated = builder.separated(" or ");
            for (condition, bind_value) in conditions.into_iter().zip(bind_values.into_iter()) {
                separated.push(&condition);
                separated.push_unseparated(&bind_value);
            }
        }

        builder.into_sql()
    }
}

#[utoipa::path(
    get,
    path = "/kill",
    tag = "kill",
    responses(
        (status = 200, description = "Перечисление всех убийств"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_kills(
    State(state): State<ApiContext>,
    AuthUser(_user): AuthUser,
    Query(query): Query<ListParams>,
) -> Result<Json<Vec<KillEventResponse>>, ApiError> {
    let records = sqlx::query_as(&query.build_query())
        .fetch_all(&state.db)
        .await?;

    Ok(Json(
        records.into_iter().map(kill::to_kill_response).collect(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_build_query_no_params() {
        let params = ListParams {
            status: None,
            killer_user_id: None,
            victim_user_id: None,
        };

        assert!(!params.build_query().contains("where"));
    }

    // TODO: закончить покрытие тестами
}
