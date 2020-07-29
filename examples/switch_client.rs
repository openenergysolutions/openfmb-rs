use futures::StreamExt;
use openfmb::encoding::ProtobufEncoding;
use std::env;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mrid = uuid::Uuid::parse_str(&env::var("SWITCH_MRID")?)?;
    let nc = nats::connect(&env::var("NATS_URL")?)?;
    let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
    let mut switch = openfmb::client::Switch::new(bus, mrid);
    while let (Some(Ok(position)), Some(Ok(closed)), Some(Ok(open))) = (
        switch.position().await?.next().await,
        switch.is_closed().await?.next().await,
        switch.is_open().await?.next().await,
    ) {
        println!(
            "{}: position: {:?}, is_closed: {:?}, is_open: {:?}",
            mrid, position, closed, open
        );
        match switch.toggle_position().await {
            Ok(_) => println!("Switch toggled!"),
            Err(err) => println!("Switch toggle failed, reason {:?}", err),
        }
    }
    Ok(())
}
