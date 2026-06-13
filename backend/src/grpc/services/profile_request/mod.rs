use crate::grpc::RequestAuthExt;
use crate::models::profile::ProfileRequestEvent;
use profilerequest::profile_request_server::ProfileRequest;
use profilerequest::{ProfileRequestEvent as ProtoEvent, SubscribeRequest};
use tokio::sync::broadcast;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub mod profilerequest {
    tonic::include_proto!("profilerequest");
}

#[derive(Clone)]
pub struct ProfileRequestService {
    pub tx: broadcast::Sender<ProfileRequestEvent>,
}

impl ProfileRequestService {
    pub fn new(tx: broadcast::Sender<ProfileRequestEvent>) -> Self {
        Self { tx }
    }
}

#[tonic::async_trait]
impl ProfileRequest for ProfileRequestService {
    type SubscribeStream = ReceiverStream<Result<ProtoEvent, Status>>;

    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let user = request.auth_user_cloned().ok_or_else(|| Status::unauthenticated(""))?;
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
                match broadcast_rx.recv().await {
                    Ok(event) => {
                        if event.user_id == user_id {
                            if tx
                                .send(Ok(ProtoEvent {
                                    profile_request_id: event.profile_request_id.to_string(),
                                    user_id: event.user_id.to_string(),
                                    status: event.status,
                                    reviewer_note: event.reviewer_note,
                                    created_at: event.created_at,
                                    updated_at: event.updated_at,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::User;
    use std::time::Duration;
    use tonic::Code;

    fn make_user(id: &str, is_admin: bool) -> User {
        User {
            user_id: Uuid::parse_str(id).unwrap(),
            is_admin,
        }
    }

    fn make_event(user_id: Uuid, status: &str) -> ProfileRequestEvent {
        ProfileRequestEvent {
            profile_request_id: Uuid::now_v7(),
            user_id,
            status: status.to_string(),
            reviewer_note: String::new(),
            created_at: String::new(),
            updated_at: String::new(),
        }
    }

    /// Helper: subscribe with optional auth user, return the mpsc receiver from the stream.
    async fn subscribe(
        svc: &ProfileRequestService,
        user_id: Uuid,
        auth_user: Option<User>,
    ) -> tokio::sync::mpsc::Receiver<Result<ProtoEvent, Status>> {
        let mut req = Request::new(SubscribeRequest {
            user_id: user_id.to_string(),
        });
        req.extensions_mut().insert(auth_user);
        svc.subscribe(req)
            .await
            .expect("subscribe should succeed")
            .into_inner()
            .into_inner()
    }

    /// Drain all events from the stream (runs until broadcast closes).
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
    async fn all_events_delivered_without_filter() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(tx.clone());

        let mut rx = subscribe(&svc, "", None).await;
        drop(svc);

        tx.send(make_event(Uuid::new_v4(), "confirmed")).unwrap();
        tx.send(make_event(Uuid::new_v4(), "rejected")).unwrap();
        tx.send(make_event(Uuid::new_v4(), "sent")).unwrap();
        drop(tx);

        let events = collect_events(&mut rx).await;
        assert_eq!(events.len(), 3);
    }

    #[tokio::test]
    async fn filter_by_user_id() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(tx.clone());
        let target = Uuid::new_v4();

        let mut rx = subscribe(&svc, target, None).await;
        drop(svc);

        tx.send(make_event(target, "confirmed")).unwrap();
        tx.send(make_event(Uuid::new_v4(), "rejected")).unwrap();
        tx.send(make_event(target, "sent")).unwrap();
        drop(tx);

        let events = collect_events(&mut rx).await;
        assert_eq!(events.len(), 2);
        assert!(events.iter().all(|e| e.user_id == target.to_string()));
    }

    #[tokio::test]
    async fn multiple_subscribers_all_receive_all_events() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(tx.clone());

        let mut rx1 = subscribe(&svc, "", None).await;
        let mut rx2 = subscribe(&svc, "", None).await;
        drop(svc);

        tx.send(make_event("u1", "confirmed")).unwrap();
        tx.send(make_event("u2", "rejected")).unwrap();
        drop(tx);

        let e1 = collect_events(&mut rx1).await;
        let e2 = collect_events(&mut rx2).await;
        assert_eq!(e1.len(), 2);
        assert_eq!(e2.len(), 2);
    }

    #[tokio::test]
    async fn multiple_subscribers_independent_filters() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(tx.clone());
        let alice = "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa";
        let bob = "bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb";

        let mut rx_a = subscribe(&svc, alice, None).await;
        let mut rx_b = subscribe(&svc, bob, None).await;
        drop(svc);

        tx.send(make_event(alice, "a1")).unwrap();
        tx.send(make_event(bob, "b1")).unwrap();
        tx.send(make_event(alice, "a2")).unwrap();
        drop(tx);

        let alice_events = collect_events(&mut rx_a).await;
        let bob_events = collect_events(&mut rx_b).await;

        assert_eq!(alice_events.len(), 2);
        assert!(alice_events.iter().all(|e| e.user_id == alice));
        assert_eq!(bob_events.len(), 1);
        assert_eq!(bob_events[0].user_id, bob);
    }

    #[tokio::test]
    async fn non_admin_subscribes_to_own_events_succeeds() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(tx.clone());
        let uid = "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa";
        let user = make_user(uid, false);

        let mut rx = subscribe(&svc, uid, Some(user)).await;
        drop(svc);

        tx.send(make_event(uid, "ok")).unwrap();
        drop(tx);

        let events = collect_events(&mut rx).await;
        assert_eq!(events.len(), 1);
    }

    #[tokio::test]
    async fn admin_can_subscribe_to_any_user() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(tx.clone());
        let admin = make_user("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa", true);

        let mut rx = subscribe(&svc, "bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb", Some(admin)).await;
        drop(svc);

        tx.send(make_event("bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb", "ok")).unwrap();
        drop(tx);

        let events = collect_events(&mut rx).await;
        assert_eq!(events.len(), 1);
    }

