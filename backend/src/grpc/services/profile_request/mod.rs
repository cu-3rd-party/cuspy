use crate::grpc::RequestAuthExt;
use crate::models::agent_data::AgentData as ModelAgentData;
use crate::models::profile::{ProfileRequestEvent, ProfileRequestResponse};
use crate::models::profile::ProfileRequestRecord;
use log::info;
use profilerequest::profile_request_server::ProfileRequest;
use profilerequest::{
    AgentData as ProtoAgentData, ProfileRequestEvent as ProtoEvent,
    ProfileRequestId as ProtoProfileRequestId, ProfileRequestResponse as ProtoProfileRequestResponse,
    SubscribeRequest,
};
use sqlx::PgPool;
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
    pub tx: broadcast::Sender<ProfileRequestEvent>,
}

impl ProfileRequestService {
    pub fn new(db: PgPool, tx: broadcast::Sender<ProfileRequestEvent>) -> Self {
        Self { db, tx }
    }
}

fn internal_error(error: impl std::fmt::Display) -> Status {
    Status::internal(error.to_string())
}

fn to_proto_agent_data(agent_data: ModelAgentData) -> ProtoAgentData {
    ProtoAgentData {
        agent_data_id: agent_data.agent_data_id.to_string(),
        codename: agent_data.codename,
        academic_group: agent_data.academic_group,
        academic_level: agent_data.academic_level.map(|value| value.to_string()),
        course_number: agent_data.course_number,
        bachelor_track: agent_data.bachelor_track.map(|value| value.to_string()),
        identification_name: agent_data.identification_name,
        identification_image_id: agent_data.identification_image_id.map(|value| value.to_string()),
        physical_contact_allowed: agent_data.physical_contact_allowed,
        hugs_close_proximity_allowed: agent_data.hugs_close_proximity_allowed,
    }
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
        let user = request
            .auth_user_cloned()
            .ok_or_else(|| Status::unauthenticated(""))?;
        let req = request.into_inner();

        let profile_request_id = Uuid::parse_str(&req.profile_request_id)
            .map_err(|_| Status::invalid_argument("invalid profile_request_id"))?;

        let mut tx = self.db.begin().await.map_err(internal_error)?;
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
            .map_err(internal_error)?;

        tx.commit().await.map_err(internal_error)?;

        Ok(Response::new(to_proto_profile_request(response)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::User;
    use sqlx::postgres::PgPoolOptions;
    use std::time::Duration;
    use tonic::Code;

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
        let svc = ProfileRequestService::new(test_db(), tx.clone());

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
        let svc = ProfileRequestService::new(test_db(), tx.clone());

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
        let svc = ProfileRequestService::new(test_db(), tx.clone());

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
        let svc = ProfileRequestService::new(test_db(), tx.clone());

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
        let svc = ProfileRequestService::new(test_db(), tx.clone());

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
        let svc = ProfileRequestService::new(test_db(), tx.clone());
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
        let svc = ProfileRequestService::new(test_db(), tx.clone());
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
        let svc = ProfileRequestService::new(test_db(), tx.clone());

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
        let svc = ProfileRequestService::new(test_db(), tx.clone());

        let user = make_user(UID_A, false);
        let mut rx = subscribe(&svc, UID_A, user).await;
        drop(svc);
        drop(tx);

        assert!(rx.recv().await.is_none());
    }

    #[tokio::test]
    async fn lagged_receiver_logs_and_continues() {
        let (tx, _) = broadcast::channel(2);
        let svc = ProfileRequestService::new(test_db(), tx.clone());
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
        let svc = ProfileRequestService::new(test_db(), tx.clone());
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
