use crate::grpc::RequestAuthExt;
use crate::grpc::services::helpers;
use crate::grpc::services::profile_request::profilerequest::{
    CreateProfileRequestRequest, DeleteProfileRequestResponse, ListProfileRequestsRequest,
    ListProfileRequestsResponse, UpdateProfileRequestRequest,
};
use crate::models::agent_data::{AcademicLevel, BachelorTrack};
use crate::models::agent_data::{AgentData as ModelAgentData, AgentDataMetadata};
use crate::models::profile::ProfileRequestRecord;
use crate::models::profile::{ProfileRequestEvent, ProfileRequestResponse};
use crate::models::resource::Resource;
use crate::rest::helpers::format_timestamp;
use log::info;
use profilerequest::profile_request_server::ProfileRequest;
use profilerequest::{
    AgentData as ProtoAgentData, AgentDataMetadata as ProtoAgentDataMetadata,
    ProfileRequestEvent as ProtoEvent, ProfileRequestId as ProtoProfileRequestId,
    ProfileRequestResponse as ProtoProfileRequestResponse, SubscribeRequest,
};
use s3::Bucket;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub mod profilerequest {
    tonic::include_proto!("profilerequest");
}

#[derive(Clone)]
pub struct ProfileRequestService {
    pub db: PgPool,
    pub bucket: Arc<Box<Bucket>>,
    pub tx: broadcast::Sender<ProfileRequestEvent>,
}

impl ProfileRequestService {
    pub fn new(
        db: PgPool,
        bucket: Arc<Box<Bucket>>,
        tx: broadcast::Sender<ProfileRequestEvent>,
    ) -> Self {
        Self { db, bucket, tx }
    }
}

fn academic_level_to_proto(value: AcademicLevel) -> String {
    match value {
        AcademicLevel::Bachelor => "Bachelor".to_string(),
        AcademicLevel::Master => "Master".to_string(),
    }
}

fn bachelor_track_to_proto(value: BachelorTrack) -> String {
    match value {
        BachelorTrack::SWE => "SWE".to_string(),
        BachelorTrack::AI => "AI".to_string(),
        BachelorTrack::BA => "BA".to_string(),
    }
}

fn to_proto_agent_data(agent_data: ModelAgentData) -> ProtoAgentData {
    ProtoAgentData {
        agent_data_id: agent_data.agent_data_id.to_string(),
        codename: agent_data.codename,
        academic_group: agent_data.academic_group,
        academic_level: agent_data.academic_level.map(academic_level_to_proto),
        course_number: agent_data.course_number,
        bachelor_track: agent_data.bachelor_track.map(bachelor_track_to_proto),
        identification_name: agent_data.identification_name,
        identification_image_id: agent_data
            .identification_image_id
            .map(|value| value.to_string()),
        physical_contact_allowed: agent_data.physical_contact_allowed,
        hugs_close_proximity_allowed: agent_data.hugs_close_proximity_allowed,
    }
}

fn to_proto_profile_request_record(
    record: ProfileRequestRecord,
    requested_profile_data: Option<ModelAgentData>,
) -> ProtoProfileRequestResponse {
    ProtoProfileRequestResponse {
        profile_request_id: record.profile_request_id.to_string(),
        user_id: record.user_id.to_string(),
        requested_profile_data: requested_profile_data.map(to_proto_agent_data),
        status: record.status,
        reviewer_note: record.reviewer_note,
        reviewed_at: record.reviewed_at.map(format_timestamp),
    }
}

fn parse_academic_level(value: Option<String>) -> Result<Option<AcademicLevel>, Status> {
    value
        .map(|value| match value.as_str() {
            "Bachelor" | "bachelor" => Ok(AcademicLevel::Bachelor),
            "Master" | "master" => Ok(AcademicLevel::Master),
            _ => Err(Status::invalid_argument("invalid academic_level")),
        })
        .transpose()
}

fn parse_bachelor_track(value: Option<String>) -> Result<Option<BachelorTrack>, Status> {
    value
        .map(|value| match value.as_str() {
            "SWE" | "swe" => Ok(BachelorTrack::SWE),
            "AI" | "ai" => Ok(BachelorTrack::AI),
            "BA" | "ba" => Ok(BachelorTrack::BA),
            _ => Err(Status::invalid_argument("invalid bachelor_track")),
        })
        .transpose()
}

