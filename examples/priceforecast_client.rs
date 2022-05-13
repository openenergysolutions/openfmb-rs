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

    let mrid = uuid::Uuid::parse_str("aa6238a7-009e-42fa-ab9e-9ce6fe0e101b")?;
    let nats_url = env::var("NATS_URL").unwrap_or("nats://127.0.0.1:4222".to_string());
    let nc = nats::asynk::connect(&nats_url).await?;
    let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
    let mut pforecast = openfmb::client::PriceForecast::new(bus, mrid);

    info!(
        "Connected priceforecast {:?} using nats at {:?}",
        mrid, nats_url
    );

    while let (Some(Ok(pfp)),) = (
        pforecast.forecastpoints().await?.next().await,        
    ) {
        info!(
            "{}: price_forecast_points: {:?}",
            mrid, pfp
        );
    }
    
    Ok(())
}
