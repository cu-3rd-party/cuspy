use crate::ApiContext;
use crate::models;
use crate::rest;
use crate::rest::OpenApi;
use crate::rest::health;
use crate::rest::routes;
use axum::Router;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa_swagger_ui::SwaggerUi;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi
            .components
            .get_or_insert_with(utoipa::openapi::Components::new);

        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        rest::root,
        health::health,
        routes::auth::login::login,
        routes::auth::me::me,
        routes::auth::register::register,
        routes::user::get::get_user,
        routes::user::update::update_user,
        routes::user::delete::delete_user,
        routes::agent_data::create::create_agent_data,
        routes::agent_data::get::get_agent_data,
        routes::resource::get::get_resource,
        routes::kill::list::list_kills,
        routes::kill::report::report_kill,
        routes::kill::confirm::confirm_kill,
        routes::kill::moderate::moderate_kill,
        routes::profile_request::list::list_profile_requests,
        routes::profile_request::get::get_profile_request,
        routes::profile_request::create::create_profile_request,
        routes::profile_request::update::update_profile_request,
        routes::profile_request::delete::delete_profile_request,
        routes::stats::stats::rankings,
        routes::stats::stats::user_stats,
        routes::admin::users_crud::admin_list_users,
        routes::admin::users_crud::admin_create_user,
        routes::admin::users_crud::admin_get_user,
        routes::admin::users_crud::admin_update_user,
        routes::admin::users_crud::admin_delete_user,
        routes::admin::profile_request_crud::admin_list_profile_requests,
        routes::admin::profile_request_crud::admin_get_profile_request,
        routes::admin::profile_request_crud::admin_update_profile_request,
        routes::admin::profile_request_crud::admin_delete_profile_request,
    ),
    components(
        schemas(
            models::ErrorResponse,
            models::HealthResponse,
            models::auth::LoginRequest,
            models::auth::RegisterRequest,
            models::auth::AuthResponse,
            models::user::UserResponse,
            models::user::CreateUserRequest,
            models::user::UpdateUserRequest,
            models::agent_data::AcademicLevel,
            models::agent_data::BachelorTrack,
            models::agent_data::AgentData,
            models::agent_data::AgentDataMetadata,
            routes::agent_data::create::CreateAgentDataMultipartRequest,
            models::resource::Resource,
            models::kill::KillEventResponse,
            models::kill::ReportKillRequest,
            models::kill::ConfirmKillRequest,
            models::kill::RankingEntry,
            models::kill::UserStatsResponse,
            routes::kill::moderate::ModerationActions,
            routes::kill::moderate::ModerateKillRequest,
            models::profile::ProfileRequestResponse,
            models::profile::CreateProfileRequest,
            models::profile::UpdateProfileRequest,
            models::profile::AdminUpdateProfileRequest,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "system", description = "System endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "user", description = "User endpoints"),
        (name = "agent-data", description = "Agent data endpoints"),
        (name = "resource", description = "Resource endpoints"),
        (name = "kill", description = "Kill event endpoints"),
        (name = "profile-request", description = "Profile request endpoints"),
        (name = "stats", description = "Stats endpoints"),
        (name = "admin", description = "Administrative endpoints"),
    )
)]
pub struct ApiDoc;

pub fn docs_router() -> Router<ApiContext> {
    SwaggerUi::new("/docs")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
        .into()
}
