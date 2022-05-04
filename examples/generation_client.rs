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

    let mrid = uuid::Uuid::parse_str("46f9c003-96c9-4de6-b6a1-110ffca8362a")?;
    let nats_url = env::var("NATS_URL").unwrap_or("nats://127.0.0.1:4222".to_string());
    let nc = nats::asynk::connect(&nats_url).await?;
    let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
    let mut gen = openfmb::client::Generation::new(bus, mrid);

    info!(
        "Connected to gnerator {:?} using nats at {:?}",
        mrid, nats_url
    );

    while let (Some(Ok(p)),Some(Ok(q)),) = (
        gen.p().await?.next().await, gen.q().await?.next().await,        

    ) {
        info!(
            "{}: p: {:?}, q: {:?}",
            mrid, p, q
        );
    }
    
    Ok(())
}
