// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use futures::stream::StreamExt;
use log::info;
use openfmb::encoding::ProtobufEncoding;
use openfmb_messages::{capbankmodule::*, commonmodule::*};
use std::env;
use std::time::SystemTime;
use tokio::time;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let mrid = uuid::Uuid::parse_str(&env::var("CAPBANK_MRID")?)?;
    let nats_url = env::var("NATS_URL").unwrap_or("nats://127.0.0.1:4222".to_string());
    let nc = nats::asynk::connect(&nats_url).await?;
    let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
    let mut capbank = openfmb::device::CapBank::new(bus, mrid);
    info!(
        "Connected to capbank {:?} using nats at {:?}",
        mrid, nats_url
    );
    let mut controls = capbank.control().await?;
    let mut poll_interval = time::interval(time::Duration::from_secs(1));
    let mut identified_object: IdentifiedObject = Default::default();
    identified_object.description = Some(format!("OpenFMB-RS Example CapBank Device {}", mrid));
    identified_object.m_rid = Some(format!("{}", mrid));
    let message_info = MessageInfo {
        identified_object: Some(identified_object),
        message_time_stamp: None,
    };
    let mut status_dps: StatusDps = Default::default();
    status_dps.st_val = DbPosKind::Closed.into();
    let mut cap_bank_event_and_status_ypsh: CapBankEventAndStatusYpsh = Default::default();
    cap_bank_event_and_status_ypsh.pos = Some(PhaseDps {
        phs3: Some(status_dps),
        ..Default::default()
    });
    let mut cap_bank_status: CapBankStatus = Default::default();
    cap_bank_status.cap_bank_event_and_status_ypsh = Some(cap_bank_event_and_status_ypsh);
    let mut status: CapBankStatusProfile = Default::default();
    status.cap_bank_status = Some(cap_bank_status);
    status.status_message_info = Some(StatusMessageInfo {
        message_info: Some(message_info),
    });
    loop {
        tokio::select! {
            ctl = controls.next() => {
                info!("Received control");
                if let Some(Ok(ctl)) = ctl {
                    for sch_pt in ctl.cap_bank_control()
                        .cap_bank_control_fscc()
                        .control_fscc()
                        .control_schedule_fsch()
                        .val_acsg().sch_pts().iter() {
                            for sch_prmtr in sch_pt.schedule_parameter().iter() {
                                println!("Received `{:?}` value: {}", sch_prmtr.schedule_parameter_type(), sch_prmtr.value())
                            }
                        }
                }
            },
            _ = poll_interval.tick() => {
                info!("Tick, publishing status");
                let status = status.clone();
                let mut capbank = capbank.clone();
                tokio::spawn(async move {
                    let time = SystemTime::now();
                    let mut status = status.clone();
                    status.status_message_info.as_mut().unwrap().message_info.as_mut().unwrap().message_time_stamp = Some(Timestamp {
                        seconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                        nanoseconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
                        tq: None,
                    });
                    capbank.status(status).await.unwrap();
                });
            },
        }
    }
}