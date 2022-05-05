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

    let mrid = uuid::Uuid::parse_str("51a949c6-ff44-4128-b74b-ae1f11d7453e")?;
    let nats_url = env::var("NATS_URL").unwrap_or("nats://127.0.0.1:4222".to_string());
    let nc = nats::asynk::connect(&nats_url).await?;
    let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
    let mut sforecast = openfmb::client::SolarForecast::new(bus, mrid);

    info!(
        "Connected to solarforecast {:?} using nats at {:?}",
        mrid, nats_url
    );

    while let (Some(Ok(sfp)),) = (
        sforecast.solarforecastpoints().await?.next().await,        
    ) {
        info!(
            "{}: solar_forecast_points: {:?}",
            mrid, sfp
        );
    }
    
    Ok(())
}
