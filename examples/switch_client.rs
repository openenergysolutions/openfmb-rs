use futures::StreamExt;
use openfmb::encoding::ProtobufEncoding;
use openfmb::prelude::*;
use std::env;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mrid = uuid::Uuid::parse_str(&env::var("SWITCH_MRID")?)?;
    let nc = nats::connect(&env::var("NATS_URL")?)?;
    let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
    let mut switch = openfmb::client::Switch::new(bus, mrid);

    let mut status_stream = switch.status_stream()?;
    while let Some(Ok(status)) = status_stream.next().await {
        println!("{:?}", status);
    }
    Ok(())
}
