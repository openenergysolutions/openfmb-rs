use futures::stream::StreamExt;
use log::info;
use openfmb::encoding::ProtobufEncoding;
use openfmb_messages::{
    commonmodule::{
        DbPosKind, DynamicTestKind, EnsDynamicTestKind, IdentifiedObject, Ied, StatusDps,
    },
    switchmodule::{SwitchStatus, SwitchStatusProfile, SwitchStatusXswi},
};
use std::env;
use tokio::time;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let mrid = uuid::Uuid::parse_str(&env::var("SWITCH_MRID")?)?;
    let nats_url = env::var("NATS_URL")?;
    let nc = nats::connect(&env::var("NATS_URL")?)?;
    let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
    let mut switch = openfmb::device::Switch::new(bus, mrid);
    info!(
        "Connected to switch {:?} using nats at {:?}",
        mrid, nats_url
    );
    let mut controls = switch.control()?;
    let mut poll_interval = time::interval(time::Duration::from_secs(1));
    let mut identified_object: IdentifiedObject = Default::default();
    identified_object.description = Some(format!("OpenFMB-RS Example Switch Device {}", mrid));
    identified_object.m_rid = Some(format!("{}", mrid));
    let mut ied: Ied = Default::default();
    ied.identified_object = Some(identified_object);
    let dynamic_test = DynamicTestKind::None;
    let ens_dynamic_test = EnsDynamicTestKind {
        st_val: dynamic_test.into(),
        ..Default::default()
    };
    let mut status_dps: StatusDps = Default::default();
    status_dps.st_val = DbPosKind::Closed.into();
    let mut switch_status_xswi: SwitchStatusXswi = Default::default();
    switch_status_xswi.dynamic_test = Some(ens_dynamic_test);
    switch_status_xswi.pos = Some(status_dps);
    let mut switch_status: SwitchStatus = Default::default();
    switch_status.switch_status_xswi = Some(switch_status_xswi);
    let mut status: SwitchStatusProfile = Default::default();
    status.ied = Some(ied);
    status.switch_status = Some(switch_status);
    loop {
        tokio::select! {
            ctl = controls.next() => {
                info!("Got control {:?}", ctl);
            },
            _ = poll_interval.tick() => {
                info!("Tick, publishing status");
                let status = status.clone();
                let mut switch = switch.clone();
                tokio::spawn(async move {
                    switch.status(status.clone()).await.unwrap();
                });
            },
        }
    }
}