fn to_agent_data_metadata(metadata: ProtoAgentDataMetadata) -> Result<AgentDataMetadata, Status> {
    Ok(AgentDataMetadata {
        codename: metadata.codename,
        academic_group: metadata.academic_group,
        academic_level: parse_academic_level(metadata.academic_level)?,
        course_number: metadata.course_number,
        bachelor_track: parse_bachelor_track(metadata.bachelor_track)?,
        identification_name: metadata.identification_name,
        physical_contact_allowed: metadata.physical_contact_allowed,
        hugs_close_proximity_allowed: metadata.hugs_close_proximity_allowed,
    })
}

fn to_proto_profile_request(response: ProfileRequestResponse) -> ProtoProfileRequestResponse {
    ProtoProfileRequestResponse {
        profile_request_id: response.profile_request_id.to_string(),
        user_id: response.user_id.to_string(),
        requested_profile_data: response.requested_profile_data.map(to_proto_agent_data),
        status: response.status,
        reviewer_note: response.reviewer_note,
        reviewed_at: response.reviewed_at,
    }
}

#[tonic::async_trait]
impl ProfileRequest for ProfileRequestService {
    type SubscribeStream = ReceiverStream<Result<ProtoEvent, Status>>;

    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let user = request
            .auth_user_cloned()
            .ok_or_else(|| Status::unauthenticated(""))?;
        let req = request.into_inner();

        let user_id = Uuid::parse_str(&req.user_id)
            .map_err(|_| Status::invalid_argument("invalid user_id"))?;

        if !user.is_admin && user_id != user.user_id {
            return Err(Status::permission_denied(
                "cannot subscribe to other user's events",
            ));
        }

        let mut broadcast_rx = self.tx.subscribe();

        let (tx, rx) = tokio::sync::mpsc::channel(256);

