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
    let mrid = uuid::Uuid::parse_str(&env::var("CAPBANK_MRID")?)?;
    let nats_url = env::var("NATS_URL").unwrap_or("nats://127.0.0.1:4222".to_string());
    let nc = nats::asynk::connect(&nats_url).await?;
    let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
    let mut cap_bank = openfmb::client::CapBank::new(bus, mrid);

    info!(
        "Connected to capbank {:?} using nats at {:?}",
        mrid, nats_url
    );

    while let (Some(Ok(_))) = (cap_bank.status().await?.next().await) {
        info!("Received status, setting VArNetMag and WNetMag schedules");
        if let Err(e) = cap_bank.set_VArNetMag_schedule(330_f64).await {
            info!("`set_VArNetMag_schedule()` failed: {:?}", e);
        }
        if let Err(e) = cap_bank.set_WNetMag_schedule(165_f64).await {
            info!("`set_WNetMag_schedule()` failed: {:?}", e);
        }
    }
    Ok(())
}