    #[tokio::test]
    async fn unauthenticated_user_can_subscribe() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(tx.clone());

        let mut rx = subscribe(&svc, "", None).await;
        drop(svc);

        tx.send(make_event("u1", "ok")).unwrap();
        drop(tx);

        let events = collect_events(&mut rx).await;
        assert_eq!(events.len(), 1);
    }

    // ─── Error cases ──────────────────────────────────────────────

    #[tokio::test]
    async fn non_admin_cannot_subscribe_to_other_user() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(tx.clone());
        let user = make_user("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa", false);

        let mut req = Request::new(SubscribeRequest {
            user_id: "bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb".to_string(),
        });
        req.extensions_mut().insert(Some(user));

        let err = svc.subscribe(req).await.unwrap_err();
        assert_eq!(err.code(), Code::PermissionDenied);
    }

    #[tokio::test]
    async fn invalid_user_id_returns_invalid_argument() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(tx.clone());

        let mut req = Request::new(SubscribeRequest {
            user_id: "not-a-uuid".to_string(),
        });
        req.extensions_mut().insert(None::<User>);

        let err = svc.subscribe(req).await.unwrap_err();
        assert_eq!(err.code(), Code::InvalidArgument);
    }

    // ─── Edge cases ───────────────────────────────────────────────

    #[tokio::test]
    async fn empty_filter_non_admin_works() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(tx.clone());
        let user = make_user("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa", false);

        // Empty filter with non-admin user — allowed (gets own events implicitly)
        let mut rx = subscribe(&svc, "", Some(user)).await;
        drop(svc);

        tx.send(make_event("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa", "ok")).unwrap();
        drop(tx);

        let events = collect_events(&mut rx).await;
        assert_eq!(events.len(), 1);
    }

    #[tokio::test]
    async fn stream_ends_when_all_senders_dropped() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(tx.clone());

        let mut rx = subscribe(&svc, "", None).await;
        drop(svc);

        tx.send(make_event("u1", "ok")).unwrap();
        drop(tx);

        // Should get the one event then None
        assert!(rx.recv().await.is_some());
        assert!(rx.recv().await.is_none());
    }

    #[tokio::test]
    async fn no_messages_yields_immediate_stream_end() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(tx.clone());

        let mut rx = subscribe(&svc, "", None).await;
        drop(svc);
        drop(tx);

        assert!(rx.recv().await.is_none());
    }

    #[tokio::test]
    async fn lagged_subscriber_logs_and_continues() {
        // Use a tiny buffer so we can force lagging
        let (tx, _) = broadcast::channel(2);
        let svc = ProfileRequestService::new(tx.clone());

        let mut rx = subscribe(&svc, "", None).await;
        drop(svc);

        // Fill the 2-slot buffer + 1 more to force the receiver to lag
        tx.send(make_event("u1", "a")).unwrap();
        tx.send(make_event("u1", "b")).unwrap();
        tx.send(make_event("u1", "c")).unwrap(); // receiver lags here

        // Give the spawned task time to process the lag and catch up
        tokio::time::sleep(Duration::from_millis(100)).await;

        tx.send(make_event("u1", "d")).unwrap();
        drop(tx);

        let events = collect_events(&mut rx).await;
        // After lagging at "c", the receiver resets and should get "d"
        assert!(!events.is_empty(), "should still receive post-lag events");
        assert_eq!(events.last().unwrap().status, "d");
    }

    #[tokio::test]
    async fn subscriber_dropped_does_not_affect_others() {
        let (tx, _) = broadcast::channel(256);
        let svc = ProfileRequestService::new(tx.clone());

        let mut rx1 = subscribe(&svc, "", None).await;
        let mut rx2 = subscribe(&svc, "", None).await;
        drop(svc);

        // Drop rx1 (the mpsc receiver) — the spawned task will get SendError and exit
        // rx2 should still work
        rx1.close();

        tx.send(make_event("u1", "ok")).unwrap();
        drop(tx);

        let events = collect_events(&mut rx2).await;
        assert_eq!(events.len(), 1);
    }
}
