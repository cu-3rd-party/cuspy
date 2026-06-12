use crate::ApiContext;
use crate::api::extractor::AuthUser;
use crate::api::models::kill::KillEventResponse;
use crate::api::models::{ApiError, kill};
use crate::api::routes::kill::helpers::KILL_EVENT_COLUMNS;
use axum::Json;
use axum::extract::{Query, State};
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

#[derive(Deserialize, IntoParams)]
pub struct ListParams {
    status: Option<Vec<String>>,
    killer_user_id: Option<Uuid>,
    victim_user_id: Option<Uuid>,
}

impl ListParams {
    pub fn build_query(self) -> String {
        let mut conditions = Vec::new();

        if let Some(user_id) = self.killer_user_id {
            conditions.push(format!("cast(killer_id as text) = '{user_id}'"));
        }
        if let Some(user_id) = self.victim_user_id {
            conditions.push(format!("cast(victim_id as text) = '{user_id}'"));
        }
        if let Some(statuses) = self.status {
            let statuses: Vec<String> = statuses
                .into_iter()
                .map(|status| format!("'{}'", status.replace('\'', "''")))
                .collect();

            if !statuses.is_empty() {
                conditions.push(format!("status in ({})", statuses.join(", ")));
            }
        }

        let mut query = format!(
            r#"
            select
                {KILL_EVENT_COLUMNS}
            from kill_event
            "#
        );

        if !conditions.is_empty() {
            query.push_str(" where ");
            query.push_str(&conditions.join(" and "));
        }

        query.push_str(" order by created_at desc");
        query
    }
}

#[utoipa::path(
    get,
    path = "/kill",
    tag = "kill",
    params(ListParams),
    responses(
        (status = 200, description = "List kill events", body = [KillEventResponse]),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error", body = crate::api::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_kills(
    State(state): State<ApiContext>,
    AuthUser(_user): AuthUser,
    Query(query): Query<ListParams>,
) -> Result<Json<Vec<KillEventResponse>>, ApiError> {
    let records = sqlx::query_as::<_, kill::KillEventRecord>(&query.build_query())
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

    #[test]
    pub fn test_build_query_with_killer_filter() {
        let killer_id = Uuid::now_v7();
        let query = ListParams {
            status: None,
            killer_user_id: Some(killer_id),
            victim_user_id: None,
        }
        .build_query();

        assert!(query.contains(&format!("cast(killer_id as text) = '{killer_id}'")));
    }

    #[test]
    pub fn test_build_query_with_victim_filter() {
        let victim_id = Uuid::now_v7();
        let query = ListParams {
            status: None,
            killer_user_id: None,
            victim_user_id: Some(victim_id),
        }
        .build_query();

        assert!(query.contains(&format!("cast(victim_id as text) = '{victim_id}'")));
    }

    #[test]
    pub fn test_build_query_with_status_filter() {
        let query = ListParams {
            status: Some(vec!["REPORTED".into(), "CONFIRMED".into()]),
            killer_user_id: None,
            victim_user_id: None,
        }
        .build_query();

        assert!(query.contains("status in ('REPORTED', 'CONFIRMED')"));
    }

    #[test]
    pub fn test_build_query_combines_filters_with_and() {
        let killer_id = Uuid::now_v7();
        let victim_id = Uuid::now_v7();
        let query = ListParams {
            status: Some(vec!["ADMIN_APPROVED".into()]),
            killer_user_id: Some(killer_id),
            victim_user_id: Some(victim_id),
        }
        .build_query();

        assert!(query.contains(" and "));
        assert!(query.contains("status in ('ADMIN_APPROVED')"));
    }
}
