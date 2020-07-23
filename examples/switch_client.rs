use futures::StreamExt;
use openfmb::encoding::ProtobufEncoding;
use std::env;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mrid = uuid::Uuid::parse_str(&env::var("SWITCH_MRID")?)?;
    let nc = nats::connect(&env::var("NATS_URL")?)?;
    let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
    let mut switch = openfmb::client::Switch::new(bus, mrid);

    let mut status = switch.status()?;
    while let Some(Ok(status)) = status.next().await {
        println!("{:?}", status);
    }
    Ok(())
}
