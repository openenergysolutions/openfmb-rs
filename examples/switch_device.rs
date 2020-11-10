use futures::stream::StreamExt;
use log::info;
use std::time::SystemTime;
use openfmb::encoding::ProtobufEncoding;
use openfmb_messages::{
    commonmodule::*,
    switchmodule::*,
};
use std::env;
use tokio::time;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let mrid = uuid::Uuid::parse_str(&env::var("SWITCH_MRID")?)?;
    let nats_url = env::var("NATS_URL")?;
    let nc = nats::asynk::connect(&env::var("NATS_URL")?).await?;
    let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
    let mut switch = openfmb::device::Switch::new(bus, mrid);
    info!(
        "Connected to switch {:?} using nats at {:?}",
        mrid, nats_url
    );
    let mut controls = switch.control().await?;
    let mut poll_interval = time::interval(time::Duration::from_secs(1));
    let mut identified_object: IdentifiedObject = Default::default();
    identified_object.description = Some(format!("OpenFMB-RS Example Switch Device {}", mrid));
    identified_object.m_rid = Some(format!("{}", mrid));
    let message_info = MessageInfo {
        identified_object: Some(identified_object),
        message_time_stamp: None,
    };
    let dynamic_test = DynamicTestKind::None;
    let ens_dynamic_test = EnsDynamicTestKind {
        st_val: dynamic_test.into(),
        ..Default::default()
    };
    let mut status_dps: StatusDps = Default::default();
    status_dps.st_val = DbPosKind::Closed.into();
    let mut switch_status_xswi: SwitchStatusXswi = Default::default();
    switch_status_xswi.dynamic_test = Some(ens_dynamic_test);
    switch_status_xswi.pos = Some(PhaseDps {
        phs3: Some(status_dps),
        ..Default::default()
    });
    let mut switch_status: SwitchStatus = Default::default();
    switch_status.switch_status_xswi = Some(switch_status_xswi);
    let mut status: SwitchStatusProfile = Default::default();
    status.switch_status = Some(switch_status);
    status.status_message_info = Some(StatusMessageInfo {
        message_info: Some(message_info),
    });
    loop {
        tokio::select! {
            ctl = controls.next() => {
                info!("Got control {:?}", ctl);
                if let Some(Ok(ctl)) = ctl {
                    if let Some(ref phs3) = ctl.switch_discrete_control()
                                           .switch_discrete_control_xswi()
                                           .pos()
                                           .phs3 {
                                               if phs3.ctl_val {
                                                   status.switch_status_mut()
                                                         .switch_status_xswi_mut()
                                                         .pos_mut()
                                                         .phs3_mut()
                                                         .st_val = 1;
                                               } else {
                                                   status.switch_status_mut()
                                                         .switch_status_xswi_mut()
                                                         .pos_mut()
                                                         .phs3_mut()
                                                         .st_val = 2;
                                               }
                                           }
                }
            },
            _ = poll_interval.tick() => {
                info!("Tick, publishing status");
                let status = status.clone();
                let mut switch = switch.clone();
                tokio::spawn(async move {
                    let time = SystemTime::now();
                    let mut status = status.clone();
                    status.status_message_info.as_mut().unwrap().message_info.as_mut().unwrap().message_time_stamp = Some(Timestamp {
                        seconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                        nanoseconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
                        tq: None,
                    });
                    switch.status(status).await.unwrap();
                });
            },
        }
    }
}
