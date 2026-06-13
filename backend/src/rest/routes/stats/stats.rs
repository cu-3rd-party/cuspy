use crate::ApiContext;
use crate::rest::extractor::AuthUser;
use crate::rest::helpers;
use crate::rest::models::kill::{RankingEntry, UserStatsResponse};
use crate::rest::models::{ApiError, db_uuid};
use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/stats/rankings",
    tag = "stats",
    responses(
        (status = 200, description = "Current ranking leaderboard", body = [RankingEntry]),
        (status = 401, description = "Unauthorized", body = crate::rest::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::rest::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn rankings(
    State(state): State<ApiContext>,
    AuthUser(_user): AuthUser,
) -> Result<Json<Vec<RankingEntry>>, ApiError> {
    let entries = sqlx::query_as::<_, RankingEntry>(
        r#"
        with latest_ratings as (
            select distinct on (user_id) user_id, rating
            from rating_history
            order by user_id, created_at desc, rating_history_id desc
        ),
        kill_totals as (
            select
                user_id,
                coalesce(sum(approved_kills), 0)::bigint as approved_kills,
                coalesce(sum(approved_deaths), 0)::bigint as approved_deaths
            from (
                select killer_id as user_id, count(*)::bigint as approved_kills, 0::bigint as approved_deaths
                from kill_event
                where status = 'ADMIN_APPROVED'
                group by killer_id
                union all
                select victim_id as user_id, 0::bigint as approved_kills, count(*)::bigint as approved_deaths
                from kill_event
                where status = 'ADMIN_APPROVED'
                group by victim_id
            ) totals
                group by user_id
        ),
        leaderboard as (
        select
            rank() over (order by coalesce(latest_ratings.rating, $1) desc, u.created_at asc)::bigint as rank,
            u.user_id,
            u.agent_name,
            coalesce(latest_ratings.rating, $1) as rating,
            coalesce(kill_totals.approved_kills, 0) as approved_kills,
            coalesce(kill_totals.approved_deaths, 0) as approved_deaths
        from "user" u
        left join latest_ratings on latest_ratings.user_id = u.user_id
        left join kill_totals on kill_totals.user_id = u.user_id
        )
        select rank, cast(user_id as text) as user_id, agent_name, rating, approved_kills, approved_deaths
        from leaderboard
        order by rank asc, user_id asc
        "#,
    )
    .bind(helpers::DEFAULT_RATING)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(entries))
}

#[utoipa::path(
    get,
    path = "/stats/user/{user_id}",
    tag = "stats",
    params(("user_id" = Uuid, Path, description = "User id")),
    responses(
        (status = 200, description = "User gameplay statistics", body = UserStatsResponse),
        (status = 401, description = "Unauthorized", body = crate::rest::models::ErrorResponse),
        (status = 404, description = "User not found", body = crate::rest::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::rest::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn user_stats(
    State(state): State<ApiContext>,
    AuthUser(_user): AuthUser,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserStatsResponse>, ApiError> {
    let stats = sqlx::query_as::<_, UserStatsResponse>(
        r#"
        with latest_rating as (
            select rating
            from rating_history
            where cast(user_id as text) = $1
            order by created_at desc, rating_history_id desc
            limit 1
        )
        select
            cast($1 as text) as user_id,
            coalesce((select rating from latest_rating), $2) as rating,
            (
                select count(*)::bigint
                from kill_event
                where cast(killer_id as text) = $1 and status = 'ADMIN_APPROVED'
            ) as approved_kills,
            (
                select count(*)::bigint
                from kill_event
                where cast(victim_id as text) = $1 and status = 'ADMIN_APPROVED'
            ) as approved_deaths,
            (
                select count(*)::bigint
                from kill_event
                where cast(killer_id as text) = $1 and status in ('REPORTED', 'VICTIM_CONFIRMED')
            ) as pending_kills
        where exists (select 1 from "user" where cast(user_id as text) = $1)
        "#,
    )
    .bind(db_uuid(user_id))
    .bind(helpers::DEFAULT_RATING)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(stats))
}
