// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use futures::StreamExt;
use log::info;
use openfmb::{
    encoding::ProtobufEncoding,
    prelude::Publisher,
    topic::{self, ProfileTopic},
};
use openfmb_messages::{switchmodule::SwitchDiscreteControlProfile, Module, Profile};
use openfmb_messages_ext::SwitchControlExt;
use std::{env, str::FromStr};

fn topic(profile: topic::Profile, mrid: &uuid::Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::SwitchModule, profile, mrid.clone())
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let mrid = uuid::Uuid::from_str(
        &env::var("SWITCH_MRID").unwrap_or("ec9dc3dd-d329-4ec6-9149-762b033f0be8".to_string()),
    )?;

    let nats_url = env::var("NATS_URL").unwrap_or("nats://192.168.86.30:4222".to_string());
    let nc = async_nats::connect(&nats_url).await?;
    let mut bus = openfmb::bus::NatsAsyncBus::<ProtobufEncoding>::new(nc);
    let mut switch = openfmb::client::Switch::new(bus.clone(), mrid);

    info!(
        "Connected to switch {:?} using nats at {:?}",
        mrid, nats_url
    );

    while let Some(Ok(closed)) = switch.is_closed().await?.next().await {
        info!("{}: is_closed: {:?}, Toggling position...", mrid, closed);

        let msg = match closed {
            true => SwitchDiscreteControlProfile::switch_open_msg(&mrid.to_string()),
            false => SwitchDiscreteControlProfile::switch_close_msg(&mrid.to_string()),
        };

        match bus
            .publish(
                topic(Profile::SwitchDiscreteControlProfile, &mrid).iter(),
                msg,
            )
            .await
        {
            Ok(_) => info!("Switch toggled!"),
            Err(err) => info!("Switch toggle failed, reason {:?}", err),
        }
    }

    Ok(())
}