        tokio::spawn(async move {
            loop {
                info!("user={:?} subscribed to event loop", &user_id);
                match broadcast_rx.recv().await {
                    Ok(event) => {
                        info!("RECEIVED EVENT: {:?}", event);
                        if event.user_id == user_id {
                            info!("SENDING EVENT: user={:?} event = {:?}", &user_id, event);
                            if tx
                                .send(Ok(ProtoEvent {
                                    profile_request_id: event.profile_request_id.to_string(),
                                    user_id: event.user_id.to_string(),
                                    status: event.status,
                                    reviewer_note: event.reviewer_note,
                                    reviewed_at: event.reviewed_at,
                                }))
                                .await
                                .is_err()
                            {
                                break;
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        log::warn!("profile_request subscriber lagged by {n} messages");
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_profile_request(
        &self,
        request: Request<ProtoProfileRequestId>,
    ) -> Result<Response<ProtoProfileRequestResponse>, Status> {
        let user = helpers::require_authenticated_user(&request)?;
        let req = request.into_inner();

        let profile_request_id = Uuid::parse_str(&req.profile_request_id)
            .map_err(|_| Status::invalid_argument("invalid profile_request_id"))?;

        let mut tx = self.db.begin().await.map_err(helpers::internal_error)?;
        let profile = ProfileRequestRecord::get_by_id(&mut *tx, profile_request_id)
            .await
            .ok_or_else(|| Status::not_found("profile request not found"))?;

        if profile.user_id != user.user_id && !user.is_admin {
            return Err(Status::permission_denied(
                "cannot access other user's profile request",
            ));
        }

        let response = profile
            .into_response(&mut *tx)
            .await
            .map_err(helpers::api_error_to_status)?;

        tx.commit().await.map_err(helpers::internal_error)?;

        Ok(Response::new(to_proto_profile_request(response)))
    }

    async fn list_profile_requests(
        &self,
        request: Request<ListProfileRequestsRequest>,
    ) -> Result<Response<ListProfileRequestsResponse>, Status> {
        let user = helpers::require_authenticated_user(&request)?;
        let req = request.into_inner();

        let mut tx = self.db.begin().await.map_err(helpers::internal_error)?;
        let requests = if req.all && user.is_admin {
            ProfileRequestRecord::get_all(&mut *tx)
                .await
                .map_err(helpers::api_error_to_status)?
        } else {
            ProfileRequestRecord::get_by_user_id(&mut *tx, user.user_id)
                .await
                .map_err(helpers::api_error_to_status)?
        };

        let agent_data_ids: Vec<_> = requests
            .iter()
            .map(|r| r.requested_profile_data_id)
            .collect();
        let agent_data_map = ModelAgentData::get_by_ids(&mut *tx, &agent_data_ids).await;

        let profile_requests = requests
            .into_iter()
            .map(|r| {
                let requested_profile_data =
                    agent_data_map.get(&r.requested_profile_data_id).cloned();
                to_proto_profile_request_record(r, requested_profile_data)
            })
            .collect();

        tx.commit().await.map_err(helpers::internal_error)?;

        Ok(Response::new(ListProfileRequestsResponse {
            profile_requests,
        }))
    }

    async fn create_profile_request(
        &self,
        request: Request<CreateProfileRequestRequest>,
    ) -> Result<Response<ProtoProfileRequestResponse>, Status> {
        let user = helpers::require_authenticated_user(&request)?;
        let req = request.into_inner();
        let metadata = req
            .requested_profile_data
            .ok_or_else(|| Status::invalid_argument("requested_profile_data is required"))?;
        let metadata = to_agent_data_metadata(metadata)?;

        if ProfileRequestRecord::get_by_user_id(&self.db, user.user_id)
            .await
            .map_err(helpers::api_error_to_status)?
            .into_iter()
            .any(|r| r.status == "sent")
        {
            return Err(Status::invalid_argument(
                "you already have a pending profile",
            ));
        }

        let mut tx = self.db.begin().await.map_err(helpers::internal_error)?;
        let resource = match req.image {
            Some(upload) => Some(
                Resource::new(
                    &mut *tx,
                    self.bucket.clone(),
                    upload.content.into(),
                    upload.content_type,
                )
                .await
                .map_err(helpers::api_error_to_status)?,
            ),
            None => None,
        };
        let agent_data = ModelAgentData::create(&mut *tx, metadata, resource)
            .await
            .map_err(helpers::api_error_to_status)?;
        let profile = ProfileRequestRecord::create(
            &mut *tx,
            user.user_id,
            agent_data.agent_data_id,
            "sent".to_string(),
        )
        .await
        .map_err(helpers::api_error_to_status)?;
        let response = profile
            .into_response(&mut *tx)
            .await
            .map_err(helpers::api_error_to_status)?;
        tx.commit().await.map_err(helpers::internal_error)?;

        Ok(Response::new(to_proto_profile_request(response)))
    }

    async fn update_profile_request(
        &self,
        request: Request<UpdateProfileRequestRequest>,
    ) -> Result<Response<ProtoProfileRequestResponse>, Status> {
        let _user = helpers::require_admin_user(&request)?;
        let req = request.into_inner();

        let profile_request_id = Uuid::parse_str(&req.profile_request_id)
            .map_err(|_| Status::invalid_argument("invalid profile_request_id"))?;

        let mut tx = self.db.begin().await.map_err(helpers::internal_error)?;
        let profile = ProfileRequestRecord::get_by_id(&mut *tx, profile_request_id)
            .await
            .ok_or_else(|| Status::not_found("profile request not found"))?;

        let profile = profile
            .update(&mut *tx, req.status, req.reviewer_note)
            .await
            .map_err(helpers::api_error_to_status)?;

        let response = profile
            .into_response(&mut *tx)
            .await
            .map_err(helpers::api_error_to_status)?;
        tx.commit().await.map_err(helpers::internal_error)?;

        Ok(Response::new(to_proto_profile_request(response)))
    }

    async fn delete_profile_request(
        &self,
        request: Request<ProtoProfileRequestId>,
    ) -> Result<Response<DeleteProfileRequestResponse>, Status> {
        let user = helpers::require_authenticated_user(&request)?;
        let req = request.into_inner();

        let profile_request_id = Uuid::parse_str(&req.profile_request_id)
            .map_err(|_| Status::invalid_argument("invalid profile_request_id"))?;

        let mut tx = self.db.begin().await.map_err(helpers::internal_error)?;
        let profile = ProfileRequestRecord::get_by_id(&mut *tx, profile_request_id)
            .await
            .ok_or_else(|| Status::not_found("profile request not found"))?;
        if profile.user_id != user.user_id && !user.is_admin {
            return Err(Status::permission_denied(
                "cannot delete other user's profile request",
            ));
        }
        let deleted = profile
            .delete(&mut *tx)
            .await
            .map_err(helpers::api_error_to_status)?;
        if deleted {
            tx.commit().await.map_err(helpers::internal_error)?;
        }

        Ok(Response::new(DeleteProfileRequestResponse { deleted }))
    }
}

#[cfg(test)]
mod tests {
    use crate::grpc::services::profile_request::ProfileRequestService;
    use crate::grpc::services::profile_request::ProtoEvent;
    use crate::grpc::services::profile_request::profilerequest::SubscribeRequest;
    use crate::grpc::services::profile_request::profilerequest::profile_request_server::ProfileRequest;
    use crate::models::profile::ProfileRequestEvent;
    use crate::models::user::User;
    use s3::creds::Credentials;
    use s3::{Bucket, Region};
    use sqlx::PgPool;
    use sqlx::postgres::PgPoolOptions;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::broadcast;
    use tonic::{Code, Request, Status};
    use uuid::Uuid;

    fn make_user(id: Uuid, is_admin: bool) -> User {
        User {
            user_id: id,
            username: None,
            agent_data_id: None,
            rating: 0,
            is_admin,
            created_at: time::OffsetDateTime::now_utc(),
            updated_at: Some(time::OffsetDateTime::now_utc()),
        }
    }

    fn make_event(user_id: Uuid, status: &str) -> ProfileRequestEvent {
        ProfileRequestEvent {
            profile_request_id: Uuid::now_v7(),
            user_id,
            status: status.to_string(),
            reviewer_note: None,
            reviewed_at: None,
        }
    }

    const UID_A: Uuid = Uuid::nil();
    const UID_B: Uuid = Uuid::from_u128(1);

    fn test_db() -> PgPool {
        PgPoolOptions::new()
            .connect_lazy("postgres://postgres:postgres@localhost/cukiller")
            .expect("lazy test pool")
    }

    fn test_bucket() -> Arc<Box<Bucket>> {
        Arc::new(
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
            .with_path_style(),
        )
    }

    async fn subscribe(
        svc: &ProfileRequestService,
        subscribe_to: Uuid,
        as_user: User,
    ) -> tokio::sync::mpsc::Receiver<Result<ProtoEvent, Status>> {
        let mut req = Request::new(SubscribeRequest {
            user_id: subscribe_to.to_string(),
        });
        req.extensions_mut().insert(Some(as_user));
        svc.subscribe(req)
            .await
            .expect("subscribe should succeed")
            .into_inner()
            .into_inner()
    }

    async fn collect_events(
        rx: &mut tokio::sync::mpsc::Receiver<Result<ProtoEvent, Status>>,
    ) -> Vec<ProtoEvent> {
        let mut out = Vec::new();
        while let Some(Ok(event)) = rx.recv().await {
            out.push(event);
        }
        out
    }

    // ─── Happy path ───────────────────────────────────────────────

    #[tokio::test]
    async fn user_receives_only_own_events() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(test_db(), test_bucket(), tx.clone());

        let user = make_user(UID_A, false);
        let mut rx = subscribe(&svc, UID_A, user).await;
        drop(svc);

        tx.send(make_event(UID_A, "confirmed")).unwrap();
        tx.send(make_event(UID_B, "rejected")).unwrap();
        tx.send(make_event(UID_A, "sent")).unwrap();
        drop(tx);

        let events = collect_events(&mut rx).await;
        assert_eq!(events.len(), 2);
        assert!(events.iter().all(|e| e.user_id == UID_A.to_string()));
    }

    #[tokio::test]
    async fn two_users_independent_filters() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(test_db(), test_bucket(), tx.clone());

        let alice = make_user(UID_A, false);
        let bob = make_user(UID_B, false);

        let mut rx_a = subscribe(&svc, UID_A, alice).await;
        let mut rx_b = subscribe(&svc, UID_B, bob).await;
        drop(svc);

        tx.send(make_event(UID_A, "a1")).unwrap();
        tx.send(make_event(UID_B, "b1")).unwrap();
        tx.send(make_event(UID_A, "a2")).unwrap();
        drop(tx);

        let a_events = collect_events(&mut rx_a).await;
        let b_events = collect_events(&mut rx_b).await;

        assert_eq!(a_events.len(), 2);
        assert!(a_events.iter().all(|e| e.user_id == UID_A.to_string()));
        assert_eq!(b_events.len(), 1);
        assert_eq!(b_events[0].user_id, UID_B.to_string());
    }

    #[tokio::test]
    async fn same_user_two_subscribers_both_get_events() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(test_db(), test_bucket(), tx.clone());

        let user = make_user(UID_A, false);
        let mut rx1 = subscribe(&svc, UID_A, user.clone()).await;
        let mut rx2 = subscribe(&svc, UID_A, user).await;
        drop(svc);

        tx.send(make_event(UID_A, "e1")).unwrap();
        tx.send(make_event(UID_A, "e2")).unwrap();
        drop(tx);

        let e1 = collect_events(&mut rx1).await;
        let e2 = collect_events(&mut rx2).await;
        assert_eq!(e1.len(), 2);
        assert_eq!(e2.len(), 2);
    }

    #[tokio::test]
    async fn admin_subscribes_to_arbitrary_user() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(test_db(), test_bucket(), tx.clone());

        let admin = make_user(UID_A, true);
        let target = Uuid::from_u128(42);

        let mut rx = subscribe(&svc, target, admin).await;
        drop(svc);

        tx.send(make_event(target, "ok")).unwrap();
        tx.send(make_event(UID_A, "ignore-me")).unwrap();
        drop(tx);

        let events = collect_events(&mut rx).await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].user_id, target.to_string());
    }

    // ─── Error cases ──────────────────────────────────────────────

    #[tokio::test]
    async fn unauthenticated_request_rejected() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(test_db(), test_bucket(), tx.clone());

        let mut req = Request::new(SubscribeRequest {
            user_id: UID_A.to_string(),
        });
        req.extensions_mut().insert(None::<User>);

        let err = svc.subscribe(req).await.unwrap_err();
        assert_eq!(err.code(), Code::Unauthenticated);
    }

