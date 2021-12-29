// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use futures::StreamExt;
use log::info;
use openfmb::encoding::ProtobufEncoding;
use std::env;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let bus_type = env::var("BUS_TYPE").unwrap_or("NATS".to_string());
    let mrid = uuid::Uuid::parse_str(&env::var("SWITCH_MRID")?)?;

    if bus_type == "NATS" {
        let nats_url = env::var("NATS_URL").unwrap_or("nats://127.0.0.1:4222".to_string());
        let nc = nats::connect(&nats_url)?;
        let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
        let mut switch = openfmb::client::Switch::new(bus, mrid);

        info!(
            "Connected to switch {:?} using nats at {:?}",
            mrid, nats_url
        );

        while let (Some(Ok(position)), Some(Ok(closed)), Some(Ok(open))) = (
            switch.position().await?.next().await,
            switch.is_closed().await?.next().await,
            switch.is_open().await?.next().await,
        ) {
            info!(
                "{}: position: {:?}, is_closed: {:?}, is_open: {:?}, Toggling position...",
                mrid, position, closed, open
            );
            match switch.toggle_position().await {
                Ok(_) => info!("Switch toggled!"),
                Err(err) => info!("Switch toggle failed, reason {:?}", err),
            }
        }
    } else {
        let bus = openfmb::bus::ZenohBus::<ProtobufEncoding>::new();

        println!("Connected to switch {:?} using zenoh bus {:?}", mrid, &bus);
        let mut switch = openfmb::client::Switch::new(bus, mrid);

        while let (Some(Ok(position)), Some(Ok(closed)), Some(Ok(open))) = (
            switch.position().await?.next().await,
            switch.is_closed().await?.next().await,
            switch.is_open().await?.next().await,
        ) {
            info!(
                "{}: position: {:?}, is_closed: {:?}, is_open: {:?}, Toggling position...",
                mrid, position, closed, open
            );
            match switch.toggle_position().await {
                Ok(_) => info!("Switch toggled!"),
                Err(err) => info!("Switch toggle failed, reason {:?}", err),
            }
        }
    }

    Ok(())
}
