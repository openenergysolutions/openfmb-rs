//use futures::stream::StreamExt;
use log::info;
use openfmb::encoding::ProtobufEncoding;
use openfmb_messages::{commonmodule::*, essmodule::*};

use std::env;
use std::time::SystemTime;
use tokio::time;
//use rand::Rng;

/*  TODO: Pull the values to send for this from a configuration file?  This is just testing data we hope to receive from live devices
        bus_id: doc.get_first_node("//Cluster/Array/Name[text()='PV_BUS_ID']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
        power_rating: doc.get_first_node("//Cluster/Array/Name[text()='PV_PowerRating_kVA']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),

*/

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
  pretty_env_logger::init();

  let mrid = uuid::Uuid::parse_str("46f9c003-96c9-4de6-b6a1-110ffca8362a")?;
  let nats_url = env::var("NATS_URL").unwrap_or("nats://127.0.0.1:4222".to_string());
  let nc = nats::asynk::connect(&nats_url).await?;
  let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
  let ess = openfmb::device::Solar::new(bus, mrid);
  info!(
    "Connected to ess {:?} using nats at {:?}",
    mrid, nats_url
  );

  let mut poll_interval = time::interval(time::Duration::from_secs(1));
  let mut identified_object: IdentifiedObject = Default::default();
  identified_object.description = Some(format!("OpenFMB ESS Device {}", mrid));
  identified_object.m_rid = Some(format!("{}", mrid));
  let message_info = MessageInfo {
    identified_object: Some(identified_object),
    message_time_stamp: None,
  };


  // let mut mv: Mv = Default::default();
  // let mut ess_status_z: EssStatusZbat = Default::default();

  // mv.mag = 50.0;
  // ess_status_z.soc = Some(mv);
  
  // info!(
  //   "Set soc f val {:?}", ess_status_z.clone().soc.clone().unwrap()
  // );




  // let mut ess_status: EssStatus = Default::default();
  // ess_status.ess_status_zbat = Some(ess_status_z);
  
  // let mut status: EssStatusProfile = Default::default();
  // status.ess_status = Some(ess_status);
  // status.status_message_info = Some(StatusMessageInfo {
  //   message_info: Some(message_info)
  // });

  // loop {
  //   tokio::select! {
  //     _ = poll_interval.tick() => {
  //       info!("Tick, publishing status");
  //       let status = status.clone();
  //       let mut ess = ess.clone();
  //       tokio::spawn(async move {
  //           let time = SystemTime::now();
  //           let mut status = status.clone();
  //           status.status_message_info.as_mut().unwrap().message_info.as_mut().unwrap().message_time_stamp = Some(Timestamp {
  //               seconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
  //               nanoseconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
  //               tq: None,
  //           });
  //           ess.status(status).await.unwrap();
  //       });
  //     },
  //   } // select
  // } // loop dloop

}