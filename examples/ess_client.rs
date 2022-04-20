// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use futures::StreamExt;
use log::info;
use openfmb::encoding::ProtobufEncoding;
use std::env;
use tokio::time;


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let mrid = uuid::Uuid::parse_str("4f659ee9-5ce8-4ef2-bcb5-0f38f2d31868")?;
    let nats_url = env::var("NATS_URL").unwrap_or("nats://127.0.0.1:4222".to_string());
    let nc = nats::asynk::connect(&nats_url).await?;
    let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
    let mut ess = openfmb::client::Ess::new(bus, mrid);

    info!(
        "Connected to ess {:?} using nats at {:?}",
        mrid, nats_url
    );

    while let (Some(Ok(mag)),) = (
        ess.soc_mag().await?.next().await,        

    ) {
        info!(
            "{}: soc_mag: {:?}",
            mrid, mag
        );
    }
    
    Ok(())
}
