use std::{convert::TryInto, env, sync::Arc};

use bytes::Bytes;
use futures::stream::StreamExt;
use openfmb_messages::{
    breakermodule::{Breaker, BreakerStatus, BreakerStatusProfile},
    commonmodule::{
        ConductingEquipment, IdentifiedObject, MessageInfo, PhaseDps, StatusAndEventXcbr,
        StatusDps, StatusMessageInfo,
    },
};
use openfmb_messages_ext::OpenFMBMessage;
use prost::Message;

#[tokio::test]
async fn test_nats_async() {
    let nats_url = env::var("NATS_URL").unwrap_or("nats://127.0.0.1:4222".to_string());
    let client = async_nats::connect(&nats_url).await.unwrap();

    let mut subscriber = client.subscribe("openfmb.>".into()).await.unwrap();

    for n in 0..10 {
        let profile = BreakerStatusProfile {
            status_message_info: Some(StatusMessageInfo {
                message_info: Some(MessageInfo {
                    identified_object: Some(IdentifiedObject {
                        m_rid: Some(uuid::Uuid::new_v4().hyphenated().to_string()),
                        ..Default::default()
                    }),
                    message_time_stamp: None,
                }),
            }),
            breaker: Some(Breaker {
                conducting_equipment: Some(ConductingEquipment {
                    m_rid: "ec9dc3dd-d329-4ec6-9149-762b033f0be8".into(),
                    named_object: None,
                }),
            }),
            breaker_status: Some(BreakerStatus {
                status_and_event_xcbr: Some(StatusAndEventXcbr {
                    pos: Some(PhaseDps {
                        phs3: Some(StatusDps {
                            st_val: n % 2,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                status_value: None,
            }),
        };
        let mut buffer = vec![];
        profile.encode(&mut buffer).unwrap();
        client
            .publish(
                "openfmb.breakermodule.BreakerStatusProfile.ec9dc3dd-d329-4ec6-9149-762b033f0be8"
                    .into(),
                Bytes::copy_from_slice(&buffer),
            )
            .await
            .unwrap()
    }
    client.flush().await.unwrap();

    while let Ok(message) =
        tokio::time::timeout(tokio::time::Duration::from_millis(500), subscriber.next()).await
    {
        match message {
            Some(message) => {
                let message = Arc::new(message);
                let profile: Result<OpenFMBMessage, _> = message.as_ref().try_into();
                assert!(profile.is_ok());
            }
            None => assert!(false),
        }
    }
}
