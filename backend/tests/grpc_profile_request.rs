mod common;

use common::{ADMIN_SECRET, JWT_SECRET, TestContext};
use cukiller_backend::ApiContext;
use cukiller_backend::config::Config;
use cukiller_backend::grpc;
use cukiller_backend::grpc::services::profile_request::profilerequest::{
    AgentDataMetadata, CreateProfileRequestRequest, ListProfileRequestsRequest, ProfileRequestId,
    UpdateProfileRequestRequest, profile_request_client::ProfileRequestClient,
};
use cukiller_backend::models::auth::AuthUserRecord;
use cukiller_backend::models::profile::ProfileRequestEvent;
use cukiller_backend::models::user::User;
use cukiller_backend::rest::helpers;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use std::sync::Arc;
use tokio::task::JoinHandle;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic::{Code, Request};
use uuid::Uuid;

struct GrpcTestContext {
    ctx: TestContext,
    state: ApiContext,
    base_url: String,
    server: JoinHandle<std::io::Result<()>>,
}

impl Drop for GrpcTestContext {
    fn drop(&mut self) {
        self.server.abort();
    }
}

impl GrpcTestContext {
    async fn new() -> Self {
        let ctx = TestContext::new().await;
        let state = ApiContext {
            db: ctx.db.clone(),
            bucket: Arc::new(test_bucket()),
            config: Config {
                database_url: String::new(),
                bind_address: "127.0.0.1:0".parse().expect("bind address"),
                admin_secret: ADMIN_SECRET.to_string(),
                jwt_secret: JWT_SECRET.to_string(),
                cors_origin: "http://localhost:5173".to_string(),
                s3_access_key: String::new(),
                s3_secret_key: String::new(),
                s3_endpoint: String::new(),
                s3_region: "us-east-1".to_string(),
                s3_bucket_name: "test-bucket".to_string(),
                #[cfg(feature = "telegram")]
                telegram_bot_token: "test-bot-token".to_string(),
                #[cfg(feature = "telegram")]
                public_webapp_url: "https://test.example.com".to_string(),
            },
            admin_secret: ADMIN_SECRET.to_string(),
            jwt_secret: JWT_SECRET.to_string(),
            profile_request_tx: tokio::sync::broadcast::channel::<ProfileRequestEvent>(16).0,
            #[cfg(feature = "telegram")]
            telegram_bot_token: "test-bot-token".to_string(),
            #[cfg(feature = "telegram")]
            public_webapp_url: "https://test.example.com".to_string(),
        };

        let app = grpc::router(state.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind grpc listener");
        let addr = listener.local_addr().expect("listener addr");
        let server = tokio::spawn(async move {
            axum::serve(
                listener,
                app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
            )
            .await
        });

        Self {
            ctx,
            state,
            base_url: format!("http://{addr}"),
            server,
        }
    }

    async fn client(&self) -> ProfileRequestClient<Channel> {
        ProfileRequestClient::connect(self.base_url.clone())
            .await
            .expect("connect grpc client")
    }

    async fn seed_user_token(&self, username: &str) -> String {
        let mut tx = self.ctx.db.begin().await.expect("transaction");
        let user = User::create(&mut *tx, Some(username.to_string()), false, None)
            .await
            .expect("create user");
        tx.commit().await.expect("commit seeded user");

        helpers::create_token_pair(
            &self.state,
            &AuthUserRecord {
                auth_user_id: Uuid::now_v7(),
                user_id: Some(user.user_id),
                telegram_id: None,
                email: None,
                password_hash: None,
            },
            Some(user),
        )
        .expect("token pair")
        .access_token
    }
}

fn test_bucket() -> Box<Bucket> {
    Bucket::new(
        "test-bucket",
        Region::Custom {
            region: "us-east-1".into(),
            endpoint: "http://127.0.0.1:9000".into(),
        },
        Credentials {
            access_key: Some("test".into()),
            secret_key: Some("test".into()),
            security_token: None,
            session_token: None,
            expiration: None,
        },
    )
    .expect("test bucket")
    .with_path_style()
}

fn auth_request<T>(message: T, token: &str) -> Request<T> {
    let mut request = Request::new(message);
    request.metadata_mut().insert(
        "authorization",
        MetadataValue::try_from(format!("Bearer {token}")).expect("auth metadata"),
    );
    request
}

fn admin_request<T>(message: T) -> Request<T> {
    let mut request = Request::new(message);
    request.metadata_mut().insert(
        "x-admin-secret",
        MetadataValue::try_from(ADMIN_SECRET).expect("admin metadata"),
    );
    request
}

fn create_request_body(codename: &str) -> CreateProfileRequestRequest {
    CreateProfileRequestRequest {
        requested_profile_data: Some(AgentDataMetadata {
            codename: Some(codename.to_string()),
            academic_group: Some("CS-1".to_string()),
            academic_level: Some("Bachelor".to_string()),
            course_number: Some(2),
            bachelor_track: Some("SWE".to_string()),
            identification_name: Some(format!("{codename} Name")),
            physical_contact_allowed: true,
            hugs_close_proximity_allowed: false,
        }),
        image: None,
    }
}

#[tokio::test]
async fn grpc_profile_request_create_list_and_get_match_rest_behavior() {
    let grpc = GrpcTestContext::new().await;
    let token = grpc.seed_user_token("Alpha").await;

    let mut client = grpc.client().await;
    let created = client
        .create_profile_request(auth_request(create_request_body("Cipher"), &token))
        .await
        .expect("create profile request")
        .into_inner();

    assert_eq!(created.status, "sent");
    assert_eq!(created.reviewer_note, None);
    assert_eq!(
        created
            .requested_profile_data
            .as_ref()
            .and_then(|data| data.codename.as_deref()),
        Some("Cipher")
    );

    let listed = client
        .list_profile_requests(auth_request(
            ListProfileRequestsRequest { all: false },
            &token,
        ))
        .await
        .expect("list profile requests")
        .into_inner();

    assert_eq!(listed.profile_requests.len(), 1);
    assert_eq!(
        listed.profile_requests[0].profile_request_id,
        created.profile_request_id
    );

    let fetched = client
        .get_profile_request(auth_request(
            ProfileRequestId {
                profile_request_id: created.profile_request_id.clone(),
            },
            &token,
        ))
        .await
        .expect("get profile request")
        .into_inner();

    assert_eq!(fetched.profile_request_id, created.profile_request_id);
    assert_eq!(fetched.user_id, created.user_id);
}

#[tokio::test]
async fn grpc_profile_request_list_all_and_update_follow_admin_rules() {
    let grpc = GrpcTestContext::new().await;
    let token_a = grpc.seed_user_token("Bravo").await;
    let token_b = grpc.seed_user_token("Charlie").await;

    let mut client = grpc.client().await;
    let created_a = client
        .create_profile_request(auth_request(create_request_body("Bravo"), &token_a))
        .await
        .expect("create request a")
        .into_inner();
    client
        .create_profile_request(auth_request(create_request_body("Charlie"), &token_b))
        .await
        .expect("create request b");

    let listed_as_user = client
        .list_profile_requests(auth_request(
            ListProfileRequestsRequest { all: true },
            &token_a,
        ))
        .await
        .expect("user list requests")
        .into_inner();
    assert_eq!(listed_as_user.profile_requests.len(), 1);

    let listed_as_admin = client
        .list_profile_requests(admin_request(ListProfileRequestsRequest { all: true }))
        .await
        .expect("admin list requests")
        .into_inner();
    assert_eq!(listed_as_admin.profile_requests.len(), 2);

    let err = client
        .update_profile_request(auth_request(
            UpdateProfileRequestRequest {
                profile_request_id: created_a.profile_request_id.clone(),
                status: Some("confirmed".to_string()),
                reviewer_note: Some("approved".to_string()),
            },
            &token_a,
        ))
        .await
        .expect_err("non-admin update should fail");
    assert_eq!(err.code(), Code::PermissionDenied);

    let updated = client
        .update_profile_request(admin_request(UpdateProfileRequestRequest {
            profile_request_id: created_a.profile_request_id,
            status: Some("confirmed".to_string()),
            reviewer_note: Some("approved".to_string()),
        }))
        .await
        .expect("admin update request")
        .into_inner();

    assert_eq!(updated.status, "confirmed");
    assert_eq!(updated.reviewer_note.as_deref(), Some("approved"));
    assert!(updated.reviewed_at.is_some());
}

#[tokio::test]
async fn grpc_profile_request_delete_obeys_ownership() {
    let grpc = GrpcTestContext::new().await;
    let owner_token = grpc.seed_user_token("Delta").await;
    let other_token = grpc.seed_user_token("Echo").await;

    let mut client = grpc.client().await;
    let created = client
        .create_profile_request(auth_request(create_request_body("Delta"), &owner_token))
        .await
        .expect("create request")
        .into_inner();

    let err = client
        .delete_profile_request(auth_request(
            ProfileRequestId {
                profile_request_id: created.profile_request_id.clone(),
            },
            &other_token,
        ))
        .await
        .expect_err("non-owner delete should fail");
    assert_eq!(err.code(), Code::PermissionDenied);

    let deleted = client
        .delete_profile_request(auth_request(
            ProfileRequestId {
                profile_request_id: created.profile_request_id.clone(),
            },
            &owner_token,
        ))
        .await
        .expect("owner delete request")
        .into_inner();
    assert!(deleted.deleted);

    let err = client
        .get_profile_request(auth_request(
            ProfileRequestId {
                profile_request_id: created.profile_request_id,
            },
            &owner_token,
        ))
        .await
        .expect_err("deleted request should not exist");
    assert_eq!(err.code(), Code::NotFound);
}