    #[tokio::test]
    async fn non_admin_cannot_spy_on_other_user() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(test_db(), test_bucket(), tx.clone());
        let user = make_user(UID_A, false);

        let mut req = Request::new(SubscribeRequest {
            user_id: UID_B.to_string(),
        });
        req.extensions_mut().insert(Some(user));

        let err = svc.subscribe(req).await.unwrap_err();
        assert_eq!(err.code(), Code::PermissionDenied);
    }

    #[tokio::test]
    async fn invalid_user_id_rejected() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(test_db(), test_bucket(), tx.clone());
        let user = make_user(UID_A, false);

        let mut req = Request::new(SubscribeRequest {
            user_id: "not-a-uuid".to_string(),
        });
        req.extensions_mut().insert(Some(user));

        let err = svc.subscribe(req).await.unwrap_err();
        assert_eq!(err.code(), Code::InvalidArgument);
    }

    // ─── Stream lifecycle ──────────────────────────────────────────

    #[tokio::test]
    async fn stream_delivers_then_ends_when_senders_gone() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(test_db(), test_bucket(), tx.clone());

        let user = make_user(UID_A, false);
        let mut rx = subscribe(&svc, UID_A, user).await;
        drop(svc);

        tx.send(make_event(UID_A, "ok")).unwrap();
        drop(tx);

        assert!(rx.recv().await.is_some());
        assert!(rx.recv().await.is_none());
    }

    #[tokio::test]
    async fn no_events_immediate_end() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(test_db(), test_bucket(), tx.clone());

        let user = make_user(UID_A, false);
        let mut rx = subscribe(&svc, UID_A, user).await;
        drop(svc);
        drop(tx);

        assert!(rx.recv().await.is_none());
    }

    #[tokio::test]
    async fn lagged_receiver_logs_and_continues() {
        let (tx, _) = broadcast::channel(2);
        let svc = ProfileRequestService::new(test_db(), test_bucket(), tx.clone());
        let user = make_user(UID_A, false);

        let mut rx = subscribe(&svc, UID_A, user).await;
        drop(svc);

        tx.send(make_event(UID_A, "a")).unwrap();
        tx.send(make_event(UID_A, "b")).unwrap();
        tx.send(make_event(UID_A, "c")).unwrap(); // forces lag on broadcast_rx
        tokio::time::sleep(Duration::from_millis(100)).await;

        tx.send(make_event(UID_A, "d")).unwrap();
        drop(tx);

        let events = collect_events(&mut rx).await;
        assert!(!events.is_empty(), "should receive post-lag events");
        assert_eq!(events.last().unwrap().status, "d");
    }

    #[tokio::test]
    async fn dropping_one_subscriber_does_not_affect_other() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(test_db(), test_bucket(), tx.clone());
        let user = make_user(UID_A, false);

        let mut rx1 = subscribe(&svc, UID_A, user.clone()).await;
        let mut rx2 = subscribe(&svc, UID_A, user).await;
        drop(svc);

        rx1.close();

        tx.send(make_event(UID_A, "ok")).unwrap();
        drop(tx);

        let events = collect_events(&mut rx2).await;
        assert_eq!(events.len(), 1);
    }
}
